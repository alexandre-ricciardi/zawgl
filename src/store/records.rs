pub struct NodeRecord {
    pub in_use: bool,
    pub next_rel_id: u64,
    pub next_prop_id: u64,
}

fn u64_to_bytes(val: u64) -> [u8; 8] {
    let mut b: [u8; 8] = [0; 8];
    let be = val.to_be();
    let mut shift = 58;
    let mut index = 0;
    while index < 8 {
        b[index] = (be >> shift) as u8;
        index += 1;
        shift -= 8;
    }
    b
}

fn u64_from_bytes(b: &[u8]) -> u64 {
    let mut res = 0u64;
    let mut shift = 58;
    let mut index = 0;
    while index < 8 {
        res += (b[index] as u64) << shift;
        shift -= 8;
        index += 1;
    }
    u64::from_be(res)
}

pub fn nr_to_bytes(nr: NodeRecord) -> [u8; 17] {
    let mut bytes: [u8; 17] = [0; 17];
    if nr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..9].clone_from_slice(&u64_to_bytes(nr.next_rel_id));
    bytes[9..17].clone_from_slice(&u64_to_bytes(nr.next_prop_id));
    bytes
}

pub fn nr_from_bytes(bytes: [u8; 17]) -> NodeRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let rel_id = u64_from_bytes(&bytes[1..]);
    let prop_id = u64_from_bytes(&bytes[9..]);
    NodeRecord {in_use: in_use, next_rel_id: rel_id, next_prop_id: prop_id}
}

pub fn rr_to_bytes(rr: RelationshipRecord) -> [u8; 65] {
    let mut bytes: [u8; 65] = [0; 65];
    if rr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..9].clone_from_slice(&u64_to_bytes(rr.first_node));
    bytes[9..17].clone_from_slice(&u64_to_bytes(rr.second_node));
    bytes[17..25].clone_from_slice(&u64_to_bytes(rr.relationship_type));
    bytes[25..33].clone_from_slice(&u64_to_bytes(rr.first_prev_rel_id));
    bytes[33..41].clone_from_slice(&u64_to_bytes(rr.first_next_rel_id));
    bytes[41..49].clone_from_slice(&u64_to_bytes(rr.second_prev_rel_id));
    bytes[49..57].clone_from_slice(&u64_to_bytes(rr.second_next_rel_id));
    bytes[57..65].clone_from_slice(&u64_to_bytes(rr.next_prop_id));
    bytes
}

pub fn rr_from_bytes(bytes: [u8; 65]) -> RelationshipRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let f_node = u64_from_bytes(&bytes[1..]);
    let s_node = u64_from_bytes(&bytes[9..]);
    let rt = u64_from_bytes(&bytes[17..]);
    let fp_rel = u64_from_bytes(&bytes[25..]);
    let fn_rel = u64_from_bytes(&bytes[33..]);
    let sp_rel = u64_from_bytes(&bytes[41..]);
    let sn_rel = u64_from_bytes(&bytes[49..]);
    let p = u64_from_bytes(&bytes[57..]);
    RelationshipRecord {in_use: in_use, first_node: f_node, second_node: s_node,
        relationship_type: rt, first_prev_rel_id: fp_rel, first_next_rel_id: fn_rel,
        second_prev_rel_id: sp_rel, second_next_rel_id: sn_rel, next_prop_id: p}
}

pub struct RelationshipRecord {
    pub in_use: bool,
    pub first_node: u64,
    pub second_node: u64,
    pub relationship_type: u64,
    pub first_prev_rel_id: u64,
    pub first_next_rel_id: u64,
    pub second_prev_rel_id: u64,
    pub second_next_rel_id: u64,
    pub next_prop_id: u64,
}

pub struct PropertyRecord {
    pub in_use: bool,
    pub prop_type: u8,
    pub key_id: u64,
    pub prop_block: [u8; 24],
    pub next_prop_id: u64,
}

pub struct DynamicStoreRecord {
    pub in_use: bool,
    pub next: u64,
    pub data: [u8; 120]
}


pub fn dr_to_bytes(dr: DynamicStoreRecord) -> [u8; 129] {
    let mut bytes: [u8; 129] = [0; 129];
    if dr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1..9].clone_from_slice(&u64_to_bytes(dr.next));
    bytes[9..129].clone_from_slice(&dr.data);
    bytes
}

pub fn dr_from_bytes(bytes: [u8; 129]) -> DynamicStoreRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let next = u64_from_bytes(&bytes[1..]);
    let mut data = [0u8; 120];
    data.copy_from_slice(&bytes[9..]);
    DynamicStoreRecord {in_use: in_use, next: next, data: data}
}

pub fn pr_to_bytes(pr: PropertyRecord) -> [u8; 42] {
    let mut bytes: [u8; 42] = [0; 42];
    if pr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    bytes[1] = pr.prop_type;
    bytes[2..10].copy_from_slice(&u64_to_bytes(pr.key_id));
    bytes[10..34].copy_from_slice(&pr.prop_block);
    bytes[34..42].copy_from_slice(&u64_to_bytes(pr.next_prop_id));
    bytes
}

pub fn pr_from_bytes(bytes: [u8; 42]) -> PropertyRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let ptype = bytes[1];
    let key = u64_from_bytes(&bytes[2..]);
    let mut block = [0u8; 24];
    block.copy_from_slice(&bytes[10..]);
    let next = u64_from_bytes(&bytes[34..]);
    let mut data = [0u8; 24];
    data.copy_from_slice(&bytes[9..]);
    PropertyRecord {in_use: in_use, prop_type: ptype, key_id: key, prop_block: block, next_prop_id: next}
}


#[cfg(test)]
mod test_records {
    use super::*;
    #[test]
    fn test_u64() {
        let val = 977856654765u64;
        let bytes = u64_to_bytes(val);
        assert_eq!(u64_from_bytes(&bytes), val);
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