pub struct NodeRecord {
    in_use: bool,
    next_rel_id: u32,
    next_prop_id: u32,
}

impl NodeRecord {
    
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

fn nr_to_bytes(nr: NodeRecord) -> [u8; 9] {
    let mut bytes: [u8; 9] = [0; 9];
    if nr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..5].clone_from_slice(&to_bytes(nr.next_rel_id));
    bytes[5..9].clone_from_slice(&to_bytes(nr.next_prop_id));
    bytes
}

fn nr_from_bytes(bytes: [u8; 9]) -> NodeRecord {
    let in_use = bytes[0] & 0b00000001 > 0;
    let rel_id = from_bytes(&bytes[1..5]);
    let prop_id = from_bytes(&bytes[5..9]);
    NodeRecord {in_use: in_use, next_rel_id: rel_id, next_prop_id: prop_id}
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
}