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

mod records;
mod pool;

use log::*;
use std::collections::HashMap;
use self::records::*;
use super::super::super::buf_config::*;
use super::model::*;
use super::super::records::*;
use self::pool::*;


pub type MutableRecordsManager = RecordsManager;

pub struct BTreeNodeStore {
    records_pool: NodeRecordPool,
    nodes_pool: BTreeNodePool,
}

fn append_key(vkey: &mut Vec<u8>, key_buf: &[u8]) {
    let mut it = key_buf.iter();
    let str_end = it.position(|&c| c == b'\0').unwrap_or(key_buf.len());
    vkey.extend_from_slice(&key_buf[0..str_end]);
}

fn append_list_ptr(ptrs: &mut Vec<u64>, buf: &[u8]) {
    let counter = read_counter(buf);
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

fn read_counter(payload: &[u8]) -> u16 {
    let mut counter_buf = [0u8; 2];
    counter_buf.copy_from_slice(&payload[..2]);
    u16::from_be_bytes(counter_buf)
}

enum CellLoadRes {
    InteriorCell(NodeId),
    LeafCell(Vec<NodeId>),
}

impl BTreeNodeStore {
    pub fn new(file: &str) -> Self {
        let records_manager = RecordsManager::new(file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD);
        let records_pool = NodeRecordPool::new(records_manager);
        let nodes_pool: BTreeNodePool = BTreeNodePool::new();
        BTreeNodeStore{records_pool, nodes_pool}
    }

    fn retrieve_overflow_cells(&mut self, cell_record: &CellRecord, vkey: &mut Vec<u8>) -> Option<CellLoadRes> {
        let overflow_cell_records = self.load_overflow_cell_records(cell_record)?;
        let mut is_leaf_cell = false;
        let mut ptrs = Vec::new();
        for overflow_cell in overflow_cell_records.iter().rev() {
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
            .or({
                Some(CellLoadRes::InteriorCell(cell_record.node_ptr))
            })
        }
    }

    fn retrieve_cell(&mut self, cell_record: &CellRecord) -> Option<Cell> {
        let mut vkey: Vec<u8> = Vec::new();
        append_key(&mut vkey, &cell_record.key);
        let cell_load_res = self.retrieve_overflow_cells(cell_record, &mut vkey);
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

    fn _retrieve_node(&mut self, nid: &NodeId) -> Option<BTreeNode> {
        let node = self.records_pool.load_node_record_clone(nid)?;
        let mut cells = Vec::new();
        for cell_record in &node.cells {
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
        
        Some(BTreeNode::new_with_id(Some(*nid), next_node_ptr, node.is_leaf(), node.is_root(), cells))
    }

    pub fn retrieve_node(&mut self, nid: &NodeId) -> Option<()> {
        let pool_node = self.nodes_pool.contains(nid);
        if !pool_node {
            self._retrieve_node(nid).and_then(|n| self.nodes_pool.put(n)).map(|_| ())
        } else {
            Some(())
        }
    }

    pub fn get_node_ref(&self, node_id: &NodeId) -> Option<&BTreeNode> {
        self.nodes_pool.get_ref(node_id)
    }

    pub fn get_node_clone(&self, node_id: &NodeId) -> Option<BTreeNode> {
        self.nodes_pool.get_ref(node_id).map(|nr| nr.clone())
    }

    pub fn is_root_node(&self, node_id: &NodeId) -> bool {
        self.get_node_ref(node_id).map_or(false, |n| n.is_root())
    }

    pub fn set_is_root(&mut self, node_id: &NodeId, is_root: bool) -> Option<()> {
        let node = self.get_node_mut(node_id)?;
        node.set_is_root(is_root);
        Some(())
    }

    pub fn is_leaf_node(&self, node_id: &NodeId) -> bool {
        self.get_node_ref(node_id).map_or(false, |n| n.is_leaf())
    }

    pub fn is_full_node(&self, node_id: &NodeId) -> bool {
        self.get_node_ref(node_id).map_or(false, |n| n.is_full())
    }

    pub fn get_node_ptr(&self, node_id: &NodeId) -> Option<NodeId> {
        self.get_node_ref(node_id).and_then(|n| n.get_node_ptr())
    }

    pub fn set_node_ptr(&mut self, node_id: &NodeId, node_ptr: Option<NodeId>) -> Option<()> {
        let node = self.get_node_mut(node_id)?;
        node.set_node_ptr(node_ptr);
        Some(())
    }
    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut BTreeNode> {
        self.nodes_pool.get_mut(node_id)
        
    }

    fn update_overflow_cells(&mut self, cell_records: &Vec<CellRecord>, prev_cell_record: &CellRecord) -> Option<(NodeId, CellId)> {
        let mut curr_cell_id = prev_cell_record.overflow_cell_ptr;
        let mut curr_node_id = prev_cell_record.node_ptr;
        let mut curr_node_record = self.records_pool.load_node_record_mut(&curr_node_id)?;
        for cell in cell_records {
            curr_node_record.cells[curr_cell_id as usize] = *cell;
            if cell.node_ptr != 0 && cell.has_overflow() && curr_node_id != cell.node_ptr {
                curr_node_record = self.records_pool.load_node_record_mut(&cell.node_ptr)?;
            } else if cell.node_ptr == 0 || !cell.has_overflow() {
                break;
            }
            curr_cell_id = cell.overflow_cell_ptr;
            curr_node_id = cell.node_ptr;
        }
        Some((curr_node_id, curr_cell_id))
    }

    fn create_overflow_cells(&mut self, cell_records: &mut [CellRecord], prev_cell_loc: (BTreeNodeId, CellId)) -> Option<BtreeCellLoc> {
        let mut prev_cell_loc = prev_cell_loc;
        for cell in cell_records {
            cell.chain_with_cell_location(prev_cell_loc);
            prev_cell_loc = self.records_pool.insert_cell_in_free_slot(cell)?;
        }
        Some(prev_cell_loc)
    }

    fn create_cell(&mut self, cell: &Cell) -> Option<Vec<CellRecord>> {
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
        
        if !cell.get_data_ptrs_ref().is_empty() {
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
            for cell_record in cell_records.iter_mut().skip(2) {
                cell_record.set_has_overflow();
            }

            let ptrs = self.create_overflow_cells(&mut cell_records[1..], (0, 0))?;

            let main_cell_record = cell_records.first_mut()?;
            main_cell_record.set_has_overflow();
            main_cell_record.chain_with_cell_location(ptrs);
        } else if let (Some(last_cell_record), Some(node_ptr)) = (cell_records.first_mut(), cell.get_node_ptr()) {
            last_cell_record.node_ptr = node_ptr;
        }

        Some(cell_records)
    }

    pub fn create(&mut self, node: &mut BTreeNode) -> Option<()> {
        let mut node_record = BNodeRecord::new();
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
        
        for (cell_id, cell) in node.get_cells_ref().iter().enumerate() {
            let cell_records = self.create_cell(cell)?;
            node_record.cells[cell_id] = *cell_records.first()?;
        }
        let id = self.records_pool.create_node_record(node_record)?;
        node.set_id(id);
        if node.is_root() {
            self.records_pool.set_root_node_ptr(id);
        }
        let mut clone = node.clone();
        clone.reset();
        self.nodes_pool.put(clone);
        self.records_pool.save_all_node_records()?;

        Some(())
    }

    fn load_overflow_cell_records(&mut self, root_cell_record: &CellRecord) -> Option<Vec<CellRecord>> {
        let mut cells = Vec::new();
        let mut curr_node_id = root_cell_record.node_ptr;
        let mut curr_overflow_cell_id = root_cell_record.overflow_cell_ptr;
        let mut has_overflow = root_cell_record.has_overflow();
        if has_overflow {
            let mut curr_node = self.records_pool.load_node_record_ref(curr_node_id)?;
            while has_overflow {
                let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
                has_overflow = overflow_cell.has_overflow();
                if has_overflow && curr_overflow_cell_id == overflow_cell.overflow_cell_ptr {
                    error!("cycle detected in node {} for cell {}", curr_node_id, curr_overflow_cell_id);
                    break;
                }
                let prev_node_id = curr_node_id;
                curr_node_id = overflow_cell.node_ptr;
                curr_overflow_cell_id = overflow_cell.overflow_cell_ptr;
                cells.push(*overflow_cell);
                if prev_node_id != curr_node_id && curr_node_id != 0 {
                    curr_node = self.records_pool.load_node_record_ref(curr_node_id)?;
                }
            }
        }
        
        Some(cells)
    }

    fn load_overflow_cell_records_head(&mut self, root_cell_record: &CellRecord) -> Option<CellRecord> {
        let curr_node_id = root_cell_record.node_ptr;
        let curr_overflow_cell_id = root_cell_record.overflow_cell_ptr;
        let has_overflow = root_cell_record.has_overflow();
        if has_overflow {
            let curr_node = self.records_pool.load_node_record_ref(curr_node_id)?;
            let overflow_cell = &curr_node.cells[curr_overflow_cell_id as usize];
            Some(*overflow_cell)
        } else {
            None
        }
    }

    fn update_cell_data_ptrs(&mut self, root_node_record_id: &NodeId, new_cell_id: usize, data_ptrs: &Vec<NodeId>, start_index: usize, is_append_only: bool) -> Option<()> {
        let root_cell_record = &self.records_pool.load_node_cell_record_clone(root_node_record_id, new_cell_id)?;
        let overflow_cell_records = if is_append_only {
            vec![self.load_overflow_cell_records_head(root_cell_record)?]
        } else {
            self.load_overflow_cell_records(root_cell_record)?
        };

        let mut list_ptr_cells = Vec::new();
        for cell_record in &overflow_cell_records {
            if cell_record.is_list_ptr() {
                list_ptr_cells.push(*cell_record);
            }
        }
        
        let mut cells_to_create = Vec::new();
        let cell_capacity = (KEY_SIZE - 2) / NODE_PTR_SIZE;
        let mut cells_to_update = vec![list_ptr_cells.pop()?];
        let mut curr_list_ptr_cell = cells_to_update.last_mut()?;
        let mut data_ptr_count: u16 = read_counter(&curr_list_ptr_cell.key);
        let mut data_ptr_offset = 2 + NODE_PTR_SIZE * data_ptr_count as usize ;

        for data_ptr in data_ptrs.iter().skip(start_index - 1) {
            if data_ptr_count >= cell_capacity as u16 {
                data_ptr_offset = 2;
                data_ptr_count = 0;
                if let Some(cell) = list_ptr_cells.pop() {
                    cells_to_update.push(cell);
                    curr_list_ptr_cell = cells_to_update.last_mut()?;
                } else {
                    let mut new_cell = CellRecord::new();
                    new_cell.set_is_active();
                    new_cell.set_is_list_ptr();
                    cells_to_create.push(new_cell);
                    curr_list_ptr_cell = cells_to_create.last_mut()?;
                }

            }
            data_ptr_offset += insert_data_ptr(&mut curr_list_ptr_cell.key, data_ptr_offset, data_ptr);
            data_ptr_count += 1;
            update_counter(&mut curr_list_ptr_cell.key, data_ptr_count);
        }
        let mut head_cell_pos = self.update_overflow_cells(&cells_to_update, root_cell_record)?;
        if !cells_to_create.is_empty() {
            head_cell_pos = self.create_overflow_cells(&mut cells_to_create, head_cell_pos)?;
        }

        //link last updated cell to created cells
        let root_cell_record = self.records_pool.load_node_cell_record_mut(root_node_record_id, new_cell_id)?;
        root_cell_record.chain_with_cell_location(head_cell_pos);
        //disable unused cells
        if !list_ptr_cells.is_empty() {
            self.records_pool.disable_cell_records(head_cell_pos);
        }
        
        Some(())
    }

    fn select_root_node(&mut self, node_id: &NodeId) -> Option<NodeId>  {
        let is_root = self.is_root_node(node_id);
        let main_node_record = self.records_pool.load_node_record_mut(node_id)?;
        if !is_root {
            main_node_record.set_is_not_root();
        } else {
            self.records_pool.set_root_node_ptr(*node_id);
        }
        Some(*node_id)
    }

    fn make_cells_change_log(&mut self, node_id: &NodeId, id: NodeId) -> Option<Vec<CellChangeContext>> {
        let node = self.get_node_clone(node_id)?;
        let mut cells_context = Vec::new();
        let list_old_ids_to_delete = {
            let main_node_record = self.records_pool.load_node_record_mut(node_id)?;
            for index in 0..main_node_record.cells.len() {
                if main_node_record.cells[index].is_active() {
                    cells_context.push(CellChangeContext::old(index));
                } else {
                    break;
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

            //delete old records
            for cell_id in &list_old_ids_to_delete {
                main_node_record.cells[*cell_id].set_inactive();
            }

            list_old_ids_to_delete
        };
        
        //delete old records
        for cell_id in list_old_ids_to_delete {
            self.records_pool.disable_cell_records((id, cell_id as u32));
        }

        Some(cells_context)
    }

    fn move_old_cell_records(&mut self, node_record_id: NodeId, cells_context: &[CellChangeContext]) -> Option<()> {
        let main_node_record = self.records_pool.load_node_record_mut(&node_record_id)?;
        let old_cell_records = main_node_record.cells;

        //move and update old records
        for (new_cell_id, ctx) in cells_context.iter().enumerate() {
            if !ctx.is_added && new_cell_id != ctx.old_cell_id {
                main_node_record.cells[new_cell_id] = old_cell_records[ctx.old_cell_id];
            }
        }
        Some(())
    }

    fn update_node_record_cells_data_ptr(&mut self, node_record_id: &NodeId, cells_context: &[CellChangeContext], node_id: &NodeId) -> Option<()> {
        let node = self.get_node_clone(node_id)?; 
        //move and update old records
        for (new_cell_id, ctx) in cells_context.iter().enumerate() {
            if !ctx.is_added {
                let current_cell = node.get_cell_ref(new_cell_id);
                if current_cell.get_change_state().did_list_data_ptr_changed() || current_cell.get_change_state().is_append_only() {      
                    let start_index = if current_cell.get_change_state().is_append_only() {
                        current_cell.get_change_state().append_index()
                    } else {
                        0
                    };
                    self.update_cell_data_ptrs(node_record_id, new_cell_id, current_cell.get_data_ptrs_ref(), start_index, current_cell.get_change_state().is_append_only())?;
                }
            }
        }
        Some(())
    }

    fn create_new_records(&mut self, node_record_id: NodeId, cells_context: &[CellChangeContext], node_id: &NodeId) -> Option<()> {   
        //create new records
        let node = self.get_node_clone(node_id)?;
        for (new_cell_id, ctx) in cells_context.iter().enumerate() {
            if ctx.is_added {
                let current_cell = node.get_cell_ref(new_cell_id);
                let cell_records = self.create_cell(current_cell)?;
                let main_node_record = self.records_pool.load_node_record_mut(&node_record_id)?;
                main_node_record.cells[new_cell_id] = cell_records[0];
            }
        }
        Some(())
    }

    pub fn save(&mut self, node_id: &NodeId) -> Option<()> {
        self._save(node_id);
        Some(())
    }

    pub fn _save(&mut self, node_id: &NodeId) -> Option<()> {
        
        let root_id = self.select_root_node(node_id)?;

        let cells_context = self.make_cells_change_log(node_id, root_id)?;

        self.move_old_cell_records(root_id, &cells_context)?;

        self.update_node_record_cells_data_ptr(&root_id, &cells_context, node_id)?;
        
        self.create_new_records(root_id, &cells_context, node_id)?;

        self.records_pool.save_all_node_records()?;

        if let Some(n) = self.get_node_mut(node_id) {
            n.reset();
        }

        Some(())
    }

    pub fn load_or_create_root_node(&mut self) -> Option<NodeId> {
        if self.is_empty() {
            let mut root = BTreeNode::new(true, true, Vec::new());
            self.create(&mut root)?;
            self.nodes_pool.put(root)
        } else {
            let root_node_id = self.records_pool.get_root_node_ptr();
            self.retrieve_node(&root_node_id);
            Some(root_node_id)
        }
        
    }

    pub fn is_empty(&mut self) -> bool {
        self.records_pool.is_empty()
    }

    pub fn sync(&mut self) {
        self.records_pool.sync();
        self.clear();
    }
    pub fn soft_sync(&mut self) {
        self.records_pool.sync();
    }
    pub fn clear(&mut self) {
        self.records_pool.clear();
        self.nodes_pool.clear();
    }
    pub fn erase(&mut self) {
        self.records_pool.erase();
        self.clear();
    }
}

struct BTreeNodePool {
    nodes_map: HashMap<u64, usize>,
    nodes: Vec<BTreeNode>,
}

impl BTreeNodePool {
    fn new() -> Self {
        BTreeNodePool { nodes_map: HashMap::new(), nodes: Vec::new() }
    }

    fn put(&mut self, node: BTreeNode) -> Option<NodeId>{
        let pos = self.nodes.len();
        if let Some(id) = node.get_id() {
            self.nodes_map.insert(id, pos);
            self.nodes.push(node);
            Some(id)
        } else {
            None
        }
    }
    
    fn get_mut(&mut self, id: &u64) -> Option<&mut BTreeNode> {
        self.nodes_map.get(id).and_then(|pos| self.nodes.get_mut(*pos))
    }

    fn get_ref(&self, id: &u64) -> Option<&BTreeNode> {
        self.nodes_map.get(id).and_then(|pos| self.nodes.get(*pos))
    }

    fn contains(&self, id: &NodeId) -> bool {
        self.nodes_map.contains_key(id)
    }
    
    fn get_clone(&mut self, id: &u64) -> Option<BTreeNode> {
        self.nodes_map.get(id).and_then(|pos| self.nodes.get(*pos).map(|n| n.clone()))
    }

    pub fn clear(&mut self) {
        self.nodes_map.clear();
        self.nodes.clear();
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
        let cells = vec![Cell::new_leaf("blabla1", 1), Cell::new_leaf("blabla2", 2), Cell::new_leaf("blabla3", 3),
                Cell::new_leaf("blabla4", 4), Cell::new_leaf("blabla5", 5), Cell::new_leaf(long_key, 6)];
        let mut node = BTreeNode::new(false, false, cells);
        node.set_node_ptr(Some(42));
        store.create(&mut node);
        store.sync();
        let id = node.get_id().unwrap();
        let mut load_store = BTreeNodeStore::new(&file);
        load_store.retrieve_node(&id);

        if let Some(loaded) = load_store.get_node_ref(&id) {
            assert_eq!(loaded.get_node_ptr(), Some(42));
            let cell = loaded.get_cell_ref(3);
            assert_eq!(cell.get_key(), &String::from("blabla4"));
            assert_eq!(cell.get_data_ptrs_ref(), &vec![4]);

            let long_key_cell = loaded.get_cell_ref(5);
            assert_eq!(long_key_cell.get_key(), &String::from(long_key));
            assert_eq!(long_key_cell.get_node_ptr(), None);

        } else {
            panic!("should not be empty");
        }

    }

    #[test]
    fn test_create_long_key() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_create_long_key.db").unwrap();
        let long_key = "blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6
        blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6blabla6";
        let mut store = BTreeNodeStore::new(&file);
        let cells = vec![Cell::new_leaf(long_key, 4)];
        let mut node = BTreeNode::new(false, false, cells);
        store.create(&mut node);
        store.sync();

        let mut load_store = BTreeNodeStore::new(&file);
        node.get_id().and_then(|id| load_store.retrieve_node(&id));

        if let Some(loaded) = node.get_id().and_then(|id| load_store.get_node_ref(&id)) {
            let long_key_cell = loaded.get_cell_ref(0);
            assert_eq!(long_key_cell.get_key(), &String::from(long_key));
        } else {
            panic!("should not be empty");
        }

    }


    #[test]
    fn test_many_ptrs() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_many_ptrs.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);
        let cells = vec![Cell::new("same key", None, vec![12, 98, 78667867867, 21, 9], true)];
        let mut node = BTreeNode::new(true, false, cells);
        store.create(&mut node);
        store.sync();
        node.insert_cell(1, Cell::new("same key", None, vec![12, 98, 78667867867, 21, 9, 12, 98, 78667867867, 21, 9], true));
        store.save(&node.get_id().unwrap()).unwrap();

        store.sync();

        node.get_id().and_then(|id| store.retrieve_node(&id)).unwrap();
        let loaded =  node.get_id().and_then(|id| store.get_node_ref(&id)).unwrap();
        assert_eq!(loaded.get_cell_ref(0).get_data_ptrs_ref().len(), 5);
        //assert_eq!(loaded.get_cell_ref(1).get_data_ptrs_ref().len(), 10);

    }

    #[test]
    fn test_many_ptrs_one_by_one() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_many_ptrs_one_by_one.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);
        let cells =  vec![Cell::new("same key", None, vec![12, 98, 77867867, 21, 9], true)];
        let mut node = BTreeNode::new(true, false, cells);
        store.create(&mut node);
        store.soft_sync();
        let id = node.get_id().unwrap();
        {
            let mref = store.get_node_mut(&id).unwrap();
            mref.insert_cell(1, Cell::new("same key", None, vec![12, 98, 78667867867, 21, 9, 12, 98, 78667867867, 21, 9], true));
            mref.insert_cell(2, Cell::new("same key", None, vec![12, 98, 78667867867, 21, 9, 12, 98], true));
            store.save(&id).unwrap();

            store.soft_sync();
        }
        for data_ptr in 0..100 {
            node.get_id().and_then(|id| store.retrieve_node(&id)).unwrap();
            let nid = node.get_id().unwrap();
            let loaded =  store.get_node_mut(&nid).unwrap();
            loaded.get_cell_mut(0).append_data_ptr(data_ptr);
            store.save(&nid).unwrap();
        }
        let loaded =  node.get_id().and_then(|id|{
            store.retrieve_node(&id);
            store.get_node_ref(&id)
        }).unwrap();
        assert_eq!(loaded.get_cell_ref(0).get_data_ptrs_ref().len(), 105);
        assert_eq!(loaded.get_cell_ref(1).get_data_ptrs_ref().len(), 10);
        assert_eq!(loaded.get_cell_ref(2).get_data_ptrs_ref().len(), 7);

    }

    #[test]
    fn test_update_ptrs() {
        let file = build_file_path_and_rm_old("b_tree_nodes", "test_update_ptrs.db").unwrap();
        let mut store = BTreeNodeStore::new(&file);

        let long_key = "blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3blabla3";
        let cells = vec![
            Cell::new_leaf("blabla1", 11),
            Cell::new_leaf("blabla2", 22),
            Cell::new_leaf(long_key, 33),
            Cell::new_leaf("blabla4", 44),
            Cell::new_leaf("blabla5", 55)];
        let mut node = BTreeNode::new(true, false, cells);

        store.create(&mut node);
        store.sync();

        let mut load_store = BTreeNodeStore::new(&file);
        node.get_id().and_then(|id| load_store.retrieve_node(&id));
        let load_id = node.get_id().unwrap();

        if let Some(load) = load_store.get_node_mut(&load_id) {
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

            load_store.save(&load_id);

        } else {
            panic!("should not be empty");
        }

        node.get_id().and_then(|id| load_store.retrieve_node(&id));
        if let Some(update) = &node.get_id().and_then(|id| load_store.get_node_mut(&id)) {
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
            panic!("should not be empty");
        }
        
    }

}

