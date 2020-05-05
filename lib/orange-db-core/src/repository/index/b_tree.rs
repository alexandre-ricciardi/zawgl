use super::super::io::file_access::FileAccess;

pub struct BNode {

}


const NB_SLOT: usize = 132;
const SLOT_SIZE: usize = std::mem::size_of::<u64>();
const BLOCK_SIZE: usize = NB_SLOT * SLOT_SIZE;
pub struct BNodeRecord {
    pub block: [u64; NB_SLOT],
}

impl BNodeRecord {
    fn get_ptr_value(&self, n: usize) -> u64 {
        self.block[n * 2]
    }
    fn get_key_value(&self, n: usize) -> u64 {
        self.block[1 + n * 2]
    }
    fn to_bytes(&self) -> [u8; BLOCK_SIZE] {
        let mut bytes = [0u8; BLOCK_SIZE];
        let mut index = 0;
        for value in self.block.iter() {
            bytes[index..index+8].copy_from_slice(&value.to_be_bytes());
            index += 8;
        }
        bytes
    }
    fn from_bytes(bytes: [u8; BLOCK_SIZE]) -> Self {
        let block = [0u64; NB_SLOT];
        for index in 0..NB_SLOT {
            let mut tmp = [0u8; SLOT_SIZE];
            tmp.copy_from_slice(&bytes[index*SLOT_SIZE..index*(SLOT_SIZE+1)]);
            block[index] = u64::from_be_bytes(tmp);
        }
        BNodeRecord{block: block}
    }
}

pub struct BTreeIndex {
    file_access: FileAccess,
    tree_depth: u32,
}

impl BTreeIndex {
    pub fn new(file: &str, tree_depth: u32) -> Self {
        BTreeIndex{file_access: FileAccess::new(file), tree_depth: tree_depth}
    }

    fn is_leaf_node(&self, depth: u32) -> bool {
        depth == self.tree_depth
    }

    fn tree_search(&mut self, value: u64, node: &BNodeRecord, depth: u32) -> Option<BNodeRecord> {
        if self.is_leaf_node(depth) {
            Some(*node)
        } else {
            let res = node.block.binary_search(value)
        }
    }

    pub fn search(&mut self, value: u64) -> Option<BNodeRecord> {
        if self.file_access.get_file_len() == 0 {
            None
        } else {
            let mut data = [0u8; 56];
            self.file_access.read_at(0, &mut  data);
            Some(BNodeRecord::from_bytes(data))
        }
    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
        let rec = BNodeRecord{ptr_0: {1 + 2 + 3}, ptr_1: 12, ptr_2: 33, ptr_3: 99, slot_0: 1899988, slot_1: 214975, slot_2: 22223};
        let data = rec.to_bytes();
        let deser = BNodeRecord::from_bytes(data);
        assert_eq!(rec, deser)
    }
}