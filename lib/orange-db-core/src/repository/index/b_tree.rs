use super::super::io::file_access::FileAccess;
use super::super::pager::*;


pub struct BNode {

}


const NB_SLOT: usize = 66;
const SLOT_SIZE: usize = std::mem::size_of::<u64>();
const NB_ITEM: usize = 2 * NB_SLOT + 1;
const BLOCK_SIZE: usize = NB_ITEM * SLOT_SIZE + 1;

#[derive(Copy, Clone)]
pub struct BNodeRecord {
    pub header: u8,
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
        bytes[index] = self.header;
        index += 1;
        for key in self.keys.iter() {
            bytes[index..index+SLOT_SIZE].copy_from_slice(&key.to_be_bytes());
            index += SLOT_SIZE;
        }
        for ptr in self.ptrs.iter() {
            bytes[index..index+SLOT_SIZE].copy_from_slice(&ptr.to_be_bytes());
            index += SLOT_SIZE;
        }
        bytes
    }
    fn from_bytes(bytes: [u8; BLOCK_SIZE]) -> Self {
        let header = bytes[0];
        let mut keys = [0u64; NB_SLOT];
        for index in 1..NB_SLOT+1 {
            let mut tmp = [0u8; SLOT_SIZE];
            tmp.copy_from_slice(&bytes[index*SLOT_SIZE..index*(SLOT_SIZE+1)]);
            keys[index] = u64::from_be_bytes(tmp);
        }
        let mut ptrs = [0u64; NB_SLOT+1];
        for index in NB_SLOT+1..BLOCK_SIZE {
            let mut tmp = [0u8; SLOT_SIZE];
            tmp.copy_from_slice(&bytes[index*SLOT_SIZE..index*(SLOT_SIZE+1)]);
            ptrs[index] = u64::from_be_bytes(tmp);
        }
        BNodeRecord{header: header, keys: keys, ptrs: ptrs}
    }
    fn len(&self) -> usize {
        (self.header & 0x00FF) as usize
    }
    fn is_full(&self) -> bool {
        self.len() >= 2 * NB_ITEM
    }
    fn is_leaf(&self) -> bool {
        (self.header & 0b1000_0000) == 1
    }
}

pub struct BTreeIndex {
    pager: Pager,
    tree_depth: u32,
}

pub type BTreeResult = std::result::Result<BNodeRecord, Option<BNodeRecord>>;

impl BTreeIndex {
    pub fn new(file: &str, tree_depth: u32) -> Self {
        BTreeIndex{pager: Pager::new(file, BLOCK_SIZE as u64), tree_depth: tree_depth}
    }

    fn is_leaf_node(&self, depth: u32) -> bool {
        depth == self.tree_depth
    }

    fn tree_search(&mut self, value: u64, node: &BNodeRecord, depth: u32) -> BTreeResult {
        if self.is_leaf_node(depth) {
            Ok(*node)
        } else {
            let res = node.keys.binary_search(&value);
            match res {
                Ok(found) => {
                    let mut data = [0u8; BLOCK_SIZE];
                    self.pager.load(node.ptrs[found], &mut data).or_else(|_| Err(None))?;
                    let child = BNodeRecord::from_bytes(data);
                    if child.is_leaf() {
                        Ok(child)
                    } else {
                        self.tree_search(value, &child, depth+1)
                    }
                },
                Err(not_found) => {
                    let mut data = [0u8; BLOCK_SIZE];
                    self.pager.load(node.ptrs[not_found], &mut data).or_else(|_| Err(None))?;
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: u64) -> BTreeResult {
        if self.pager.is_empty() {
            Err(None)
        } else {
            let mut data = [0u8; BLOCK_SIZE];
            self.pager.load(0, &mut  data).or_else(|_| Err(None))?;
            self.tree_search(value, &BNodeRecord::from_bytes(data), 0)
        }
    }

    pub fn insert(&mut self, value: u64, data_ptr: u64) {

    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
    }
}