use super::super::super::config::*;
use super::super::records::*;
use super::model::*;

const HAS_OVERFLOW_CELL_FLAG: u8 = 0b1000_0000;
const IS_ACTIVE_CELL_FLAG: u8 = 0b0100_0000;
const IS_LIST_PTR_CELL_FLAG: u8 = 0b0010_0000;

const IS_LEAF_NODE_FLAG: u8 = 0b1000_0000;
const HAS_NEXT_NODE_FLAG: u8 = 0b0100_0000;

#[derive(Copy, Clone)]
struct CellRecord {
    header: u8,
    node_ptr: u64,
    overflow_cell_ptr: u32,
    key: [u8; KEY_SIZE],
}

impl CellRecord {
    fn new() -> Self {
        CellRecord{header: 0, key: [0u8; KEY_SIZE], node_ptr: 0, overflow_cell_ptr: 0}
    }
    fn has_overflow(&self) -> bool {
        self.header & HAS_OVERFLOW_CELL_FLAG == 1
    }
    fn set_has_overflow(&mut self) {
        self.header = self.header | HAS_OVERFLOW_CELL_FLAG;
    }
    fn is_active(&self) -> bool {
        self.header & IS_ACTIVE_CELL_FLAG == 1
    }
    fn set_is_active(&mut self) {
        self.header = self.header | IS_ACTIVE_CELL_FLAG;
    }
    
    fn is_list_ptr(&self) -> bool {
        self.header & IS_LIST_PTR_CELL_FLAG == 1
    }
    fn set_is_list_ptr(&mut self) {
        self.header = self.header | IS_LIST_PTR_CELL_FLAG;
    }
    fn to_bytes(&self) -> [u8; CELL_SIZE] {
        let mut bytes = [0u8; CELL_SIZE];
        bytes[0] = self.header;
        let mut offset = CELL_HEADER_SIZE;
        bytes[offset..offset+NODE_PTR_SIZE].copy_from_slice(&self.node_ptr.to_be_bytes());
        offset += NODE_PTR_SIZE;
        bytes[offset..offset+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        offset += OVERFLOW_CELL_PTR_SIZE;
        bytes[offset..offset+KEY_SIZE].copy_from_slice(&self.node_ptr.to_be_bytes());
        bytes
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = CELL_HEADER_SIZE;
        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&bytes[offset..offset+NODE_PTR_SIZE]);
        let ptr = u64::from_be_bytes(buf);
        offset += NODE_PTR_SIZE;
        let mut overflow_cell_ptr_buf = [0u8; OVERFLOW_CELL_PTR_SIZE];
        overflow_cell_ptr_buf.copy_from_slice(&bytes[offset..offset+OVERFLOW_CELL_PTR_SIZE]);
        let overflow_cell_ptr = u32::from_be_bytes(overflow_cell_ptr_buf);
        offset += OVERFLOW_CELL_PTR_SIZE;
        let mut key = [0u8; KEY_SIZE];
        key.copy_from_slice(&bytes[offset..offset+KEY_SIZE]);
        CellRecord{
            header: bytes[0],
            node_ptr: ptr,
            overflow_cell_ptr: overflow_cell_ptr,
            key: key}
    }
}

struct BNodeRecord {
    header: u8,
    next_free_cells_node_ptr: u64,
    cells: [CellRecord; NB_CELL],
    ptr: u64,
}

impl BNodeRecord {
    fn get_ptr_value(&self, n: usize) -> u64 {
        self.cells[n].node_ptr
    }
    fn get_key_value(&self, n: usize) -> String {
        String::from_utf8(self.cells[n].key.to_vec()).unwrap()
    }
    fn get_keys_string(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for cell_id in 0..self.cells.len() {
            keys[cell_id] = String::from_utf8(self.cells[cell_id].key.to_vec()).unwrap();
        }
        keys
    }
    fn to_bytes(&self) -> [u8; BTREE_NODE_RECORD_SIZE] {
        let mut bytes = [0u8; BTREE_NODE_RECORD_SIZE];
        let mut index = 0;
        bytes[index] = self.header;
        index += 1;
        bytes[index..index+NODE_PTR_SIZE].copy_from_slice(&self.ptr.to_be_bytes());
        index += NODE_PTR_SIZE;
        bytes[index..index+FREE_CELLS_NEXT_NODE_PTR_SIZE].copy_from_slice(&self.next_free_cells_node_ptr.to_be_bytes());
        index += FREE_CELLS_NEXT_NODE_PTR_SIZE;
        for cell_id in 0..self.cells.len() {
            bytes[index..index+CELL_SIZE].copy_from_slice(&self.cells[cell_id].to_bytes());
            index += CELL_SIZE;
        }
        bytes
    }
    fn from_bytes(bytes: [u8; BTREE_NODE_RECORD_SIZE]) -> Self {
        let mut index = 0;
        let header = bytes[index];
        index += 1;

        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&bytes[index..index+NODE_PTR_SIZE]);
        let ptr = u64::from_be_bytes(buf);

        let mut free_cells_buf = [0u8; FREE_CELLS_NEXT_NODE_PTR_SIZE];
        free_cells_buf.copy_from_slice(&bytes[index..index+FREE_CELLS_NEXT_NODE_PTR_SIZE]);
        index += FREE_CELLS_NEXT_NODE_PTR_SIZE;
        let next_free_cells_node_ptr = u64::from_be_bytes(free_cells_buf);

        let mut cells = [CellRecord::new(); NB_CELL];
        for cell_id in 0..NB_CELL {
            let offset = index + cell_id * CELL_SIZE;
            cells[cell_id] = CellRecord::from_bytes(&bytes[offset..offset+CELL_SIZE]);
        }
        BNodeRecord{header: header, next_free_cells_node_ptr: next_free_cells_node_ptr, cells: cells, ptr: ptr}
    }
    fn is_leaf(&self) -> bool {
        (self.header & IS_LEAF_NODE_FLAG) == 1
    }
    fn set_leaf(&mut self) {
        self.header = self.header | IS_LEAF_NODE_FLAG;
    }
    fn has_next_node(&self) -> bool {
        (self.header & HAS_NEXT_NODE_FLAG) == 1
    }
    fn set_has_next_node(&mut self) {
        self.header = self.header | HAS_NEXT_NODE_FLAG;
    }
    fn new() -> Self {
        BNodeRecord{header: 0, next_free_cells_node_ptr: 0, cells: [CellRecord::new(); NB_CELL], ptr: 0}
    }
}

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

    fn create_overflow_cell(&mut self, cell_record: &CellRecord) {

    }

    fn create_cell(&mut self, node_record: &BNodeRecord, cell: &Cell) {
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
        
        if cell_records.len() > 1 {
            cell_records.reverse();
            for index in 1..cell_records.len() {
                cell_records[index].set_has_overflow();
            }
        }
        
    }

    fn load_root_node_

    fn create_node(&mut self, node: &mut BTreeNode) {
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
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.load(0, &mut data);
            let first_node = BNodeRecord::from_bytes(data);
            node_record.next_free_cells_node_ptr = first_node.next_free_cells_node_ptr;
            // need create node id first_node.next_free_cells_node_ptr = 0;
        }
        for cell in node.get_cells_ref() {

        }

    }

    pub fn save(&mut self, node: &mut BTreeNode) {

        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        Some(self.records_manager.create(&data).ok()?)
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_manager.is_empty()
    }
}