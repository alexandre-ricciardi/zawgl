///PAGING
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_COUNTER: usize = 8;
pub const FIRST_FREE_PAGE_PTR: usize = 8;
///RECORDS
pub const FREE_LIST_PTR_SIZE: usize = 4;
pub const FREE_LIST_ITEM_COUNTER_SIZE: usize = 4;
pub const NEXT_PAGE_PTR: usize = 8;
pub const NEXT_FREE_PAGE_PTR: usize = 8;
pub const HEADER_FLAGS: usize = 1;
pub const HEADER_SIZE: usize = 21;
///BTREE
///PAGE PAYLOAD SIZE 4071 BYTES
///UNUSED SPACE 37 BYTES
pub const NB_CELL: usize = 66;
pub const NODE_PTR_SIZE: usize = 8;
pub const KEY_SIZE: usize = 44;
pub const CELL_HEADER_SIZE: usize = 1;
pub const ACTIVE_CELLS_COUNTER: usize = 4;
pub const CELL_SIZE: usize = 61;
pub const BTREE_NODE_RECORD_SIZE: usize = 4034;
pub const OVERFLOW_CELL_PTR_SIZE: usize = 4;
pub const OVERFLOW_KEY_SIZE: usize = 57;
pub const BTREE_NB_RECORDS_PER_PAGE: usize = 1;
pub const BTREE_NB_PAGES_PER_RECORD: usize = 0;
