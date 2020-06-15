use super::super::super::config::*;
use super::super::records::*;

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

pub type NodeId = u64;
pub type CellId = u32;

struct CellChangeContext {
    added_data_ptrs: Vec<NodeId>,
    removed_data_ptrs: Vec<NodeId>,
}

impl CellChangeContext {
    fn new() -> Self {
        CellChangeContext{added_data_ptrs: Vec::new(), removed_data_ptrs: Vec::new()}
    }
}

pub struct Cell {
    pub key: String,
    pub node_ptr: NodeId,
    pub is_active: bool,
    data_ptrs: Vec<NodeId>,
    cell_change_ctx: CellChangeContext,
}

impl Cell {
    pub fn new_ptr(key: &str, ptr: NodeId) -> Self {
        Cell{key: String::from(key), node_ptr: ptr, is_active: true, data_ptrs: Vec::new(), cell_change_ctx: CellChangeContext::new()}
    }
    pub fn new_leaf(key: &str, data_ptr: NodeId) -> Self {
        Cell{key: String::from(key), node_ptr: 0, is_active: true, data_ptrs: vec![data_ptr], cell_change_ctx: CellChangeContext::new()}
    }
    fn new(key: &str, ptr: NodeId, data_ptrs: Vec<NodeId>, is_active: bool) -> Self {
        Cell{key: String::from(key), node_ptr: ptr, is_active: is_active, data_ptrs: data_ptrs, cell_change_ctx: CellChangeContext::new()}
    }
    pub fn append_data_ptr(&mut self, data_ptr: NodeId) {
        self.cell_change_ctx.added_data_ptrs.push(data_ptr);
        self.data_ptrs.push(data_ptr);
    }
    pub fn get_data_ptrs_ref(&self) -> &Vec<NodeId> {
        &self.data_ptrs
    }
}

pub struct BTreeNode {
    pub id: Option<NodeId>,
    pub cells: Vec<Cell>,
    pub node_ptr: Option<NodeId>,
    pub is_leaf: bool,
}

impl BTreeNode {
    pub fn new(is_leaf: bool) -> Self {
        BTreeNode{id: None, cells: Vec::new(), node_ptr: None, is_leaf: is_leaf}
    }

    pub fn is_full(&self) -> bool {
        self.cells.len() == NB_CELL
    }

    pub fn get_keys(&self) -> Vec<String> {
        let mut res = Vec::new();
        for cell in &self.cells {
            if cell.is_active {
                res.push(cell.key.to_owned());
            }
        }
        res
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

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        BTreeNodeStore{records_manager: RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD)}
    }

    fn load_overflow_cells(&mut self, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> Option<(NodeId, Vec<NodeId>)> {
        let mut curr_node_id = cell_record.node_ptr;
        let mut curr_overflow_cell_id = cell_record.overflow_cell_ptr;
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        let mut has_overflow = cell_record.has_overflow();
        let mut load_node = true;
        let mut curr_node = BNodeRecord::new();
        let mut ptrs = Vec::new();
        while has_overflow {
            if load_node {
                self.records_manager.load(curr_node_id, &mut data).ok()?;
                curr_node = BNodeRecord::from_bytes(data);
            }
            let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
            has_overflow = overflow_cell.has_overflow();
            if overflow_cell.is_list_ptr() {
                append_list_ptr(&mut ptrs, &overflow_cell.key);
            } else {
                append_key(vkey, &overflow_cell.key);
            }
            curr_node_id = overflow_cell.node_ptr;
            curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
            load_node = curr_node_id != overflow_cell.node_ptr;
        }
        Some((curr_node_id, ptrs))
    }

    fn retrieve_cell(&mut self, cell_record: &CellRecord) -> Option<Cell> {
        let mut vkey = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let node_id = self.load_overflow_cells(&cell_record, &mut vkey);
        node_id.map(|res| Cell::new(&String::from_utf8(vkey).unwrap(), res.0, res.1, cell_record.is_active()))
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
        Some(BTreeNode{id: Some(nid), node_ptr: next_node_ptr, cells: cells, is_leaf: node.is_leaf()})
    }

    pub fn append_node(&mut self, node: &BTreeNode) -> Option<NodeId> {
        let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
        Some(self.records_manager.append(&data).ok()?)
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_manager.is_empty()
    }
}