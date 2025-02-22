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

use super::super::super::buf_config::*;

pub type NodeId = u64;
pub type CellId = u32;

#[derive(Debug, Clone)]
pub struct CellChangeState {
    is_new_instance: bool,
    is_added: bool,
    is_removed: bool,
    list_data_pointer_changed: bool,
    is_append_only: bool,
    append_index: usize,
}

impl CellChangeState {
    fn new(new: bool, append_index: usize) -> Self {
        CellChangeState{is_new_instance: new, 
            is_added: false,
            is_removed: false, 
            list_data_pointer_changed: false, append_index, is_append_only: false}
    }
    fn set_is_removed(&mut self) {
        self.is_removed = true;
    }
    fn set_is_added(&mut self) {
        self.is_added = true;
    }
    pub fn is_removed(&self) -> bool {
        self.is_removed
    }
    pub fn is_added(&self) -> bool {
        self.is_added
    }
    pub fn did_list_data_ptr_changed(&self) -> bool {
        self.list_data_pointer_changed
    }
    pub fn is_append_only(&self) -> bool {
        self.is_append_only
    }
    pub fn append_index(&self) -> usize {
        self.append_index
    }

    pub fn reset(&mut self) {
        self.is_new_instance = false;
        self.is_added = false;
        self.is_removed = false;
    }
}
#[derive(Debug, Clone)]
pub struct Cell {
    key: String,
    node_ptr: Option<NodeId>,
    data_ptrs: Vec<NodeId>,
    cell_change_state: CellChangeState,
}

impl Cell {
    pub fn new_ptr(key: &str, ptr: Option<NodeId>) -> Self {
        Cell{key: String::from(key), node_ptr: ptr, data_ptrs: Vec::new(), cell_change_state: CellChangeState::new(true, 0)}
    }
    pub fn new_leaf(key: &str, data_ptr: NodeId) -> Self {
        Cell{key: String::from(key), node_ptr: None, data_ptrs: vec![data_ptr], cell_change_state: CellChangeState::new(true, 1)}
    }
    pub fn new(key: &str, ptr: Option<NodeId>, data_ptrs: Vec<NodeId>) -> Self {
        let index = data_ptrs.len();
        Cell{key: String::from(key), node_ptr: ptr, data_ptrs, cell_change_state: CellChangeState::new(false, index)}
    }
    pub fn append_data_ptr(&mut self, data_ptr: NodeId) {
        if !self.cell_change_state.list_data_pointer_changed {
            self.cell_change_state.is_append_only = true;
            self.cell_change_state.append_index += 1;
        } else {
            self.cell_change_state.is_append_only = false;
        }
        self.data_ptrs.push(data_ptr);
    }
    
    pub fn delete_data_ptr(&mut self, data_ptr: NodeId) {
        self.cell_change_state.list_data_pointer_changed = true;
        self.cell_change_state.is_append_only = false;
        self.data_ptrs.retain(|&curr| curr != data_ptr);
    }

    pub fn get_data_ptrs_ref(&self) -> &Vec<NodeId> {
        &self.data_ptrs
    }
    pub fn get_node_ptr(&self) -> Option<NodeId> {
        self.node_ptr
    }
    pub fn get_key(&self) -> &String {
        &self.key
    }
    pub fn get_change_state(&self) -> &CellChangeState {
        &self.cell_change_state
    }
    pub fn set_node_ptr(&mut self, id: Option<NodeId>) {
        self.node_ptr = id;
    }

    fn reset(&mut self) {

    }

}

#[derive(Debug, Clone)]
pub struct CellChangeLogItem {
    is_add: bool,
    is_remove: bool,
    index: usize,
}

impl CellChangeLogItem {
    fn new(index: usize, is_added: bool, is_removed: bool) -> Self {
        CellChangeLogItem{is_add: is_added, is_remove: is_removed, index}
    }

    pub fn is_remove(&self) -> bool {
        self.is_remove
    }

    pub fn is_add(&self) -> bool {
        self.is_add
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone)]

pub struct NodeChangeState {
    node_ptr_changed: bool,
    is_new_instance: bool,
    list_cell_change_log_items: Vec<CellChangeLogItem>,
}

impl NodeChangeState {
    fn new(is_new_instance: bool) -> Self {
        NodeChangeState{node_ptr_changed: false, 
            is_new_instance,
            list_cell_change_log_items: Vec::new()}
    }

    pub fn did_node_ptr_changed(&self) -> bool {
        self.node_ptr_changed
    }
    pub fn is_new_instance(&self) -> bool {
        self.is_new_instance
    }
    pub fn get_list_change_log(&self) -> &Vec<CellChangeLogItem> {
        &self.list_cell_change_log_items
    }

    pub fn reset(&mut self) {
        self.list_cell_change_log_items.clear();
    }
}

#[derive(Debug, Clone)]
pub struct BTreeNode {
    id: Option<NodeId>,
    cells: Vec<Cell>,
    node_ptr: Option<NodeId>,
    is_leaf: bool,
    is_root: bool,
    node_change_state: NodeChangeState,
}

impl BTreeNode {
    pub fn new(is_leaf: bool, is_root: bool, cells: Vec<Cell>) -> Self {
        let state = NodeChangeState::new(true);
        BTreeNode{id: None, cells, node_ptr: None, is_leaf, is_root, node_change_state: state}
    }

    pub fn new_with_id(id: Option<NodeId>, node_ptr: Option<NodeId>, is_leaf: bool, is_root: bool, cells: Vec<Cell>) -> Self {
        let state = NodeChangeState::new(false);
        BTreeNode{id, cells, node_ptr, is_leaf, is_root, node_change_state: state}
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().filter(|c| !c.cell_change_state.is_removed()).count() == NB_CELL
    }

    pub fn len(&self) -> usize {
        self.cells.iter().filter(|c| !c.cell_change_state.is_removed()).count()
    }

    pub fn is_half_full(&self) -> bool {
        self.len() >= (NB_CELL / 2) - 1
    }

    pub fn get_keys(&self) -> Vec<&str> {
        let mut res: Vec<&str> = Vec::new();
        for cell in &self.cells {
            res.push(&cell.key);
        }
        res
    }

    pub fn get_cells_ref(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell_ref(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    pub fn insert_cell(&mut self, index: usize, mut cell: Cell) {
        cell.cell_change_state.set_is_added();
        let cell_change_log = CellChangeLogItem::new(index, true, false);
        self.node_change_state.list_cell_change_log_items.push(cell_change_log);
        self.cells.insert(index, cell);
    }

    pub fn append_cell(&mut self, cell: Cell) {
        let index = self.cells.len();
        self.insert_cell(index, cell);
    }

    pub fn remove_cell(&mut self, index: usize) {
        let to_remove = &mut self.cells[index];
        to_remove.cell_change_state.set_is_removed();
        let cell_change_log = CellChangeLogItem::new(index,false, true);
        self.node_change_state.list_cell_change_log_items.push(cell_change_log);
    }

    pub fn remove_cells(&mut self) {
        for (index, cell) in self.cells.iter_mut().rev().enumerate() {
            if !cell.cell_change_state.is_removed() {
                let cell_change_log = CellChangeLogItem::new(index,false, true);
                self.node_change_state.list_cell_change_log_items.push(cell_change_log);
                cell.cell_change_state.set_is_removed();
            }
        }
    }

    pub fn pop_cell(&mut self) -> Option<Cell> {
        let result = self.cells.iter_mut().enumerate().rev().find(|(_index, c)| !c.cell_change_state.is_removed());
        if let Some((index, cell)) = result {
            let mut cloned_cell = cell.clone();
            let cell_change_log = CellChangeLogItem::new(index,false, true);
            self.node_change_state.list_cell_change_log_items.push(cell_change_log);
            cell.cell_change_state.set_is_removed();
            cloned_cell.cell_change_state.reset();
            Some(cloned_cell)
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    

    pub fn set_is_root(&mut self, is_root: bool) {
        self.is_root = is_root;
    }
    
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn get_id(&self) -> Option<NodeId> {
        self.id
    }

    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
    }

    pub fn get_node_ptr(&self) -> Option<NodeId> {
        self.node_ptr
    }

    pub fn set_node_ptr(&mut self, id: Option<NodeId>) {
        self.node_change_state.node_ptr_changed = true;
        self.node_ptr = id;
    }

    pub fn get_node_changes_state(&self) -> &NodeChangeState {
        &self.node_change_state
    }

    pub fn reset(&mut self) {
        let cells = self.cells.iter_mut().filter(|c| !c.cell_change_state.is_removed())
            .map(|c| {c.reset(); c.clone()}).collect();
        self.cells = cells;
        self.node_change_state.reset();
    }
}