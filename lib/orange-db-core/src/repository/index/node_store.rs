use super::super::super::config::*;
use super::super::records::*;

const HAS_OVERFLOW_FLAG: u8 = 0b1000_0000;
const IS_LEAF_FLAG: u8 = 0b1000_0000;
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
        self.header & HAS_OVERFLOW_FLAG == 1
    }
    fn set_has_overflow(&mut self) {
        self.header = self.header | HAS_OVERFLOW_FLAG;
    }
    fn to_bytes(&self) -> [u8; CELL_SIZE] {
        let mut bytes = [0u8; CELL_SIZE];
        bytes[0] = self.header;
        bytes[CELL_HEADER_SIZE..CELL_HEADER_SIZE+NODE_PTR_SIZE].copy_from_slice(&self.node_ptr.to_be_bytes());
        bytes[CELL_HEADER_SIZE+NODE_PTR_SIZE..CELL_HEADER_SIZE+NODE_PTR_SIZE+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        bytes[KEY_SIZE..].copy_from_slice(&self.node_ptr.to_be_bytes());
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
        CellRecord{header: bytes[0],
            node_ptr: ptr,
            overflow_cell_ptr: overflow_cell_ptr,
            key: key}
    }
}
struct OverflowNodeRecord {
    header: u8,
    cells: [Cell; NB_CELL],
    ptr: u64,
}

struct BNodeRecord {
    header: u8,
    cells: Vec<CellRecord>,
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
        for cell_id in 0..self.cells.len() {
            bytes[index..index+CELL_SIZE].copy_from_slice(&self.cells[cell_id].to_bytes());
            index += CELL_SIZE;
        }
        bytes[index..].copy_from_slice(&self.ptr.to_be_bytes());
        bytes
    }
    fn from_bytes(bytes: [u8; BTREE_NODE_RECORD_SIZE]) -> Self {
        let mut index = 0;
        let header = bytes[index];
        index += 1;

        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&bytes[index..index+NODE_PTR_SIZE]);
        let ptr = u64::from_be_bytes(buf);

        let mut active_cells_buf = [0u8; ACTIVE_CELLS_COUNTER];
        active_cells_buf.copy_from_slice(&bytes[index..index+ACTIVE_CELLS_COUNTER]);
        index += ACTIVE_CELLS_COUNTER;
        let nb_active_cells = u32::from_be_bytes(active_cells_buf);

        let mut cells = Vec::new();
        for cell_id in 0..nb_active_cells {
            cells.push(CellRecord::from_bytes(&bytes[index..index+CELL_SIZE]));
            index += CELL_SIZE;
        }
        BNodeRecord{header: header, cells: cells, ptr: ptr}
    }
    fn is_leaf(&self) -> bool {
        (self.header & IS_LEAF_FLAG) == 1
    }
    fn set_leaf(&mut self) {
        self.header = self.header | IS_LEAF_FLAG;
    }
    fn new() -> Self {
        BNodeRecord{header: 0, cells: Vec::new(), ptr: 0}
    }
}

pub type NodeId = u64;
pub type CellId = u32;

pub struct Cell {
    pub key: String,
    pub node_ptr: NodeId,
}

pub struct BTreeNode {
    pub id: NodeId,
    pub cells: Vec<Cell>,
    pub node_ptr: NodeId,
}


pub struct BTreeNodeStore {
    records_manager: RecordsManager,
}

fn append_key(vkey: &mut Vec<u8>, key_buf: &[u8]) {
    let mut it = key_buf.iter();
    let str_end = it.position(|&c| c == b'\0').unwrap_or(key_buf.len());
    let mut vstring = Vec::with_capacity(str_end);
    vstring.extend_from_slice(&key_buf[0..str_end]);
}

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        BTreeNodeStore{records_manager: RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD)}
    }

    fn load_overflow_cells(&mut self, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> NodeId {
        let mut curr_node_id = cell_record.node_ptr;
        let mut curr_overflow_cell_id = cell_record.overflow_cell_ptr;
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        let mut has_overflow = cell_record.has_overflow();
        let mut load_node = true;
        let mut curr_node = BNodeRecord::new();
        while has_overflow {
            if load_node {
                self.records_manager.load(curr_node_id, &mut data);
                curr_node = BNodeRecord::from_bytes(data);
            }
            let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
            has_overflow = overflow_cell.has_overflow();
            append_key(vkey, &overflow_cell.key);
            curr_node_id = overflow_cell.node_ptr;
            curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
            load_node = curr_node_id != overflow_cell.node_ptr;
        }
        curr_node_id
    }

    fn retrieve_cell(&mut self, cell_record: &CellRecord) -> Cell {
        let mut vkey = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let node_id = self.load_overflow_cells(&cell_record, &mut vkey);
        Cell{key: String::from_utf8(vkey).unwrap(), node_ptr: node_id}
    }

    pub fn retrieve_node(&mut self, nid: NodeId) -> BTreeNode {
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        self.records_manager.load(nid, &mut data);
        let node = BNodeRecord::from_bytes(data);
        let mut cells = Vec::new();
        for cell_record in &node.cells {
            cells.push(self.retrieve_cell(&cell_record));
        }
        BTreeNode{id: nid, node_ptr: node.ptr, cells: cells}
    }
}