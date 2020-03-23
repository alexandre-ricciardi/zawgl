pub struct NodeRecord {
    pub in_use: bool,
    pub next_rel_id: u32,
    pub next_prop_id: u32,
}

fn to_bytes(val: u32) -> [u8; 4] {
    let mut b: [u8; 4] = [0; 4];
    let be = val.to_be();
    b[0] = (be >> 24) as u8;
    b[1] = (be >> 16) as u8;
    b[2] = (be >> 8) as u8;
    b[3] = be as u8;
    b
}

fn from_bytes(b: &[u8]) -> u32 {
    let mut res = 0u32;
    res += (b[0] as u32) << 24 | (b[1] as u32) << 16 | (b[2] as u32) << 8 | (b[3] as u32) ;
    u32::from_be(res)
}

pub fn nr_to_bytes(nr: NodeRecord) -> [u8; 9] {
    let mut bytes: [u8; 9] = [0; 9];
    if nr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..5].clone_from_slice(&to_bytes(nr.next_rel_id));
    bytes[5..9].clone_from_slice(&to_bytes(nr.next_prop_id));
    bytes
}

pub fn nr_from_bytes(bytes: [u8; 9]) -> NodeRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let rel_id = from_bytes(&bytes[1..]);
    let prop_id = from_bytes(&bytes[5..]);
    NodeRecord {in_use: in_use, next_rel_id: rel_id, next_prop_id: prop_id}
}

pub fn rr_to_bytes(rr: RelationshipRecord) -> [u8; 35] {
    let mut bytes: [u8; 35] = [0; 35];
    if rr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..5].clone_from_slice(&to_bytes(rr.first_node));
    bytes[5..9].clone_from_slice(&to_bytes(rr.second_node));
    bytes[9..13].clone_from_slice(&to_bytes(rr.relationship_type));
    bytes[13..17].clone_from_slice(&to_bytes(rr.first_prev_rel_id));
    bytes[17..21].clone_from_slice(&to_bytes(rr.first_next_rel_id));
    bytes[23..27].clone_from_slice(&to_bytes(rr.second_prev_rel_id));
    bytes[27..31].clone_from_slice(&to_bytes(rr.second_next_rel_id));
    bytes[31..35].clone_from_slice(&to_bytes(rr.next_prop_id));
    bytes
}

pub fn rr_from_bytes(bytes: [u8; 35]) -> RelationshipRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let f_node = from_bytes(&bytes[1..]);
    let s_node = from_bytes(&bytes[5..]);
    let rt = from_bytes(&bytes[9..]);
    let fp_rel = from_bytes(&bytes[13..]);
    let fn_rel = from_bytes(&bytes[17..]);
    let sp_rel = from_bytes(&bytes[23..]);
    let sn_rel = from_bytes(&bytes[27..]);
    let p = from_bytes(&bytes[31..]);
    RelationshipRecord {in_use: in_use, first_node: f_node, second_node: s_node,
        relationship_type: rt, first_prev_rel_id: fp_rel, first_next_rel_id: fn_rel,
        second_prev_rel_id: sp_rel, second_next_rel_id: sn_rel, next_prop_id: p}
}

pub struct RelationshipRecord {
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

#[cfg(test)]
mod test_records {
    use super::*;
    #[test]
    fn test_u32() {
        let val = 0b1011_1010_1111_0001_1000_1010_1111_0001;
        let bytes = to_bytes(val);
        assert_eq!(from_bytes(&bytes), val);
    }
    #[test]
    fn test_node_record() {
        let val = NodeRecord {in_use: true, next_prop_id: 100, next_rel_id: 32};
        let bytes = nr_to_bytes(val);
        let nr = nr_from_bytes(bytes);
        assert_eq!(nr.in_use, true);
        assert_eq!(nr.next_rel_id, 32);
        assert_eq!(nr.next_prop_id, 100);
    }

    
    #[test]
    fn test_relationship_record() {
        let val = RelationshipRecord {in_use: true, first_node: 2, second_node: 3,
            first_prev_rel_id: 4, first_next_rel_id: 5, second_prev_rel_id: 6, second_next_rel_id: 7,
            relationship_type: 33, next_prop_id: 100};
        let bytes = rr_to_bytes(val);
        let rr = rr_from_bytes(bytes);
        assert_eq!(rr.in_use, true);
        assert_eq!(rr.first_node, 2);
        assert_eq!(rr.second_node, 3);
        assert_eq!(rr.first_prev_rel_id, 4);
        assert_eq!(rr.first_next_rel_id, 5);
        assert_eq!(rr.second_prev_rel_id, 6);
        assert_eq!(rr.second_next_rel_id, 7);
        assert_eq!(rr.relationship_type, 33);
        assert_eq!(rr.next_prop_id, 100);
    }
}