pub fn force_u32(n: usize) -> u32 {
    n.try_into().unwrap()
}
