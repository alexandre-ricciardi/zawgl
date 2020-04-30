use super::super::byte_utils::*;
use orange_db_binary_serde_traits::BinarySer;
use orange_db_binary_serde_traits::BinaryDeser;

fn inc_offset(o: usize) -> usize {
    std::mem::size_of::<u64>() + o
}

macro_rules! to_bytes_impl {
    [$bytes:ident, $($field:expr),*] => {
        let mut offset = 0usize;
        let upper_bound = inc_offset(offset);
        $($bytes[offset..inc_offset(offset)].clone_from_slice(&u64_to_bytes($field));offset += upper_bound;)*
    };
}


fn deser_and_inc(bytes: &[u8], offset: usize) -> u64 {
    let upper_bound = inc_offset(offset);
    let res = u64_from_bytes(&bytes[offset..upper_bound]);
    res
}

macro_rules! from_bytes_impl {
    [$bytes:ident, $s_name:ty, $($field:expr),*] => {
        let mut offset = 0;
        let res = $s_name {$($field: deser_and_inc($bytes, offset)),*};
        res
    };
}

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

fn to_bytes(bnr: &BNodeRecord) -> [u8; 64] {
    let mut bytes = [0u8; 64];
    to_bytes_impl![bytes, bnr.ptr_0, bnr.slot_0];
    bytes
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