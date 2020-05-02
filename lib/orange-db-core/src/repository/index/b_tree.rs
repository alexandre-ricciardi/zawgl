use super::super::byte_utils::*;
use orange_db_binary_serde_traits::BinarySer;
use orange_db_binary_serde_traits::BinaryDeser;


pub struct BNode {

}

#[derive(BinarySer, BinaryDeser, PartialEq, Eq, Debug)]
pub struct BNodeRecord {
    pub ptr_0: u64,
    pub slot_0: u64,
    pub ptr_1: u64,
    pub slot_1: u64,
    pub ptr_2: u64,
    pub slot_2: u64,
    pub ptr_3: u64,
    pub next: u64,
}


#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
        let rec = BNodeRecord{ptr_0: {1 + 2 + 3}, ptr_1: 0, ptr_2: 0, ptr_3: 0, slot_0: 1, slot_1: 2, slot_2: 3, next: 3};
        let data = rec.ser();
        let deser = BNodeRecord::deser(&data);
        assert_eq!(rec, deser)
    }
}