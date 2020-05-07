use super::super::io::file_access::FileAccess;

pub struct BNode {

}


const NB_SLOT: usize = 66;
const SLOT_SIZE: usize = std::mem::size_of::<u64>();
const NB_ITEM: usize = 2 * NB_SLOT + 1;
const BLOCK_SIZE: usize = NB_ITEM * SLOT_SIZE;

#[derive(Copy, Clone)]
pub struct BNodeRecord {
    pub keys: [u64; NB_SLOT],
    pub ptrs: [u64; NB_SLOT + 1],
}

impl BNodeRecord {
    fn get_ptr_value(&self, n: usize) -> u64 {
        self.ptrs[n]
    }
    fn get_key_value(&self, n: usize) -> u64 {
        self.keys[n]
    }
    fn to_bytes(&self) -> [u8; BLOCK_SIZE] {
        let mut bytes = [0u8; BLOCK_SIZE];
        let mut index = 0;
        for key in self.keys.iter() {
            bytes[index..index+8].copy_from_slice(&key.to_be_bytes());
            index += 8;
        }
        for ptr in self.ptrs.iter() {
            bytes[index..index+8].copy_from_slice(&ptr.to_be_bytes());
            index += 8;
        }
        bytes
    }
    fn from_bytes(bytes: [u8; BLOCK_SIZE]) -> Self {
        let mut keys = [0u64; NB_SLOT];
        for index in 0..NB_SLOT {
            let mut tmp = [0u8; SLOT_SIZE];
            tmp.copy_from_slice(&bytes[index*SLOT_SIZE..index*(SLOT_SIZE+1)]);
            keys[index] = u64::from_be_bytes(tmp);
        }
        let mut ptrs = [0u64; NB_SLOT+1];
        for index in 0..NB_SLOT+1 {
            let mut tmp = [0u8; SLOT_SIZE];
            tmp.copy_from_slice(&bytes[index*SLOT_SIZE..index*(SLOT_SIZE+1)]);
            ptrs[index] = u64::from_be_bytes(tmp);
        }
        BNodeRecord{keys: keys, ptrs: ptrs}
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
            let res = node.keys.binary_search(&value);
            match res {
                Ok(found) => {
                    let mut data = [0u8; BLOCK_SIZE];
                    self.file_access.read_at(node.ptrs[found+1], &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                },
                Err(not_found) => {
                    let mut data = [0u8; BLOCK_SIZE];
                    self.file_access.read_at(node.ptrs[not_found], &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: u64) -> Option<BNodeRecord> {
        if self.file_access.get_file_len() < BLOCK_SIZE as u64 {
            None
        } else {
            let mut data = [0u8; BLOCK_SIZE];
            self.file_access.read_at(0, &mut  data);
            self.tree_search(value, &BNodeRecord::from_bytes(data), 0)
        }
    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
    }
}