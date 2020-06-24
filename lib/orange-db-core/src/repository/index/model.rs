use super::super::super::config::*;

pub type NodeId = u64;
pub type CellId = u32;

pub struct CellChangeState {
    added_data_ptrs: Vec<NodeId>,
    removed_data_ptrs: Vec<NodeId>,
    is_new_instance: bool,
    is_added: bool,
    is_removed: bool,
}

impl CellChangeState {
    fn new(new: bool) -> Self {
        CellChangeState{added_data_ptrs: Vec::new(), removed_data_ptrs: Vec::new(), is_new_instance: new, is_added: false, is_removed: false}
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
}

pub struct Cell {
    key: String,
    node_ptr: Option<NodeId>,
    is_active: bool,
    data_ptrs: Vec<NodeId>,
    cell_change_state: CellChangeState,
}

impl Cell {
    pub fn new_ptr(key: &str, ptr: Option<NodeId>) -> Self {
        Cell{key: String::from(key), node_ptr: ptr, is_active: true, data_ptrs: Vec::new(), cell_change_state: CellChangeState::new(true)}
    }
    pub fn new_leaf(key: &str, data_ptr: NodeId) -> Self {
        Cell{key: String::from(key), node_ptr: None, is_active: true, data_ptrs: vec![data_ptr], cell_change_state: CellChangeState::new(true)}
    }
    pub fn new(key: &str, ptr: Option<NodeId>, data_ptrs: Vec<NodeId>, is_active: bool) -> Self {
        Cell{key: String::from(key), node_ptr: ptr, is_active: is_active, data_ptrs: data_ptrs, cell_change_state: CellChangeState::new(false)}
    }
    pub fn append_data_ptr(&mut self, data_ptr: NodeId) {
        self.cell_change_state.added_data_ptrs.push(data_ptr);
        self.data_ptrs.push(data_ptr);
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

}

pub struct NodeChangeState {
    node_ptr_changed: bool,
    is_new_instance: bool,
}

impl NodeChangeState {
    fn new(is_new_instance: bool) -> Self {
        NodeChangeState{node_ptr_changed: false, is_new_instance: is_new_instance}
    }

    pub fn did_node_ptr_changed(&self) -> bool {
        self.node_ptr_changed
    }
    pub fn is_new_instance(&self) -> bool {
        self.is_new_instance
    }
}

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
        BTreeNode{id: None, cells: cells, node_ptr: None, is_leaf: is_leaf, is_root: is_root, node_change_state: NodeChangeState::new(true)}
    }

    pub fn new_with_id(id: Option<NodeId>, node_ptr: Option<NodeId>, is_leaf: bool, is_root: bool, cells: Vec<Cell>) -> Self {
        BTreeNode{id: id, cells: cells, node_ptr: node_ptr, is_leaf: is_leaf, is_root: is_root, node_change_state: NodeChangeState::new(false)}
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

    pub fn get_cells_ref(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell_ref(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    pub fn insert_cell(&mut self, index: usize, mut cell: Cell) {
        cell.cell_change_state.set_is_added();
        self.cells.insert(index, cell);
    }

    pub fn pop_cell(&mut self) -> Option<Cell> {
        let mut cell = self.cells.pop()?;
        cell.cell_change_state.set_is_removed();
        Some(cell)
    }

    pub fn get_cell_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    

    pub fn set_root(&mut self) {
        self.is_root = true;
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
}