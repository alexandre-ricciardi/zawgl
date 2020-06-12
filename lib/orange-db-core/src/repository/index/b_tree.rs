use super::super::super::config::*;
use super::super::records::*;

const HAS_OVERFLOW_FLAG: u8 = 0b1000_0000;

#[derive(Copy, Clone)]
struct Cell {
    header: u8,
    ptr: u64,
    overflow_cell_ptr: u32,
    key: [u8; KEY_SIZE],
}

impl Cell {
    fn new() -> Self {
        Cell{header: 0, key: [0u8; KEY_SIZE], ptr: 0, overflow_cell_ptr: 0}
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
        bytes[CELL_HEADER_SIZE..CELL_HEADER_SIZE+NODE_PTR_SIZE].copy_from_slice(&self.ptr.to_be_bytes());
        bytes[CELL_HEADER_SIZE+NODE_PTR_SIZE..CELL_HEADER_SIZE+NODE_PTR_SIZE+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        bytes[KEY_SIZE..].copy_from_slice(&self.ptr.to_be_bytes());
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
        Cell{header: bytes[0],
            ptr: ptr,
            overflow_cell_ptr: overflow_cell_ptr,
            key: key,}
    }
}

#[derive(Copy, Clone)]
struct KeyOverflowCell {
    ptr: u32,
    key: [u8; OVERFLOW_KEY_SIZE],
}

#[derive(Copy, Clone)]
struct OverflowNodeRecord {
    header: u8,
    cells: [KeyOverflowCell; NB_CELL],
    ptr: u64,
}

#[derive(Copy, Clone)]
struct BNodeRecord<'a> {
    header: u8,
    cells: [Cell; NB_CELL],
    ptr: u64,
    parent: Option<&'a BNodeRecord<'a>>
}

impl <'a> BNodeRecord<'a> {
    fn get_ptr_value(&self, n: usize) -> u64 {
        self.cells[n].ptr
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
        let mut cells = [Cell::new(); NB_CELL];
        for cell_id in 0..cells.len() {
            cells[cell_id] = Cell::from_bytes(&bytes[index..index+CELL_SIZE]);
            index += CELL_SIZE;
        }
        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&bytes[index..]);
        let ptr = u64::from_be_bytes(buf);
        BNodeRecord{header: header, cells: cells, ptr: ptr, parent: None}
    }
    fn len(&self) -> usize {
        (self.header & 0x00FF) as usize
    }
    fn is_leaf(&self) -> bool {
        (self.header & 0b1000_0000) == 1
    }
}

pub type DataPtr = u64;
pub struct BTreeIndex {
    records_manager: RecordsManager,
}



impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{records_manager: RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD)}
    }

    fn tree_search(&mut self, value: &str, node: &BNodeRecord, depth: u32) -> Option<DataPtr> {
        let keys = node.get_keys_string();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf() {
                    Some(node.get_ptr_value(found))
                } else {
                    let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            },
            Err(not_found) => {
                if node.is_leaf() {
                    None
                } else {
                    let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(not_found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str) -> Option<DataPtr> {
        if self.records_manager.is_empty() {
            None
        } else {
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.load(0, &mut  data);
            self.tree_search(value, &BNodeRecord::from_bytes(data), 0)
        }
    }

    pub fn insert(&mut self, value: u64, data_ptr: u64) {

    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
    }
}