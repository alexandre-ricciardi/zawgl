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

use super::super::byte_utils::*;
use super::super::super::buf_config::*;

pub struct NodeRecord {
    pub first_outbound_edge: u64,
    pub first_inbound_edge: u64,
    pub next_prop_id: u64,
    pub node_type: u64,
}

impl NodeRecord {
    pub fn new() -> Self {
        NodeRecord{first_outbound_edge: 0, first_inbound_edge: 0, next_prop_id: 0, node_type: 0}
    }

    pub fn to_bytes(&self) -> [u8; NODE_RECORD_SIZE] {
        let mut bytes: [u8; NODE_RECORD_SIZE] = [0; NODE_RECORD_SIZE];
        let mut offset = 0;
        bytes[offset..offset+RELATIONSHIP_ID_SIZE].copy_from_slice(&u64_to_bytes(self.first_outbound_edge));
        offset += RELATIONSHIP_ID_SIZE;
        bytes[offset..offset+RELATIONSHIP_ID_SIZE].copy_from_slice(&u64_to_bytes(self.first_inbound_edge));
        offset += RELATIONSHIP_ID_SIZE;
        bytes[offset..offset+PROPERTY_ID_SIZE].copy_from_slice(&u64_to_bytes(self.next_prop_id));
        offset += PROPERTY_ID_SIZE;
        bytes[offset..offset+NODE_TYPE_SIZE].copy_from_slice(&u64_to_bytes(self.node_type));
        bytes
    }

    pub fn from_bytes(bytes: [u8; NODE_RECORD_SIZE]) -> NodeRecord {
        let mut offset = 0;
        let out_rel_id = u64_from_bytes(&bytes[offset..offset+RELATIONSHIP_ID_SIZE]);
        offset += RELATIONSHIP_ID_SIZE;
        let in_rel_id = u64_from_bytes(&bytes[offset..offset+RELATIONSHIP_ID_SIZE]);
        offset += RELATIONSHIP_ID_SIZE;
        let prop_id = u64_from_bytes(&bytes[offset..offset+PROPERTY_ID_SIZE]);
        offset += NODE_TYPE_SIZE;
        let node_type = u64_from_bytes(&bytes[offset..offset+NODE_TYPE_SIZE]);
        NodeRecord {first_outbound_edge: out_rel_id, first_inbound_edge: in_rel_id,
            next_prop_id: prop_id, node_type}
    }
}

pub struct RelationshipRecord {
    pub source: u64,
    pub target: u64,
    pub relationship_type: u64,
    pub next_outbound_edge: u64,
    pub next_inbound_edge: u64,
    pub next_prop_id: u64,
}

impl RelationshipRecord {
    pub fn new(first_node: u64, second_node: u64) -> Self {
        RelationshipRecord{source: first_node, target: second_node, relationship_type: 0, next_outbound_edge: 0,
            next_inbound_edge: 0, next_prop_id: 0}
    }

    pub fn to_bytes(&self) -> [u8; RELATIONSHIP_RECORD_SIZE] {
        let mut bytes: [u8; RELATIONSHIP_RECORD_SIZE] = [0; RELATIONSHIP_RECORD_SIZE];
        let mut offset = 0;
        bytes[offset..offset+NODE_ID_SIZE].clone_from_slice(&u64_to_bytes(self.source));
        offset += NODE_ID_SIZE;
        bytes[offset..offset+NODE_ID_SIZE].clone_from_slice(&u64_to_bytes(self.target));
        offset += NODE_ID_SIZE;
        bytes[offset..offset+RELATIONSHIP_TYPE_SIZE].clone_from_slice(&u64_to_bytes(self.relationship_type));        
        offset += RELATIONSHIP_TYPE_SIZE;
        bytes[offset..offset+RELATIONSHIP_ID_SIZE].clone_from_slice(&u64_to_bytes(self.next_outbound_edge));
        offset += RELATIONSHIP_ID_SIZE;
        bytes[offset..offset+RELATIONSHIP_ID_SIZE].clone_from_slice(&u64_to_bytes(self.next_inbound_edge));
        offset += RELATIONSHIP_ID_SIZE;
        bytes[offset..offset+PROPERTY_ID_SIZE].clone_from_slice(&u64_to_bytes(self.next_prop_id));
        bytes
    }
    
    pub fn from_bytes(bytes: [u8; RELATIONSHIP_RECORD_SIZE]) -> RelationshipRecord {
        let mut offset = 0;
        let s = u64_from_bytes(&bytes[offset..offset+NODE_ID_SIZE]);
        offset += NODE_ID_SIZE;
        let t = u64_from_bytes(&bytes[offset..offset+NODE_ID_SIZE]);
        offset += NODE_ID_SIZE;
        let rt = u64_from_bytes(&bytes[offset..offset+RELATIONSHIP_TYPE_SIZE]);
        offset += RELATIONSHIP_TYPE_SIZE;
        let out_rel = u64_from_bytes(&bytes[offset..offset+RELATIONSHIP_ID_SIZE]);
        offset += RELATIONSHIP_ID_SIZE;
        let in_rel = u64_from_bytes(&bytes[offset..offset+RELATIONSHIP_ID_SIZE]);
        offset += RELATIONSHIP_ID_SIZE;
        let p = u64_from_bytes(&bytes[offset..offset+PROPERTY_ID_SIZE]);
        RelationshipRecord {source: s, target: t,
            relationship_type: rt, next_outbound_edge: out_rel, next_inbound_edge: in_rel,
            next_prop_id: p}
    }
}

pub struct PropertyRecord {
    pub in_use: bool,
    pub key_inlined: bool,
    pub full_inlined: bool,
    pub has_next: bool,
    pub prop_type: u8,
    pub key_id: u64,
    pub prop_block: [u8; 24],
    pub next_prop_id: u64,
}

pub struct DynamicStoreRecord {
    pub in_use: bool,
    pub has_next: bool,
    pub next: u64,
    pub data: [u8; 120]
}


pub fn dr_to_bytes(dr: &DynamicStoreRecord) -> [u8; 129] {
    let mut bytes: [u8; 129] = [0; 129];
    if dr.in_use {
        bytes[0] = bytes[0] | 0b00000001;
    }
    if dr.has_next {
        bytes[0] = bytes[0] | 0b0000_1000;
    }
    bytes[1..9].clone_from_slice(&u64_to_bytes(dr.next));
    bytes[9..129].clone_from_slice(&dr.data);
    bytes
}

pub fn dr_from_bytes(bytes: [u8; 129]) -> DynamicStoreRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let has_next = bytes[0] & 0b0000_1000 > 0;
    let next = u64_from_bytes(&bytes[1..9]);
    let mut data = [0u8; 120];
    data.copy_from_slice(&bytes[9..129]);
    DynamicStoreRecord {in_use, has_next, next, data}
}

pub fn pr_to_bytes(pr: &PropertyRecord) -> [u8; 42] {
    let mut bytes: [u8; 42] = [0; 42];
    if pr.in_use {
        bytes[0] = bytes[0] | 0b0000_0001;
    }
    if pr.full_inlined {
        bytes[0] = bytes[0] | 0b0000_0010;
    }
    if pr.key_inlined {
        bytes[0] = bytes[0] | 0b0000_0100;
    }
    if pr.has_next {
        bytes[0] = bytes[0] | 0b0000_1000;
    }
    bytes[1] = pr.prop_type;
    bytes[2..10].copy_from_slice(&u64_to_bytes(pr.key_id));
    bytes[10..34].copy_from_slice(&pr.prop_block);
    bytes[34..42].copy_from_slice(&u64_to_bytes(pr.next_prop_id));
    bytes
}

pub fn pr_from_bytes(bytes: [u8; 42]) -> PropertyRecord {
    let in_use = bytes[0] & 0b0000_0001 > 0;
    let inlined = bytes[0] & 0b0000_0010 > 0;
    let key_inlined = bytes[0] & 0b0000_0100 > 0;
    let has_next = bytes[0] & 0b0000_1000 > 0;
    let ptype = bytes[1];
    let key = u64_from_bytes(&bytes[2..10]);
    let mut block = [0u8; 24];
    block.copy_from_slice(&bytes[10..34]);
    let next = u64_from_bytes(&bytes[34..42]);
    PropertyRecord {in_use, full_inlined: inlined, key_inlined, has_next, prop_type: ptype, key_id: key, prop_block: block, next_prop_id: next}
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
        let val = NodeRecord {next_prop_id: 100, first_inbound_edge: 32, first_outbound_edge: 55, node_type: 4};
        let bytes = val.to_bytes();
        let nr = NodeRecord::from_bytes(bytes);
        assert_eq!(nr.first_outbound_edge, 55);
        assert_eq!(nr.first_inbound_edge, 32);
        assert_eq!(nr.next_prop_id, 100u64);
        assert_eq!(nr.node_type, 4);
    }

    
    #[test]
    fn test_relationship_record() {
        let val = RelationshipRecord {source: 2, target: 3,
            next_inbound_edge: 4, next_outbound_edge: 5,
            relationship_type: 33, next_prop_id: 100};
        let bytes = val.to_bytes();
        let rr = RelationshipRecord::from_bytes(bytes);
        assert_eq!(rr.source, 2);
        assert_eq!(rr.target, 3);
        assert_eq!(rr.next_inbound_edge, 4);
        assert_eq!(rr.next_outbound_edge, 5);
        assert_eq!(rr.relationship_type, 33);
        assert_eq!(rr.next_prop_id, 100);
    }
}