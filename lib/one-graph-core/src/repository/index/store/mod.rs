mod records;
mod pool;

use log::*;
use std::rc::Rc;
use std::cell::RefCell;
use self::records::*;
use super::super::super::buf_config::*;
use super::model::*;
use super::super::records::*;
use self::pool::NodeRecordPool;

type CellPos = (NodeId, CellId);

pub struct BTreeNodeStore {
    records_manager: Rc<RefCell<RecordsManager>>,
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

fn insert_data_ptr(payload: &mut[u8], offset: usize, data_ptr: &u64) -> usize {
    payload[offset..offset+NODE_PTR_SIZE].copy_from_slice(&data_ptr.to_be_bytes());
    NODE_PTR_SIZE
}


fn update_counter(payload: &mut[u8], count: u16) -> usize {
    payload[..2].copy_from_slice(&count.to_be_bytes());
    2
}

enum CellLoadRes {
    InteriorCell(NodeId),
    LeafCell(Vec<NodeId>),
}

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        let rec_mngr = RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD);
        BTreeNodeStore{records_manager: Rc::new(RefCell::new(rec_mngr))}
    }

    fn retrieve_overflow_cells(&mut self, pool: &mut NodeRecordPool, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> Option<CellLoadRes> {
        let overflow_cell_records = self.load_overflow_cell_records(pool, cell_record)?;
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

    fn retrieve_cell(&mut self, pool: &mut NodeRecordPool, cell_record: &CellRecord) -> Option<Cell> {
        let mut vkey = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let cell_load_res = self.retrieve_overflow_cells(pool, &cell_record, &mut vkey);
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
        let mut pool = NodeRecordPool::new(self.records_manager.clone());
        let node = pool.load_node_record_clone(nid)?;
        let mut cells = Vec::new();
        for cell_record in &node.cells {
            if cell_record.is_active() {
                cells.push(self.retrieve_cell(&mut pool, cell_record)?);
            }
        }
        let next_node_ptr = {
            if node.has_next_node() {
                Some(node.ptr)
            } else {
                None
            }
        };
        
        pool.save_all_node_records()?;

        Some(BTreeNode::new_with_id(Some(nid), next_node_ptr, node.is_leaf(), node.is_root(), cells))
    }

    fn update_overflow_cells(&mut self, pool: &mut NodeRecordPool, cell_records: &Vec<CellRecord>, prev_cell_record: &CellRecord) -> Option<(NodeId, CellId)> {
        let mut curr_cell_id = prev_cell_record.overflow_cell_ptr;
        let mut curr_node_id = prev_cell_record.node_ptr;
        let mut curr_node_record = pool.load_node_record_mut(curr_node_id)?;
        let mut prev_cell_id = curr_cell_id;
        let mut prev_node_id = curr_node_id;
        for cell in cell_records {
            prev_cell_id = curr_cell_id;
            prev_node_id = curr_node_id;
            curr_node_record.cells[curr_cell_id as usize] = *cell;
            if curr_node_id != cell.node_ptr {
                curr_node_record = pool.load_node_record_mut(curr_node_id)?;
            }
            curr_cell_id = cell.overflow_cell_ptr;
            curr_node_id = cell.node_ptr;
        }
        Some((prev_node_id, prev_cell_id))
    }

    fn load_or_create_free_cells_overflow_node(&mut self, pool: &mut NodeRecordPool) -> Option<NodeId> {
        if pool.records_manager.borrow_mut().is_empty() {
            let mut first_free_node = BNodeRecord::new();
            first_free_node.set_overflow_node();
            let new_record = pool.create_node_record(first_free_node)?;
            self.set_first_free_list_node_ptr(new_record);
            Some(new_record)
        } else {
            let first_free_record_ptr = self.get_first_free_list_node_ptr();
            if first_free_record_ptr == 0 {
                let next_free_cells_overflow_node = self.create_overflow_node(pool)?;
                self.set_first_free_list_node_ptr(next_free_cells_overflow_node);
                Some(next_free_cells_overflow_node)
            } else {
                let free_node_record = pool.load_node_record_ref(first_free_record_ptr)?;
                if free_node_record.is_full() {
                    self.set_first_free_list_node_ptr(free_node_record.next_free_cells_node_ptr);
                    self.load_or_create_free_cells_overflow_node(pool)
                } else {
                    Some(first_free_record_ptr)
                }
            }
        }
    }

    fn create_overflow_node(&mut self, pool: &mut NodeRecordPool) -> Option<NodeId> {
        let mut next_free_cells_overflow_node = BNodeRecord::new();
        next_free_cells_overflow_node.set_overflow_node();
        let id = pool.create_node_record(next_free_cells_overflow_node)?;
        Some(id)
    }

    fn create_overflow_cells(&mut self, pool: &mut NodeRecordPool, reverse_cell_records: &mut [CellRecord]) -> Option<CellPos> {

        let mut free_cells_node_record_id = self.load_or_create_free_cells_overflow_node(pool)?;
        let mut reverse_cell_id = 0;
        let mut curr_cell_id: usize = 0;
        let mut prev_cell_ptr = 0;
        let mut prev_node_ptr = free_cells_node_record_id;
        let mut free_cells_node_record = pool.load_node_record_mut(free_cells_node_record_id)?;

        let mut is_not_ending_cell = reverse_cell_records.len() == 1;
        //loop to store all overflow cells
        loop {
            let free_cell = free_cells_node_record.cells[curr_cell_id];
            if !free_cell.is_active() {
                let cell = &mut reverse_cell_records[reverse_cell_id];
                //ending cell stores the node pointer for interior nodes
                if is_not_ending_cell {
                    cell.chain_with_cell_location((prev_node_ptr, prev_cell_ptr));
                } else {
                    is_not_ending_cell = true;
                }
                free_cells_node_record.cells[curr_cell_id] = *cell;
                
                reverse_cell_id += 1;
                if reverse_cell_id >= reverse_cell_records.len() {
                    if curr_cell_id >= NB_CELL - 1 {
                        self.pop_node_record_from_free_list(&free_cells_node_record);
                    }
                    break;
                }
            }
            prev_cell_ptr = curr_cell_id as u32;
            curr_cell_id += 1;
            if curr_cell_id >= NB_CELL {
                self.pop_node_record_from_free_list(&free_cells_node_record);
                prev_node_ptr = free_cells_node_record_id;
                free_cells_node_record_id = self.load_or_create_free_cells_overflow_node(pool)?;
                free_cells_node_record = pool.load_node_record_mut(free_cells_node_record_id)?;
                curr_cell_id = 0;

            }
        }
        Some((prev_node_ptr, curr_cell_id as u32))
    }

    fn pop_node_record_from_free_list(&mut self, node_record: &BNodeRecord) {
        self.set_first_free_list_node_ptr(node_record.next_free_cells_node_ptr);
    }

    fn create_cell(&mut self, pool: &mut NodeRecordPool, cell: &Cell) -> Option<Vec<CellRecord>> {
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
            cell_record.set_is_active();
            cell_record.set_is_list_ptr();
            let mut data_ptr_count: u16 = 0;
            let mut whole_data_ptr_count = 0;
            for data_ptr in cell.get_data_ptrs_ref() {
                data_ptr_offset += insert_data_ptr(&mut cell_record.key, data_ptr_offset, data_ptr);
                data_ptr_count += 1;
                whole_data_ptr_count += 1;
                if  data_ptr_offset + NODE_PTR_SIZE >= KEY_SIZE {
                    update_counter(&mut cell_record.key, data_ptr_count);
                    cell_records.push(cell_record);
                    cell_record = CellRecord::new();
                    cell_record.set_is_active();
                    cell_record.set_is_list_ptr();
                    data_ptr_offset = 2;
                    data_ptr_count = 0;
                } else if whole_data_ptr_count == cell.get_data_ptrs_ref().len() {
                    update_counter(&mut cell_record.key, data_ptr_count);
                    cell_records.push(cell_record);
                    break;
                }
            }
            
        }

        let nb_records = cell_records.len();
        if nb_records > 1 {
            cell_records.reverse();
            for index in 1..nb_records {
                cell_records[index].set_has_overflow();
            }

            //store node ptr into last cell if any
            let last_cell_record = cell_records.first_mut()?;
            if let Some(node_ptr) = cell.get_node_ptr() {
                last_cell_record.node_ptr = node_ptr;
            }
            
            let ptrs = self.create_overflow_cells(pool, &mut cell_records[..nb_records-1])?;
            cell_records.reverse();
            let main_cell_record = cell_records.first_mut()?;
            main_cell_record.chain_with_cell_location(ptrs);

        }

        if let (Some(last_cell_record), Some(node_ptr)) = (cell_records.last_mut(), cell.get_node_ptr()) {
            last_cell_record.node_ptr = node_ptr;
        }

        Some(cell_records)
    }

    pub fn create(&mut self, node: &mut BTreeNode) -> Option<()> {
        let mut node_record = BNodeRecord::new();
        let mut pool = NodeRecordPool::new(self.records_manager.clone());
        if node.is_leaf() {
            node_record.set_leaf();
        }
        if node.is_root() {
            node_record.set_root();
        }
        if let Some(next_id) = node.get_node_ptr() {
            node_record.set_has_next_node();
            node_record.ptr = next_id;
        }
        
        let mut cell_id = 0;
        for cell in node.get_cells_ref() {
            let cell_records = self.create_cell(&mut pool, cell)?;
            node_record.cells[cell_id] = *cell_records.first()?;
            cell_id += 1;
        }
        let id = pool.create_node_record(node_record)?;
        node.set_id(id);
        if node.is_root() {
            self.set_root_node_ptr(id);
        }

        pool.save_all_node_records()?;

        Some(())
    }

    fn load_overflow_cell_records(&mut self, pool: &mut NodeRecordPool, root_cell_record: &CellRecord) -> Option<Vec<CellRecord>> {
        let mut cells = Vec::new();
        let mut curr_node_id = root_cell_record.node_ptr;
        let mut curr_overflow_cell_id = root_cell_record.overflow_cell_ptr;
        let mut has_overflow = root_cell_record.has_overflow();
        if has_overflow {
            let mut curr_node = pool.load_node_record_ref(curr_node_id)?;
            while has_overflow {
                let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
                has_overflow = overflow_cell.has_overflow();
                if has_overflow && curr_overflow_cell_id == overflow_cell.overflow_cell_ptr {
                    error!("cycle detected in node {} for cell {}", curr_node_id, curr_overflow_cell_id);
                    break;
                }
                curr_node_id = overflow_cell.node_ptr;
                curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
                cells.push(*overflow_cell);
                if curr_node_id != overflow_cell.node_ptr {
                    curr_node = pool.load_node_record_ref(curr_node_id)?;
                }
            }
        }
        
        Some(cells)
    }

    fn update_cell_data_ptrs(&mut self, pool: &mut NodeRecordPool, root_cell_record: &CellRecord, data_ptrs: &Vec<NodeId>) -> Option<()> {
        let overflow_cell_records = self.load_overflow_cell_records(pool, root_cell_record)?;
        let mut list_ptr_cells = overflow_cell_records;
        let mut cells_to_create = Vec::new();
        let mut cells_to_update = Vec::new();
        let mut data_ptr_offset = 2;
        let mut data_ptr_count: u16 = 0;
        let mut whole_data_ptr_count = 0;
        let mut curr_list_ptr_cell = list_ptr_cells.pop()?;
        let mut to_create= false;
        for data_ptr in data_ptrs {
            data_ptr_offset += insert_data_ptr(&mut curr_list_ptr_cell.key, data_ptr_offset, data_ptr);
            data_ptr_count += 1;
            whole_data_ptr_count += 1;
            if data_ptr_offset + NODE_PTR_SIZE >= KEY_SIZE {
                update_counter(&mut curr_list_ptr_cell.key, data_ptr_count);
                if to_create {
                    cells_to_create.push(curr_list_ptr_cell);
                } else {
                    cells_to_update.push(curr_list_ptr_cell);
                }
                data_ptr_offset = 2;
                data_ptr_count = 0;
                curr_list_ptr_cell = {
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
            } else if whole_data_ptr_count == data_ptrs.len() {
                update_counter(&mut curr_list_ptr_cell.key, data_ptr_count);
                if to_create {
                    cells_to_create.push(curr_list_ptr_cell);
                } else {
                    cells_to_update.push(curr_list_ptr_cell);
                }
                break;
            }
        }

        let last_updated_cell_pos = self.update_overflow_cells(pool, &cells_to_update, &root_cell_record)?;
        if cells_to_create.len() > 0 {
            cells_to_create.reverse();
            let created_first_cell_pos = self.create_overflow_cells(pool, &mut cells_to_create)?;
            //link last updated cell to created cells
            let last_updated_node =  pool.load_node_record_mut(last_updated_cell_pos.0)?;
            let last_updated_cell = &mut last_updated_node.cells[last_updated_cell_pos.1 as usize];
            last_updated_cell.set_has_overflow();
            last_updated_cell.chain_with_cell_location(created_first_cell_pos);
        }

        //disable unused cells
        if list_ptr_cells.len() > 0 {
            self.delete_cell_records(pool, &list_ptr_cells, last_updated_cell_pos.0, last_updated_cell_pos.1)?;
        }
        
        Some(())
    }

    fn delete_cell_records(&mut self, pool: &mut NodeRecordPool, cell_records_to_delete: &Vec<CellRecord>, first_cell_node_id: NodeId, first_cell_id: CellId) -> Option<()> {
        let mut curr_node_id = first_cell_node_id;
        let mut curr_cell_id = first_cell_id;
        for cell in cell_records_to_delete {
            let mut current_node_record = pool.load_node_record_mut(curr_node_id)?;
            if current_node_record.is_full() {
                self.append_node_record_to_free_list(curr_node_id, &mut current_node_record);
            }
            let current_cell = &mut current_node_record.cells[curr_cell_id as usize];
            current_cell.set_inactive();
            curr_cell_id = cell.overflow_cell_ptr;
            curr_node_id = cell.node_ptr;
        }
        Some(())
    }

    fn append_node_record_to_free_list(&mut self, node_record_id: NodeId, node_record: &mut BNodeRecord) {
        node_record.next_free_cells_node_ptr = self.get_first_free_list_node_ptr();
        self.set_first_free_list_node_ptr(node_record_id);
    }

    fn delete_cell_records_from_root_cell(&mut self, pool: &mut NodeRecordPool, root_cell_record: &CellRecord) -> Option<()> {
        let overflow_cell_records = self.load_overflow_cell_records(pool, root_cell_record)?;
        self.delete_cell_records(pool, &overflow_cell_records, root_cell_record.node_ptr, root_cell_record.overflow_cell_ptr)
    }

    pub fn save(&mut self, node: &mut BTreeNode) -> Option<()> {
        let id = node.get_id()?;
        let mut pool = NodeRecordPool::new(self.records_manager.clone());

        let mut cells_context = Vec::new();

        {
            let main_node_record = pool.load_node_record_mut(id)?;
            if !node.is_root() {
                main_node_record.set_is_not_root();
            } else {
                self.set_root_node_ptr(id);
            }
            
            for index in 0..main_node_record.cells.len() {
                if main_node_record.cells[index].is_active() {
                    cells_context.push(CellChangeContext::old(index));
                } else {
                    break;
                }
            }
        }
        

        //replay change log
        let mut list_old_ids_to_delete = Vec::new();
        for cell_change_log in node.get_node_changes_state().get_list_change_log() {
            if cell_change_log.is_remove() {
                let index = cell_change_log.index();
                let ctx = &cells_context[index];
                if !ctx.is_added {
                    list_old_ids_to_delete.push(ctx.old_cell_id)
                }
                cells_context.remove(index);
            } else if cell_change_log.is_add() {
                let index = cell_change_log.index();
                cells_context.insert(index, CellChangeContext::added());
            }
        }

        {
            let main_node_record = pool.load_node_record_mut(id)?;
    
            //delete old records
            for cell_id in &list_old_ids_to_delete {
                main_node_record.cells[*cell_id].set_inactive();
            }
        }

        {
            let main_node_record = pool.load_node_record_clone(id)?;

            //delete old records
            for cell_id in list_old_ids_to_delete {
                self.delete_cell_records_from_root_cell(&mut pool, &main_node_record.cells[cell_id]);
            }
        }
        {
            let main_node_record = pool.load_node_record_mut(id)?;
            let old_cell_records = main_node_record.cells;

            //move and update old records
            let mut new_cell_id = 0;
            for ctx in &cells_context {
                if !ctx.is_added {
                    main_node_record.cells[new_cell_id] = old_cell_records[ctx.old_cell_id];
                }
                new_cell_id += 1;
            }

        }

        {
            let main_node_record = pool.load_node_record_clone(id)?;
            //move and update old records
            let mut new_cell_id = 0;
            for ctx in &cells_context {
                if !ctx.is_added {
                    let current_cell = node.get_cell_ref(new_cell_id);
                    if current_cell.get_change_state().did_list_data_ptr_changed() {
                        self.update_cell_data_ptrs(&mut pool, &main_node_record.cells[new_cell_id], current_cell.get_data_ptrs_ref())?;
                    }
                }
                new_cell_id += 1;
            }
        }
        let root_cells = {
            let mut roots = Vec::new();
            //create new records
            let mut new_cell_id = 0;
            for ctx in &cells_context {
                if ctx.is_added {
                    let current_cell = node.get_cell_ref(new_cell_id);
                    let cell_records = self.create_cell(&mut pool, current_cell)?;
                    roots.push(cell_records[0]);
                }
                new_cell_id += 1;
            }
            roots
        };

        {
            let main_node_record = pool.load_node_record_mut(id)?;
            //create new records
            let mut new_cell_id = 0;
            let mut root_cell_id = 0;
            for ctx in &cells_context {
                if ctx.is_added {
                    main_node_record.cells[new_cell_id] = root_cells[root_cell_id];
                    root_cell_id += 1;
                }
                new_cell_id += 1;
            }
        }

        pool.save_all_node_records()?;

        Some(())
    }

    fn get_root_node_ptr(&mut self) -> NodeId {
        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&self.records_manager.borrow_mut().get_header_page_wrapper().get_header_payload_slice_ref()[..NODE_PTR_SIZE]);
        u64::from_be_bytes(buf)
    }

    fn set_root_node_ptr(&mut self, id: NodeId) {
        self.records_manager.borrow_mut().get_header_page_wrapper().get_header_payload_slice_mut()[..NODE_PTR_SIZE].copy_from_slice(&id.to_be_bytes());
    }

    fn get_first_free_list_node_ptr(&mut self) -> NodeId {
        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&self.records_manager.borrow_mut().get_header_page_wrapper().get_header_payload_slice_ref()[NODE_PTR_SIZE..2*NODE_PTR_SIZE]);
        u64::from_be_bytes(buf)
    }

    fn set_first_free_list_node_ptr(&mut self, id: NodeId) {
        self.records_manager.borrow_mut().get_header_page_wrapper().get_header_payload_slice_mut()[NODE_PTR_SIZE..2*NODE_PTR_SIZE].copy_from_slice(&id.to_be_bytes());
    }

    pub fn load_or_create_root_node(&mut self) -> Option<BTreeNode> {
        if self.is_empty() {
            let mut root = BTreeNode::new(true, true, Vec::new());
            self.create(&mut root)?;
            Some(root)
        } else {
            let root_node_id = self.get_root_node_ptr();
            self.retrieve_node(root_node_id)
        }
        
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_manager.borrow_mut().is_empty()
    }

    pub fn sync(&mut self) {
        self.records_manager.borrow_mut().sync();
    }
}

struct CellChangeContext {
    old_cell_id: usize,
    is_added: bool,
}

impl CellChangeContext {
    fn added() -> Self {
        CellChangeContext {
            old_cell_id: 0,
            is_added: true,
        }
    }
    fn old(index: usize) -> Self {
        CellChangeContext {
            old_cell_id: index,
            is_added: false,
        }
    }
}

#[cfg(test)]
mod test_btree_node_store {
    use super::*;    
    use super::super::super::super::test_utils::*;
    #[test]
    fn test_create() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_create.db").unwrap();
        let long_key = "blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6";
        let mut store = BTreeNodeStore::new(&file);
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

        let mut load_store = BTreeNodeStore::new(&file);
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

    #[test]
    fn test_many_ptrs() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_many_ptrs.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);
        let mut cells = Vec::new();
        cells.push(Cell::new("same key", Some(42), vec![12, 98, 78667867867, 21, 9], true));
        let mut node = BTreeNode::new(true, false, cells);
        store.create(&mut node);
        store.sync();
        node.insert_cell(1, Cell::new("same key", Some(43), vec![12, 98, 78667867867, 21, 9, 12, 98, 78667867867, 21, 9], true));
        store.save(&mut node).unwrap();

        store.sync();

        let loaded =  node.get_id().and_then(|id| store.retrieve_node(id)).unwrap();
        assert_eq!(loaded.get_cell_ref(0).get_data_ptrs_ref().len(), 5);
        assert_eq!(loaded.get_cell_ref(1).get_data_ptrs_ref().len(), 10);

    }

    #[test]
    fn test_many_ptrs_one_by_one() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_many_ptrs_one_by_one.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);
        let mut cells = Vec::new();
        cells.push(Cell::new("same key", Some(42), vec![12, 98, 77867867, 21, 9], true));
        let mut node = BTreeNode::new(true, false, cells);
        store.create(&mut node);
        store.sync();

        node.insert_cell(1, Cell::new("same key", Some(43), vec![12, 98, 78667867867, 21, 9, 12, 98, 78667867867, 21, 9], true));
        node.insert_cell(2, Cell::new("same key", Some(43), vec![12, 98, 78667867867, 21, 9, 12, 98], true));
        store.save(&mut node).unwrap();

        store.sync();

        for data_ptr in 0..100 {
            let mut loaded =  node.get_id().and_then(|id| store.retrieve_node(id)).unwrap();
            loaded.get_cell_mut(0).append_data_ptr(data_ptr);
            store.save(&mut loaded).unwrap();
        }
        let loaded =  node.get_id().and_then(|id| store.retrieve_node(id)).unwrap();
        assert_eq!(loaded.get_cell_ref(0).get_data_ptrs_ref().len(), 5);
        assert_eq!(loaded.get_cell_ref(1).get_data_ptrs_ref().len(), 10);
        assert_eq!(loaded.get_cell_ref(2).get_data_ptrs_ref().len(), 7);

    }

    //#[test]
    fn test_update_overflow_cells() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_update_overflow_cells.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);
        let mut pool = NodeRecordPool::new(store.records_manager.clone());
        let overflow_node = store.create_overflow_node(&mut pool).unwrap();
        let mut reverse_cell_records = Vec::new();
        let mut c0 = CellRecord::new();
        c0.set_has_overflow();
        c0.set_is_active();
        c0.set_is_list_ptr();
        insert_data_ptr(&mut c0.key, 2, &10);
        reverse_cell_records.push(c0);
        let first_created_loc = store.create_overflow_cells(&mut pool, &mut reverse_cell_records).unwrap();
        //store.update_overflow_cells(&mut pool, cell_records, prev_cell_record)

    }

    #[test]
    fn test_update_ptrs() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_update_ptrs.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);

        let long_key = "blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3";
        let mut cells = Vec::new();
        cells.push(Cell::new_leaf("blabla1", 11));
        cells.push(Cell::new_leaf("blabla2", 22));
        cells.push(Cell::new_leaf(long_key, 33));
        cells.push(Cell::new_leaf("blabla4", 44));
        cells.push(Cell::new_leaf("blabla5", 55));
        let mut node = BTreeNode::new(true, false, cells);

        store.create(&mut node);
        store.sync();

        let mut load_store = BTreeNodeStore::new(&file);
        let mut loaded =  node.get_id().and_then(|id| load_store.retrieve_node(id));

        if let Some(load) = &mut loaded {
            assert_eq!(load.get_node_ptr(), None);
            {
                let cell = load.get_cell_ref(3);
                assert_eq!(cell.get_key(), &String::from("blabla4"));
                assert_eq!(cell.get_node_ptr(), None);
                assert_eq!(cell.get_data_ptrs_ref(), &vec![44u64]);
    
                let cell1 = load.get_cell_ref(1);
                assert_eq!(cell1.get_key(), &String::from("blabla2"));
                assert_eq!(cell1.get_node_ptr(), None);
                assert_eq!(cell1.get_data_ptrs_ref(), &vec![22u64]);

                let long_key_cell = load.get_cell_ref(2);
                assert_eq!(long_key_cell.get_key(), &String::from(long_key));
                assert_eq!(long_key_cell.get_node_ptr(), None);
                assert_eq!(long_key_cell.get_data_ptrs_ref(), &vec![33u64]);
    
            }
            let update_long_key_cell = load.get_cell_mut(2);
            update_long_key_cell.append_data_ptr(9879);

            let update_short_key_cell = load.get_cell_mut(1);
            update_short_key_cell.append_data_ptr(578876);

            load_store.save(load);

        } else {
            assert!(false);
        }

        let updated =  node.get_id().and_then(|id| load_store.retrieve_node(id));
        if let Some(update) = &updated {
            let long_key_cell = update.get_cell_ref(2);
            assert_eq!(long_key_cell.get_key(), &String::from(long_key));
            assert_eq!(long_key_cell.get_node_ptr(), None);
            assert!(long_key_cell.get_data_ptrs_ref().contains(&33));
            assert!(long_key_cell.get_data_ptrs_ref().contains(&9879));

            
            let short_key_cell = update.get_cell_ref(1);
            assert_eq!(short_key_cell.get_key(), &String::from("blabla2"));
            assert_eq!(short_key_cell.get_node_ptr(), None);
            assert!(short_key_cell.get_data_ptrs_ref().contains(&22));
            assert!(short_key_cell.get_data_ptrs_ref().contains(&578876));

        } else {
            assert!(false);
        }
        
    }

}

