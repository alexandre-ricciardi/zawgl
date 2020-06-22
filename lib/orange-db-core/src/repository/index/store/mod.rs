mod records;

use self::records::*;
use super::super::super::config::*;
use super::model::*;
use super::super::records::*;

pub struct BTreeNodeStore {
    records_manager: RecordsManager,
}

fn append_key(vkey: &mut Vec<u8>, key_buf: &[u8]) {
    let mut it = key_buf.iter();
    let str_end = it.position(|&c| c == b'\0').unwrap_or(key_buf.len());
    vkey.extend_from_slice(&key_buf[0..str_end]);
}

fn append_list_ptr(ptrs: &mut Vec<u64>, buf: &[u8]) {
    let mut counter_buf = [0u8; 2];
    counter_buf.copy_from_slice(&buf[..2]);
    let counter = u16::from_be_bytes(counter_buf);
    for ptr_index in 0..counter {
        let offset = 2 + ptr_index as usize * 8 ;
        let mut ptr_buf = [0u8; 8];
        ptr_buf.copy_from_slice(&buf[offset..offset+8]);
        ptrs.push(u64::from_be_bytes(ptr_buf));
    }
}

enum CellLoadRes {
    InteriorCell(NodeId),
    LeafCell(Vec<NodeId>),
}

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        BTreeNodeStore{records_manager: RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD)}
    }

    fn retrieve_overflow_cells(&mut self, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> Option<CellLoadRes> {
        let overflow_cell_records = self.load_overflow_cell_records(cell_record)?;
        let mut is_leaf_cell = false;
        let mut ptrs = Vec::new();
        for overflow_cell in &overflow_cell_records {
            if overflow_cell.is_list_ptr() {
                is_leaf_cell = true;
                append_list_ptr(&mut ptrs, &overflow_cell.key);
            } else {
                append_key(vkey, &overflow_cell.key);
            }
        }
        if is_leaf_cell {
            Some(CellLoadRes::LeafCell(ptrs))
        } else {
            overflow_cell_records.last().map(|cell|{
                CellLoadRes::InteriorCell(cell.node_ptr)
            })
            .or_else(|| {
                Some(CellLoadRes::InteriorCell(cell_record.node_ptr))
            })
        }
    }

    fn retrieve_cell(&mut self, cell_record: &CellRecord) -> Option<Cell> {
        let mut vkey = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let cell_load_res = self.retrieve_overflow_cells(&cell_record, &mut vkey);
        cell_load_res.map(|res| {
            match res {
                CellLoadRes::InteriorCell(id) => {
                    Cell::new(&String::from_utf8(vkey).unwrap(), Some(id), Vec::new(), cell_record.is_active())
                },
                CellLoadRes::LeafCell(ptrs) => {
                    Cell::new(&String::from_utf8(vkey).unwrap(), None, ptrs, cell_record.is_active())
                }
            }
        })
    }

    pub fn retrieve_node(&mut self, nid: NodeId) -> Option<BTreeNode> {
        let node = self.load_node_record(nid)?;
        let mut cells = Vec::new();
        for cell_record_id in 0..node.cells.len() {
            let cell_record = &node.cells[cell_record_id];
            if cell_record.is_active() {
                cells.push(self.retrieve_cell(cell_record)?);
            }
        }
        let next_node_ptr = {
            if node.has_next_node() {
                Some(node.ptr)
            } else {
                None
            }
        };
        Some(BTreeNode::new_with_id(Some(nid), next_node_ptr, node.is_leaf(), node.is_root(), cells))
    }

    fn update_overflow_cells(&mut self, reverse_cell_records: &mut [CellRecord]) -> Option<()> {
        for cell in reverse_cell_records {

        }
        Some(())
    }

    fn load_or_create_free_cells_overflow_node(&mut self) -> Option<(NodeId, BNodeRecord)> {
        if self.records_manager.is_empty() {
            let mut first_node = BNodeRecord::new();
            first_node.set_overflow_node();
            self.create_node_record(&first_node)?;
            Some((0, first_node))
        } else {
            let mut first_node = self.load_first_node_record()?;
            if first_node.is_overflow_node() {
                if first_node.contains_free_cells() {
                    Some((0, first_node))
                } else {
                    Some((first_node.next_free_cells_node_ptr, self.load_node_record(first_node.next_free_cells_node_ptr)?))
                }
            } else {
                if first_node.next_free_cells_node_ptr == 0 {
                    let mut next_free_cells_overflow_node = BNodeRecord::new();
                    next_free_cells_overflow_node.set_overflow_node();
                    let id = self.create_node_record(&next_free_cells_overflow_node)?;
                    first_node.next_free_cells_node_ptr = id;
                    self.save_first_node_record(&first_node)?;
                    Some((id, next_free_cells_overflow_node))
                } else {
                    Some((first_node.next_free_cells_node_ptr, self.load_node_record(first_node.next_free_cells_node_ptr)?))
                }
            }
            
        }
    }

    fn create_overflow_cells(&mut self, reverse_cell_records: &mut [CellRecord]) -> Option<(u32, u64)> {

        let mut free_cells_node_record = self.load_or_create_free_cells_overflow_node()?;
        let mut reverse_cell_id = 0;
        let mut curr_cell_id: usize = 0;
        let mut prev_cell_ptr = 0;
        let mut prev_node_ptr = free_cells_node_record.0;

        let mut is_not_ending_cell = false;
        //loop to store all overflow cells
        loop {
            let free_cell = free_cells_node_record.1.cells[curr_cell_id];
            if !free_cell.is_active() {
                let mut cell = &mut reverse_cell_records[reverse_cell_id];
                //ending cell stores the node pointer for interior nodes
                if is_not_ending_cell {
                    cell.overflow_cell_ptr = prev_cell_ptr;
                    cell.node_ptr = prev_node_ptr;
                } else {
                    is_not_ending_cell = true;
                }
                free_cells_node_record.1.cells[curr_cell_id] = *cell;
                
                reverse_cell_id += 1;
                if reverse_cell_id >= reverse_cell_records.len() {
                    self.save_node_record(free_cells_node_record.0, &free_cells_node_record.1)?;
                    break;
                }
            }
            prev_cell_ptr = curr_cell_id as u32;
            curr_cell_id += 1;
            if curr_cell_id > NB_CELL {
                self.save_node_record(free_cells_node_record.0, &free_cells_node_record.1)?;
                prev_node_ptr = free_cells_node_record.0;
                free_cells_node_record = self.load_or_create_free_cells_overflow_node()?;
                curr_cell_id = 0;

            }
        }
        Some((curr_cell_id as u32, prev_node_ptr))
    }

    fn create_cell(&mut self, cell: &Cell) -> Option<Vec<CellRecord>> {
        let mut cell_records = Vec::new();
        let key_vec = cell.get_key().clone().into_bytes();
        
        let mut offset = 0;
        while offset < key_vec.len() {
            let mut cell_record = CellRecord::new();
            cell_record.set_is_active();
            if offset + KEY_SIZE > key_vec.len() {
                let len = key_vec.len() - offset;
                cell_record.key[..len].copy_from_slice(&key_vec[offset..key_vec.len()]);
                cell_record.key[len] = b'\0';
            } else {
                cell_record.key.copy_from_slice(&key_vec[offset..offset+KEY_SIZE]);
            }
            offset += KEY_SIZE;

            cell_records.push(cell_record);
        }
        
        if cell.get_data_ptrs_ref().len() > 0 {
            let mut data_ptr_offset = 2;
            let mut cell_record = CellRecord::new();
            let mut data_ptr_count: u16 = 0;
            for data_ptr in cell.get_data_ptrs_ref() {
                cell_record.key[data_ptr_offset..data_ptr_offset+8].copy_from_slice(&data_ptr.to_be_bytes());
                data_ptr_offset += NODE_PTR_SIZE;
                data_ptr_count += 1;
                if data_ptr_offset > KEY_SIZE {
                    cell_record.key.copy_from_slice(&data_ptr_count.to_be_bytes());
                    cell_records.push(cell_record);
                    cell_record = CellRecord::new();
                    data_ptr_offset = 2;
                    data_ptr_count = 0;
                }
            }
            
        }

        let nb_records = cell_records.len();
        if nb_records > 1 {
            cell_records.reverse();
            for index in 1..nb_records {
                cell_records[index].set_has_overflow();
            }
            let last_cell_record = cell_records.first_mut()?;
            last_cell_record.node_ptr = cell.get_node_ptr()?;

            let ptrs = self.create_overflow_cells(&mut cell_records[..nb_records-1])?;
            cell_records.reverse();
            let main_cell_record = cell_records.first_mut()?;
            main_cell_record.node_ptr = ptrs.1;
            main_cell_record.overflow_cell_ptr = ptrs.0;

        }

        if let (Some(last_cell_record), Some(node_ptr)) = (cell_records.last_mut(), cell.get_node_ptr()) {
            last_cell_record.node_ptr = node_ptr;
        }

        Some(cell_records)
    }

    fn load_first_node_record(&mut self) -> Option<BNodeRecord> {
        self.load_node_record(0)
    }

    fn save_first_node_record(&mut self, node_record: &BNodeRecord) -> Option<()> {
        self.save_node_record(0, node_record)
    }

    fn load_node_record(&mut self, id: NodeId) -> Option<BNodeRecord> {
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        self.records_manager.load(id, &mut data).ok()?;
        Some(BNodeRecord::from_bytes(data))
    }

    fn create_node_record(&mut self, node_record: &BNodeRecord) -> Option<NodeId> {
        self.records_manager.create(&node_record.to_bytes()).ok()
    }

    fn save_node_record(&mut self, id: NodeId, node_record: &BNodeRecord) -> Option<()> {
        self.records_manager.save(id, &node_record.to_bytes()).ok()
    }

    pub fn create(&mut self, node: &mut BTreeNode) -> Option<()> {
        let mut node_record = BNodeRecord::new();
        if node.is_leaf() {
            node_record.set_leaf();
        }
        if let Some(next_id) = node.get_node_ptr() {
            node_record.set_has_next_node();
            node_record.ptr = next_id;
        }
        
        let mut cell_id = 0;
        for cell in node.get_cells_ref() {
            let cell_records = self.create_cell(cell)?;
            node_record.cells[cell_id] = *cell_records.first()?;
            cell_id += 1;
        }
        let id = self.create_node_record(&node_record)?;
        node.set_id(id);

        Some(())
    }

    fn load_overflow_cell_records(&mut self, root_cell_record: &CellRecord) -> Option<Vec<CellRecord>> {
        let mut cells = Vec::new();
        let mut curr_node_id = root_cell_record.node_ptr;
        let mut curr_overflow_cell_id = root_cell_record.overflow_cell_ptr;
        let mut has_overflow = root_cell_record.has_overflow();
        if has_overflow {
            let mut curr_node = self.load_node_record(curr_node_id)?;
            while has_overflow {
                let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
                has_overflow = overflow_cell.has_overflow();
                curr_node_id = overflow_cell.node_ptr;
                curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
                cells.push(*overflow_cell);
                if curr_node_id != overflow_cell.node_ptr {
                    curr_node = self.load_node_record(curr_node_id)?;
                }
            }
        }
        
        Some(cells)
    }

    fn update_cell_data_ptrs(&mut self, root_cell_record: &CellRecord, root_node_record: &BNodeRecord, data_ptrs: &Vec<NodeId>) -> Option<()> {
        let overflow_cell_records = self.load_overflow_cell_records(root_cell_record)?;
        let mut list_ptr_cells = Vec::new();
        let mut prev_cell_record = CellRecord::new();
        for cell_record in &overflow_cell_records {
            if cell_record.is_list_ptr() {
                list_ptr_cells.push(*cell_record);
            } else {
                prev_cell_record = *cell_record;
            }
        }
        
        list_ptr_cells.reverse();

        let mut cells_to_create = Vec::new();
        let mut cells_to_update = Vec::new();
        let mut data_ptr_offset = 2;
        let mut data_ptr_count: u16 = 0;
        for data_ptr in data_ptrs {
            let to_create;
            let mut list_ptr_cell = {
                if let Some(cell) = list_ptr_cells.pop() {
                    to_create = false;
                    cell
                } else {
                    to_create = true;
                    let mut new_cell = CellRecord::new();
                    new_cell.set_is_active();
                    new_cell.set_is_list_ptr();
                    new_cell
                }
            };
            list_ptr_cell.key[data_ptr_offset..data_ptr_offset+8].copy_from_slice(&data_ptr.to_be_bytes());
            data_ptr_offset += NODE_PTR_SIZE;
            data_ptr_count += 1;
            if data_ptr_offset > KEY_SIZE {
                list_ptr_cell.key.copy_from_slice(&data_ptr_count.to_be_bytes());
                if to_create {
                    cells_to_create.push(list_ptr_cell);
                } else {
                    cells_to_update.push(list_ptr_cell);
                }
                data_ptr_offset = 2;
                data_ptr_count = 0;
            }
        }
        self.update_overflow_cells(&mut cells_to_update)?;
        self.create_overflow_cells(&mut cells_to_create)?;

        //disable unused cells
        if list_ptr_cells.len() > 0 {
            for cell in &mut list_ptr_cells {
                cell.set_inactive();
            }
            self.update_overflow_cells(&mut list_ptr_cells)?;
        }
        
        Some(())
    }

    fn append_node_record_to_free_list(&mut self, node_record_id: NodeId, node_record: &mut BNodeRecord) -> Option<()> {
        let mut first_node = self.load_first_node_record()?;
        node_record.next_free_cells_node_ptr = first_node.next_free_cells_node_ptr;
        first_node.next_free_cells_node_ptr = node_record_id;
        self.save_first_node_record(&first_node)?;
        Some(())
    }

    fn delete_cell_records(&mut self, root_cell_record: &mut CellRecord, root_node_record_id: NodeId) -> Option<()> {
        root_cell_record.set_inactive();
        let mut overflow_cell_records = self.load_overflow_cell_records(root_cell_record)?;
        let mut curr_node_id = root_node_record_id;
        let mut curr_cell_id = root_cell_record.overflow_cell_ptr;
        for overflow_cell in &mut overflow_cell_records {
            let mut current_node_record = self.load_node_record(curr_node_id)?;
            let current_overflow_cell = &mut current_node_record.cells[curr_cell_id as usize];
            current_overflow_cell.set_inactive();
            if current_node_record.is_full() {
                self.append_node_record_to_free_list(curr_node_id, &mut current_node_record)?;
            }
            self.save_node_record(curr_node_id, &current_node_record)?;
            curr_cell_id = overflow_cell.overflow_cell_ptr;
            curr_node_id = overflow_cell.node_ptr;
        }
        Some(())
    }

    pub fn save(&mut self, node: &mut BTreeNode) -> Option<()> {
        let id = node.get_id()?;
        if node.get_node_changes_state().is_new_instance() {

        } else {
            let mut main_node_record = self.load_node_record(id)?;
            let mut cell_records_to_store = Vec::new();
            let mut existing_record_cell_id = 0;
            for cell_id in 0..node.get_cells_ref().len() {
                let current_cell = node.get_cell_ref(cell_id);
                if current_cell.get_change_state().is_added() {
                    let cell_records = self.create_cell(current_cell)?;
                    cell_records_to_store.push(cell_records[0]);
                } else {
                    cell_records_to_store.push(main_node_record.cells[existing_record_cell_id]);
                    existing_record_cell_id += 1;
                }
            }

            //store new cell values and delete unused cells
            for cell_id in 0..main_node_record.cells.len() {
                if cell_id < cell_records_to_store.len() {
                    main_node_record.cells[cell_id] = cell_records_to_store[cell_id];
                } else if main_node_record.cells[cell_id].is_active() {
                    self.delete_cell_records(&mut main_node_record.cells[cell_id], id);
                }
            }
            self.save_node_record(id, &main_node_record)?;
        }
        Some(())
    }

    fn get_root_node_ptr(&mut self) -> NodeId {
        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&self.records_manager.get_header_page_wrapper().get_header_payload_slice_ref()[..NODE_PTR_SIZE]);
        u64::from_be_bytes(buf)
    }

    pub fn load_root_node(&mut self) -> Option<BTreeNode> {
        let root_node_id = self.get_root_node_ptr();
        self.retrieve_node(root_node_id)
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_manager.is_empty()
    }

    pub fn sync(&mut self) {
        self.records_manager.sync();
    }
}

#[cfg(test)]
mod test_btree_node_store {
    use super::*;
    #[test]
    fn test_create() {
        let file = "C:\\Temp\\test_btree_node_store_create.db";
        let long_key = "blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6";
        std::fs::remove_file(file);
        let mut store = BTreeNodeStore::new(file);
        let mut cells = Vec::new();
        cells.push(Cell::new_ptr("blabla1", Some(1)));
        cells.push(Cell::new_ptr("blabla2", Some(2)));
        cells.push(Cell::new_ptr("blabla3", Some(3)));
        cells.push(Cell::new_ptr("blabla4", Some(4)));
        cells.push(Cell::new_ptr("blabla5", Some(5)));
        cells.push(Cell::new_ptr(long_key, Some(6)));
        let mut node = BTreeNode::new(false, false, cells);
        node.set_node_ptr(Some(42));
        store.create(&mut node);
        store.sync();

        let mut load_store = BTreeNodeStore::new(file);
        let load =  node.get_id().and_then(|id| load_store.retrieve_node(id));

        if let Some(loaded) = &load {
            assert_eq!(loaded.get_node_ptr(), Some(42));
            let cell = loaded.get_cell_ref(3);
            assert_eq!(cell.get_key(), &String::from("blabla4"));
            assert_eq!(cell.get_node_ptr(), Some(4));

            let long_key_cell = loaded.get_cell_ref(5);
            assert_eq!(long_key_cell.get_key(), &String::from(long_key));
            assert_eq!(long_key_cell.get_node_ptr(), Some(6));

        } else {
            assert!(false);
        }

    }
}