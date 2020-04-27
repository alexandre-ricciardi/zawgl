pub fn u64_to_bytes(val: u64) -> [u8; 8] {
    val.to_be_bytes()
}

pub fn u64_from_bytes(b: &[u8]) -> u64 {
    let mut bytes = [0u8; std::mem::size_of::<u64>()];
    bytes.copy_from_slice(b);
    u64::from_be_bytes(bytes)
}
