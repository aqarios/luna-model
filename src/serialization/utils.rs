pub fn force_u32(n: usize) -> u32 {
    n.try_into().unwrap()
}

pub fn force_u8(n: u32) -> u8 {
    n.try_into().unwrap()
}

pub trait Slicable {
    fn as_slice(&self) -> &[u8];
}

pub trait Vectorizeable {
    fn to_vec(self) -> Vec<u8>;
}
