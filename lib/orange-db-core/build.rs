use std::io::{Write, Result};

///PAGING
const PAGE_SIZE: usize = 4096;
const PAGE_COUNTER_SIZE: usize = 8;
const FIRST_FREE_PAGE_PTR: usize = 8;

///RECORDS
const RECORDS_COUNTER_SIZE: usize = 8;
const FREE_LIST_PTR_SIZE: usize = 4;
const FREE_LIST_ITEM_COUNTER_SIZE: usize = 4;
const NEXT_PAGE_PTR: usize = 8;
const NEXT_FREE_PAGE_PTR: usize = 8;
const HEADER_FLAGS: usize = 1; 
const HEADER_SIZE: usize = HEADER_FLAGS + NEXT_FREE_PAGE_PTR + NEXT_PAGE_PTR + FREE_LIST_ITEM_COUNTER_SIZE;

///B+TREE
const NB_CELL: usize = 66;
const NODE_PTR_SIZE: usize = 8;
const KEY_SIZE: usize = 45;
const CELL_HEADER_SIZE: usize = 1;
const BTREE_NODE_HEADER_SIZE: usize = 1;
const FREE_CELLS_NEXT_NODE_PTR_SIZE: usize = 8;
const CELL_SIZE: usize = KEY_SIZE + NODE_PTR_SIZE + CELL_HEADER_SIZE + OVERFLOW_CELL_PTR_SIZE;
const BTREE_NODE_RECORD_SIZE: usize = BTREE_NODE_HEADER_SIZE + CELL_SIZE * NB_CELL + NODE_PTR_SIZE + FREE_CELLS_NEXT_NODE_PTR_SIZE;
const OVERFLOW_CELL_PTR_SIZE: usize = 4;
const OVERFLOW_KEY_SIZE: usize = CELL_SIZE - OVERFLOW_CELL_PTR_SIZE;

const fn max_nb_records(record_size: usize) -> usize {
    (PAGE_SIZE - HEADER_SIZE) / record_size
}

const fn compute_unused_page_size(record_size: usize, nb_records: usize) -> usize {
    PAGE_SIZE - HEADER_SIZE - nb_records * record_size
}
const fn compute_freelist_size(free_list_capacity: usize) -> usize {
    FREE_LIST_PTR_SIZE * free_list_capacity
}

fn compute_nb_records_per_page(record_size: usize) -> usize {
    let mut nb_records = max_nb_records(record_size);
    let mut unused_space = compute_unused_page_size(record_size, nb_records);
    let mut free_list_size = compute_freelist_size(nb_records);
    while unused_space < free_list_size {
        nb_records -= 1;
        if nb_records == 0 {
            break;
        }
        unused_space = compute_unused_page_size(record_size, nb_records);
        free_list_size = compute_freelist_size(nb_records);
    }
    nb_records
}

fn compute_nb_pages_per_record(record_size: usize) -> usize {
    let page_payload_size = PAGE_SIZE - HEADER_SIZE;
    let nb_pages = record_size / page_payload_size;
    if page_payload_size == record_size {
        1
    } else if nb_pages > 0 {
        nb_pages + 1
    } else {
        nb_pages
    }
}

fn compute_page_free_space_size(record_size: usize, nb_records_per_page: usize, nb_pages_per_records: usize) -> usize {
    let page_payload_size = PAGE_SIZE - HEADER_SIZE;
    if nb_pages_per_records > 0 {
        (nb_pages_per_records * page_payload_size) % record_size 
    } else {
        let free_list_size = compute_freelist_size(nb_records_per_page);
        (page_payload_size - free_list_size) % record_size
    }
}

fn compute_page_payload_size(nb_records_per_page: usize) -> usize {
    let page_payload_size = PAGE_SIZE - HEADER_SIZE;
    let free_list_size = compute_freelist_size(nb_records_per_page);
    page_payload_size - free_list_size
}

fn generate_config() -> Result<()> {
    let mut config = std::fs::File::create("src/config.rs")?;
    writeln!(config, "//PAGING")?;
    writeln!(config, "pub const PAGE_SIZE: usize = {};", PAGE_SIZE)?;
    writeln!(config, "pub const PAGE_COUNTER_SIZE: usize = {};", PAGE_COUNTER_SIZE)?;
    writeln!(config, "pub const FIRST_FREE_PAGE_PTR: usize = {};", FIRST_FREE_PAGE_PTR)?;
    
    writeln!(config, "//RECORDS")?;    
    writeln!(config, "pub const RECORDS_COUNTER_SIZE: usize = {};", RECORDS_COUNTER_SIZE)?;
    writeln!(config, "pub const FREE_LIST_PTR_SIZE: usize = {};", FREE_LIST_PTR_SIZE)?;
    writeln!(config, "pub const FREE_LIST_ITEM_COUNTER_SIZE: usize = {};", FREE_LIST_ITEM_COUNTER_SIZE)?;
    writeln!(config, "pub const NEXT_PAGE_PTR: usize = {};", NEXT_PAGE_PTR)?;
    writeln!(config, "pub const NEXT_FREE_PAGE_PTR: usize = {};", NEXT_FREE_PAGE_PTR)?;
    writeln!(config, "pub const HEADER_FLAGS: usize = {};", HEADER_FLAGS)?;
    writeln!(config, "pub const HEADER_SIZE: usize = {};", HEADER_SIZE)?;

    let nb_records_per_page = compute_nb_records_per_page(BTREE_NODE_RECORD_SIZE);
    let nb_pages_per_record = compute_nb_pages_per_record(BTREE_NODE_RECORD_SIZE);
    writeln!(config, "//BTREE")?;
    writeln!(config, "//PAGE PAYLOAD SIZE {} BYTES", compute_page_payload_size(nb_records_per_page))?;
    writeln!(config, "//UNUSED SPACE {} BYTES", compute_page_free_space_size(BTREE_NODE_RECORD_SIZE, nb_records_per_page, nb_pages_per_record))?;
    writeln!(config, "pub const NB_CELL: usize = {};", NB_CELL)?;
    writeln!(config, "pub const NODE_PTR_SIZE: usize = {};", NODE_PTR_SIZE)?;
    writeln!(config, "pub const KEY_SIZE: usize = {};", KEY_SIZE)?;
    writeln!(config, "pub const CELL_HEADER_SIZE: usize = {};", CELL_HEADER_SIZE)?;
    writeln!(config, "pub const FREE_CELLS_NEXT_NODE_PTR_SIZE: usize = {};", FREE_CELLS_NEXT_NODE_PTR_SIZE)?;
    writeln!(config, "pub const CELL_SIZE: usize = {};", CELL_SIZE)?;
    writeln!(config, "pub const BTREE_NODE_RECORD_SIZE: usize = {};", BTREE_NODE_RECORD_SIZE)?;
    writeln!(config, "pub const OVERFLOW_CELL_PTR_SIZE: usize = {};", OVERFLOW_CELL_PTR_SIZE)?;
    writeln!(config, "pub const BTREE_NODE_HEADER_SIZE: usize = {};", BTREE_NODE_HEADER_SIZE)?;
    writeln!(config, "pub const BTREE_NB_RECORDS_PER_PAGE: usize = {};", nb_records_per_page)?;
    writeln!(config, "pub const BTREE_NB_PAGES_PER_RECORD: usize = {};", nb_pages_per_record)?;
    Ok(())
}

fn main() {
    if let Err(e) = generate_config() {
        eprintln!("Error: {}", e);
    }
}
