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

    fn load_overflow_cells(&mut self, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> Option<CellLoadRes> {
        let mut curr_node_id = cell_record.node_ptr;
        let mut curr_overflow_cell_id = cell_record.overflow_cell_ptr;
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        let mut has_overflow = cell_record.has_overflow();
        let mut load_node = true;
        let mut curr_node = BNodeRecord::new();
        let mut ptrs = Vec::new();
        let mut is_leaf_cell = false;
        while has_overflow {
            if load_node {
                self.records_manager.load(curr_node_id, &mut data).ok()?;
                curr_node = BNodeRecord::from_bytes(data);
            }
            let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
            has_overflow = overflow_cell.has_overflow();
            curr_node_id = overflow_cell.node_ptr;
            curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
            load_node = curr_node_id != overflow_cell.node_ptr;
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
            Some(CellLoadRes::InteriorCell(curr_node_id))
        }
    }

    fn retrieve_cell(&mut self, cell_record: &CellRecord) -> Option<Cell> {
        let mut vkey = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let node_id = self.load_overflow_cells(&cell_record, &mut vkey);
        node_id.map(|res| {
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
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        self.records_manager.load(nid, &mut data).ok()?;
        let node = BNodeRecord::from_bytes(data);
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
        Some(BTreeNode::new_with_id(Some(nid), next_node_ptr, node.is_leaf(), cells))
    }

    fn create_overflow_cells(&mut self, reverse_cell_records: &[CellRecord]) -> Option<()> {
        let mut root_node_record = self.load_root_node_record()?;
        let mut curr_free_cells_node_record = self.load_node_record(root_node_record.next_free_cells_node_ptr)?;
        let mut reverse_cell_id = 0;
        let mut curr_cell_id: usize = 0;
        let mut curr_node_ptr: u64 = 0;
        let mut prev_cell_ptr: u32 = 0;

        loop {
            let free_cell = curr_free_cells_node_record.cells[curr_cell_id];
            if !free_cell.is_active() {
                let cell = &reverse_cell_records[reverse_cell_id];
                cell.overflow_cell_ptr = prev_cell_ptr;
                cell.node_ptr = curr_node_ptr;
                curr_free_cells_node_record.cells[curr_cell_id] = *cell;
                
                
                reverse_cell_id += 1;
                if reverse_cell_id > reverse_cell_records.len() {
                    break;
                }
            }
            curr_cell_id += 1;
            if curr_cell_id > NB_CELL {
                if curr_free_cells_node_record.has_next_node() {
                    curr_node_ptr = curr_free_cells_node_record.next_free_cells_node_ptr;
                    curr_free_cells_node_record = self.load_node_record(curr_node_ptr)?;
                } else {
                    curr_free_cells_node_record = BNodeRecord::new();
                    let new_record_id = self.records_manager.create(&curr_free_cells_node_record.to_bytes()).ok()?;
                    root_node_record.next_free_cells_node_ptr = new_record_id;
                }
            }
        }

        let mut found_next_free_cells_node = false;
        for cell_id in curr_cell_id..NB_CELL {
            if !curr_free_cells_node_record.cells[cell_id].is_active() {
                found_next_free_cells_node = true;
                root_node_record.next_free_cells_node_ptr = curr_node_ptr;
                break;
            }
        }

        if !found_next_free_cells_node {
            root_node_record.next_free_cells_node_ptr = curr_free_cells_node_record.next_free_cells_node_ptr;
        }
        Some(())
    }

    fn create_cell(&mut self, node_record: &BNodeRecord, cell: &Cell) -> Option<()> {
        let cell_records = Vec::new();
        let key_vec = cell.get_key().into_bytes();
        
        let mut offset = 0;
        while offset < key_vec.len() {
            let cell_record = CellRecord::new();
            cell_record.set_is_active();
            if offset + KEY_SIZE > key_vec.len() {
                cell_record.key.copy_from_slice(&key_vec[offset..key_vec.len()]);
                cell_record.key[key_vec.len()] = b'\0';
            } else if offset + KEY_SIZE == key_vec.len() {
                cell_record.key.copy_from_slice(&key_vec[offset..offset+KEY_SIZE]);
            } else {
                cell_record.key.copy_from_slice(&key_vec[offset..offset+KEY_SIZE]);
            }
            offset += KEY_SIZE;

            cell_records.push(cell_record);
        }
        
        if node_record.is_leaf() {
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

        if cell_records.len() > 1 {
            cell_records.reverse();
            for index in 1..cell_records.len() {
                cell_records[index].set_has_overflow();
            }
            self.create_overflow_cells(&cell_records[..cell_records.len()-1])?;
        }
        Some(())
    }

    fn load_root_node_record(&mut self) -> Option<BNodeRecord> {
        self.load_node_record(0)
    }

    fn load_node_record(&mut self, id: NodeId) -> Option<BNodeRecord> {
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        self.records_manager.load(id, &mut data).ok()?;
        Some(BNodeRecord::from_bytes(data))
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
        let is_not_full = node.get_cells_ref().len() < NB_CELL;
        if is_not_full {
            let root_node_record = self.load_root_node_record()?;
            node_record.next_free_cells_node_ptr = root_node_record.next_free_cells_node_ptr;
            // need create node id first_node.next_free_cells_node_ptr = 0;
        }
        for cell in node.get_cells_ref() {
            self.create_cell(&node_record, cell)?;
        }
        Some(())
    }

    pub fn save(&mut self, node: &mut BTreeNode) -> Option<()> {

        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        Some(self.records_manager.create(&data).ok()?)
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_manager.is_empty()
    }
}