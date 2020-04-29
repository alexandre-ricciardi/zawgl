pub trait BinarySer {
    fn ser(&self) -> Vec<u8>;
}

pub trait BinaryDeser {
    type Item;
    fn deser(bytes: &Vec<u8>) -> Self::Item;
}