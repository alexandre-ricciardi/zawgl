use super::super::records::*;

pub struct BNode {

}


const NB_CELL: usize = 66;
const PTR_SIZE: usize = 8;
const KEY_SIZE: usize = 40;
const CELL_HEADER_SIZE: usize = 1;
const CELL_SIZE: usize = KEY_SIZE + PTR_SIZE + CELL_HEADER_SIZE + OVERFLOW_CELL_PTR_SIZE;
const RECORD_SIZE: usize = CELL_SIZE * NB_CELL + PTR_SIZE;
const OVERFLOW_CELL_PTR_SIZE: usize = 4;
const OVERFLOW_KEY_SIZE: usize = CELL_SIZE - OVERFLOW_CELL_PTR_SIZE;

const HAS_OVERFLOW: u8 = 0b1000_0000;

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
    fn to_bytes(&self) -> [u8; CELL_SIZE] {
        let mut bytes = [0u8; CELL_SIZE];
        bytes[0] = self.header;
        bytes[CELL_HEADER_SIZE..CELL_HEADER_SIZE+PTR_SIZE].copy_from_slice(&self.ptr.to_be_bytes());
        bytes[CELL_HEADER_SIZE+PTR_SIZE..CELL_HEADER_SIZE+PTR_SIZE+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        bytes[KEY_SIZE..].copy_from_slice(&self.ptr.to_be_bytes());
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
struct BNodeRecord {
    header: u8,
    cells: [Cell; NB_CELL],
    ptr: u64,
}

impl BNodeRecord {
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
    fn to_bytes(&self) -> [u8; RECORD_SIZE] {
        let mut bytes = [0u8; RECORD_SIZE];
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
    fn from_bytes(bytes: [u8; RECORD_SIZE]) -> Self {
        let mut index = 0;
        let header = bytes[index];
        index += 1;
        let mut cells = [Cell::new(); NB_CELL];
        for cell_id in 0..cells.len() {
            cells[cell_id] = Cell::from_bytes(&bytes[index..index+CELL_SIZE]);
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

pub struct BTreeIndex {
    records_manager: RecordsManager,
    tree_depth: u32,
}

pub type BTreeResult = std::result::Result<BNodeRecord, Option<BNodeRecord>>;

impl BTreeIndex {
    pub fn new(file: &str, tree_depth: u32) -> Self {
        BTreeIndex{records_manager: RecordsManager::new(file, RECORD_SIZE), tree_depth: tree_depth}
    }

    fn is_leaf_node(&self, depth: u32) -> bool {
        depth == self.tree_depth
    }

    fn tree_search(&mut self, value: &str, node: &BNodeRecord, depth: u32) -> BTreeResult {
        if self.is_leaf_node(depth) {
            Ok(*node)
        } else {
            let keys = node.get_keys_string();
            let res = keys.binary_search(&String::from(value));
            match res {
                Ok(found) => {
                    let mut data = [0u8; RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    if child.is_leaf() {
                        Ok(child)
                    } else {
                        self.tree_search(value, &child, depth+1)
                    }
                },
                Err(not_found) => {
                    let mut data = [0u8; RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(not_found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str) -> BTreeResult {
        if self.records_manager.is_empty() {
            Err(None)
        } else {
            let mut data = [0u8; RECORD_SIZE];
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