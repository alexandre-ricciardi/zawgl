use super::super::records::*;

const NB_CELL: usize = 66;
const PTR_SIZE: usize = 8;
const KEY_SIZE: usize = 40;
const CELL_HEADER_SIZE: usize = 1;
const CELL_SIZE: usize = KEY_SIZE + PTR_SIZE + CELL_HEADER_SIZE + OVERFLOW_CELL_PTR_SIZE;
const BTREE_NODE_RECORD_SIZE: usize = CELL_SIZE * NB_CELL + PTR_SIZE;
const OVERFLOW_CELL_PTR_SIZE: usize = 4;
const OVERFLOW_KEY_SIZE: usize = CELL_SIZE - OVERFLOW_CELL_PTR_SIZE;

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
        bytes[CELL_HEADER_SIZE..CELL_HEADER_SIZE+PTR_SIZE].copy_from_slice(&self.node_ptr.to_be_bytes());
        bytes[CELL_HEADER_SIZE+PTR_SIZE..CELL_HEADER_SIZE+PTR_SIZE+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        bytes[KEY_SIZE..].copy_from_slice(&self.node_ptr.to_be_bytes());
        bytes
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = CELL_HEADER_SIZE;
        let mut buf = [0u8; PTR_SIZE];
        buf.copy_from_slice(&bytes[offset..offset+PTR_SIZE]);
        let ptr = u64::from_be_bytes(buf);
        offset += PTR_SIZE;
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

struct KeyOverflowCell {
    ptr: u32,
    key: [u8; OVERFLOW_KEY_SIZE],
}

struct OverflowNodeRecord {
    header: u8,
    cells: [KeyOverflowCell; NB_CELL],
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
        let mut cells = [CellRecord::new(); NB_CELL];
        for cell_id in 0..cells.len() {
            cells[cell_id] = CellRecord::from_bytes(&bytes[index..index+CELL_SIZE]);
            index += CELL_SIZE;
        }
        let mut buf = [0u8; PTR_SIZE];
        buf.copy_from_slice(&bytes[index..]);
        let ptr = u64::from_be_bytes(buf);
        BNodeRecord{header: header, cells: cells, ptr: ptr}
    }
    fn len(&self) -> usize {
        (self.header & 0x00FF) as usize
    }
    fn is_leaf(&self) -> bool {
        (self.header & 0b1000_0000) == 1
    }
}

pub type NodeId = u64;

pub struct Cell {
    key: String,
    node_ptr: NodeId,
}

pub struct BTreeNode {
    cells: Vec<Cell>,
    node_ptr: NodeId,
}


pub struct BTreeNodeStore {
    records_manager: RecordsManager,
}

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        BTreeNodeStore{records_manager: RecordsManager::new(file, BTREE_NODE_RECORD_SIZE)}
    }

    pub fn retrieve_node(&mut self, nid: NodeId) -> BTreeNode {
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        self.records_manager.load(nid, &mut data);
        let node = BNodeRecord::from_bytes(data);
    }
}