use super::super::io::file_access::FileAccess;

pub struct BNode {

}

#[derive(Binarize, PartialEq, Eq, Debug)]
pub struct BNodeRecord {
    pub ptr_0: u64,
    pub slot_0: u64,
    pub ptr_1: u64,
    pub slot_1: u64,
    pub ptr_2: u64,
    pub slot_2: u64,
    pub ptr_3: u64,
}

pub struct BTreeIndex {
    file_access: FileAccess,
}

impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{file_access: FileAccess::new(file)}
    }

    pub fn search(&mut self, value: u64) -> Option<BNodeRecord> {
        let mut data = [0u8; 56];
        self.file_access.read_at(0, &mut  data);
        Some(BNodeRecord::from_bytes(data))
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