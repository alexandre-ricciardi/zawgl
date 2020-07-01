//PAGING
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_COUNTER_SIZE: usize = 8;
pub const FIRST_FREE_PAGE_PTR: usize = 8;
//RECORDS
pub const RECORDS_COUNTER_SIZE: usize = 8;
pub const FREE_LIST_PTR_SIZE: usize = 4;
pub const FREE_LIST_ITEM_COUNTER_SIZE: usize = 4;
pub const NEXT_PAGE_PTR: usize = 8;
pub const NEXT_FREE_PAGE_PTR: usize = 8;
pub const HEADER_FLAGS: usize = 1;
pub const HEADER_SIZE: usize = 21;
//BTREE
//PAGE PAYLOAD SIZE 4071 BYTES
//UNUSED SPACE 226 BYTES
pub const NB_CELL: usize = 66;
pub const NODE_PTR_SIZE: usize = 8;
pub const KEY_SIZE: usize = 45;
pub const CELL_HEADER_SIZE: usize = 1;
pub const FREE_CELLS_NEXT_NODE_PTR_SIZE: usize = 8;
pub const CELL_SIZE: usize = 58;
pub const BTREE_NODE_RECORD_SIZE: usize = 3845;
pub const OVERFLOW_CELL_PTR_SIZE: usize = 4;
pub const BTREE_NODE_HEADER_SIZE: usize = 1;
pub const BTREE_NB_RECORDS_PER_PAGE: usize = 1;
pub const BTREE_NB_PAGES_PER_RECORD: usize = 0;
//NODES
//PAGE PAYLOAD SIZE 3299 BYTES
//UNUSED SPACE 1 BYTES
pub const NODE_HEADER_SIZE: usize = 1;
pub const NODE_ID_SIZE: usize = 8;
pub const NODE_RECORD_SIZE: usize = 17;
pub const NODE_NB_RECORDS_PER_PAGE: usize = 194;
pub const NODE_NB_PAGES_PER_RECORD: usize = 0;
//RELATIONSHIPS
//PAGE PAYLOAD SIZE 3839 BYTES
//UNUSED SPACE 4 BYTES
pub const RELATIONSHIP_HEADER_SIZE: usize = 1;
pub const RELATIONSHIP_ID_SIZE: usize = 8;
pub const RELATIONSHIP_RECORD_SIZE: usize = 65;
pub const RELATIONSHIP_NB_RECORDS_PER_PAGE: usize = 59;
pub const RELATIONSHIP_NB_PAGES_PER_RECORD: usize = 0;
//PROPERTIES
//PAGE PAYLOAD SIZE 3723 BYTES
//UNUSED SPACE 0 BYTES
pub const PROPERTY_HEADER_SIZE: usize = 1;
pub const PROPERTY_ID_SIZE: usize = 8;
pub const PROPERTY_BLOCK_SIZE: usize = 24;
pub const PROPERTY_RECORD_SIZE: usize = 42;
pub const PROPERTY_NB_RECORDS_PER_PAGE: usize = 88;
pub const PROPERTY_NB_PAGES_PER_RECORD: usize = 0;
//DYN STORE
//PAGE PAYLOAD SIZE 3955 BYTES
//UNUSED SPACE 85 BYTES
pub const DYN_HEADER_SIZE: usize = 1;
pub const DYN_ID_SIZE: usize = 8;
pub const DYN_PAYLOAD_SIZE: usize = 120;
pub const DYN_RECORD_SIZE: usize = 129;
pub const DYN_NB_RECORDS_PER_PAGE: usize = 30;
pub const DYN_NB_PAGES_PER_RECORD: usize = 0;
