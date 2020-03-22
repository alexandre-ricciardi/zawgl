struct NodeRecord {
    in_use: bool,
    next_rel_id: u32,
    next_prop_id: u32,
}

impl NodeRecord {
    pub fn to_bytes() -> [u8; 9] {
        
    }
}

struct RelationshipRecord {
    in_use: bool,
    first_node: u32,
    second_node: u32,
    relationship_type: u32,
    first_prev_rel_id: u32,
    first_next_rel_id: u32,
    second_prev_rel_id: u32,
    second_next_rel_id: u32,
    next_prop_id: u32,
}