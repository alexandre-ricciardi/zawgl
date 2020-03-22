pub struct NodeRecord {
    in_use: bool,
    next_rel_id: u32,
    next_prop_id: u32,
}

impl NodeRecord {
    pub fn to_bytes(&self) -> [u8; 9] {
        let flags = 0u8;
        if self.in_use {
            flags = flags | 0b00000001;    
        }
    }
}

impl From<[u8; 9]> for NodeRecord {
    fn from(bytes: [u8; 9]) -> Self {
        let in_use = bytes[0] & 0b00000001;
        let brel_id = &bytes[1..5];
        let mut rel_id = 0u32;
        rel_id += brel_id[0] as u32;
        rel_id += (brel_id[1] as u32) << 4;
        rel_id += brel_id[2] as u32 << 4;

        NodeRecord {in_use: in_use, next_rel_id: }
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