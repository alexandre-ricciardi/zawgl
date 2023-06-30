// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::repository::index::model::{CellId, NodeId};

use super::super::super::super::buf_config::*;

const HAS_OVERFLOW_CELL_FLAG: u8 = 0b1000_0000;
const IS_ACTIVE_CELL_FLAG: u8 = 0b0100_0000;
const IS_LIST_PTR_CELL_FLAG: u8 = 0b0010_0000;

const IS_LEAF_NODE_FLAG: u8 = 0b1000_0000;
const HAS_NEXT_NODE_FLAG: u8 = 0b0100_0000;
const IS_OVERFLOW_NODE_FLAG: u8 = 0b0010_0000;
const IS_ROOT_NODE_FLAG: u8 = 0b0001_0000;

#[derive(Debug, Copy, Clone)]
pub struct CellRecord {
    pub header: u8,
    pub node_ptr: u64,
    pub overflow_cell_ptr: u32,
    pub key: [u8; KEY_SIZE],
}

impl CellRecord {
    pub fn new() -> Self {
        CellRecord{header: 0, key: [0u8; KEY_SIZE], node_ptr: 0, overflow_cell_ptr: 0}
    }
    pub fn has_overflow(&self) -> bool {
        self.header & HAS_OVERFLOW_CELL_FLAG > 0
    }
    pub fn set_has_overflow(&mut self) {
        self.header |= HAS_OVERFLOW_CELL_FLAG;
    }
    pub fn is_active(&self) -> bool {
        self.header & IS_ACTIVE_CELL_FLAG > 0
    }
    pub fn set_is_active(&mut self) {
        self.header |= IS_ACTIVE_CELL_FLAG;
    }
    
    pub fn set_inactive(&mut self) {
        self.header &= !IS_ACTIVE_CELL_FLAG;
    }

    pub fn is_list_ptr(&self) -> bool {
        self.header & IS_LIST_PTR_CELL_FLAG > 0
    }
    pub fn set_is_list_ptr(&mut self) {
        self.header |= IS_LIST_PTR_CELL_FLAG;
    }
    pub fn to_bytes(self) -> [u8; CELL_SIZE] {
        let mut bytes = [0u8; CELL_SIZE];
        let mut offset = 0;
        bytes[offset] = self.header;
        offset += CELL_HEADER_SIZE;
        bytes[offset..offset+NODE_PTR_SIZE].copy_from_slice(&self.node_ptr.to_be_bytes());
        offset += NODE_PTR_SIZE;
        bytes[offset..offset+OVERFLOW_CELL_PTR_SIZE].copy_from_slice(&self.overflow_cell_ptr.to_be_bytes());
        offset += OVERFLOW_CELL_PTR_SIZE;
        bytes[offset..offset+KEY_SIZE].copy_from_slice(&self.key);
        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;
        let header = bytes[offset];
        offset += CELL_HEADER_SIZE;
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
            header,
            node_ptr: ptr,
            overflow_cell_ptr,
            key}
    }

    pub fn chain_with_cell_location(&mut self, location: (NodeId, CellId)) {
        self.node_ptr = location.0;
        self.overflow_cell_ptr = location.1;
    }

    pub fn get_next_cell_location(&self) -> (NodeId, CellId) {
        (self.node_ptr, self.overflow_cell_ptr)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BNodeRecord {
    pub header: u8,
    pub next_free_cells_node_ptr: u64,
    pub cells: [CellRecord; NB_CELL],
    pub ptr: u64,
}

impl BNodeRecord {
    pub fn get_ptr_value(&self, n: usize) -> u64 {
        self.cells[n].node_ptr
    }
    pub fn get_key_value(&self, n: usize) -> String {
        String::from_utf8(self.cells[n].key.to_vec()).unwrap()
    }
    pub fn get_keys_string(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for (cell_id, cell) in self.cells.iter().enumerate() {
            keys[cell_id] = String::from_utf8(cell.key.to_vec()).unwrap();
        }
        keys
    }
    pub fn to_bytes(self) -> [u8; BTREE_NODE_RECORD_SIZE] {
        let mut bytes = [0u8; BTREE_NODE_RECORD_SIZE];
        let mut index = 0;
        bytes[index] = self.header;
        index += BTREE_NODE_HEADER_SIZE;
        bytes[index..index+NODE_PTR_SIZE].copy_from_slice(&self.ptr.to_be_bytes());
        index += NODE_PTR_SIZE;
        
        bytes[index..index+FREE_CELLS_NEXT_NODE_PTR_SIZE].copy_from_slice(&self.next_free_cells_node_ptr.to_be_bytes());
        index += FREE_CELLS_NEXT_NODE_PTR_SIZE;
        for cell_id in 0..NB_CELL {
            bytes[index..index+CELL_SIZE].copy_from_slice(&self.cells[cell_id].to_bytes());
            index += CELL_SIZE;
        }
        bytes
    }
    pub fn from_bytes(bytes: &[u8; BTREE_NODE_RECORD_SIZE]) -> Self {
        let mut index = 0;
        let header = bytes[index];
        index += BTREE_NODE_HEADER_SIZE;

        let mut buf = [0u8; NODE_PTR_SIZE];
        buf.copy_from_slice(&bytes[index..index+NODE_PTR_SIZE]);
        let ptr = u64::from_be_bytes(buf);
        index += NODE_PTR_SIZE;

        let mut free_cells_buf = [0u8; FREE_CELLS_NEXT_NODE_PTR_SIZE];
        free_cells_buf.copy_from_slice(&bytes[index..index+FREE_CELLS_NEXT_NODE_PTR_SIZE]);
        index += FREE_CELLS_NEXT_NODE_PTR_SIZE;
        let next_free_cells_node_ptr = u64::from_be_bytes(free_cells_buf);

        let mut cells = [CellRecord::new(); NB_CELL];
        for cell_id in 0..NB_CELL {
            let offset = index + cell_id * CELL_SIZE;
            cells[cell_id] = CellRecord::from_bytes(&bytes[offset..offset+CELL_SIZE]);
        }
        BNodeRecord{header, next_free_cells_node_ptr, cells, ptr}
    }
    pub fn is_leaf(&self) -> bool {
        (self.header & IS_LEAF_NODE_FLAG) > 0
    }
    pub fn set_leaf(&mut self) {
        self.header |= IS_LEAF_NODE_FLAG;
    }
    
    pub fn is_root(&self) -> bool {
        (self.header & IS_ROOT_NODE_FLAG) > 0
    }
    pub fn set_root(&mut self) {
        self.header |= IS_ROOT_NODE_FLAG;
    }
    pub fn set_is_not_root(&mut self) {
        self.header &= !IS_ROOT_NODE_FLAG;
    }
    
    pub fn is_overflow_node(&self) -> bool {
        (self.header & IS_OVERFLOW_NODE_FLAG) > 0
    }
    pub fn set_overflow_node(&mut self) {
        self.header |= IS_OVERFLOW_NODE_FLAG;
    }

    pub fn has_next_node(&self) -> bool {
        (self.header & HAS_NEXT_NODE_FLAG) > 0
    }
    pub fn set_has_next_node(&mut self) {
        self.header |= HAS_NEXT_NODE_FLAG;
    }

    pub fn new() -> Self {
        BNodeRecord{header: 0, next_free_cells_node_ptr: 0, cells: [CellRecord::new(); NB_CELL], ptr: 0}
    }

    pub fn is_full(&self) -> bool {
        let mut is_full = true;
        for cell_id in 0..self.cells.len() {
            if !self.cells[cell_id].is_active() {
                is_full = false;
                break;
            }
        }
        is_full
    }
}


#[cfg(test)]
mod test_btree_node_records {
    use super::*;
    #[test]
    fn test_bytes() {
        let mut node = BNodeRecord::new();
        node.set_leaf();
        node.next_free_cells_node_ptr = 28967;
        node.set_has_next_node();
        node.cells[0].set_is_active();
        let bytes = node.to_bytes();
        let mut from = BNodeRecord::from_bytes(&bytes);

        assert!(from.is_leaf());
        assert_eq!(from.next_free_cells_node_ptr, 28967);
        assert!(from.has_next_node());
        assert!(from.cells[0].is_active());
        from.cells[0].set_is_list_ptr();
        from.cells[0].set_inactive();

        let fbytes = from.to_bytes();
        let from_1 = BNodeRecord::from_bytes(&fbytes);

        assert!(from_1.is_leaf());
        assert!(from_1.has_next_node());
        assert!(!from_1.cells[0].is_active());
        assert!(from_1.cells[0].is_list_ptr());

    }
}