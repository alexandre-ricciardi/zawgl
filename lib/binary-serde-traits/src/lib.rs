pub trait BinarySer {
    fn ser(&self) -> Vec<u8>;
}

pub trait BinaryDeser {
    fn deser(bytes: &Vec<u8>) -> Self;
}