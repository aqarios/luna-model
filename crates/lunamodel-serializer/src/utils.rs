/// Force translation of a `usize` to a `u32`. Used heavily in serialization tasks due
/// to our restrictions of a maximum number of variables that can be represented by a
/// u32. However, some default methods return `usize` typed data by default, which
/// however we know to always be expressable by a u32, due to the internals of this lib.
pub fn force_u32(n: usize) -> u32 {
    n.try_into().unwrap()
}

/// Force translation of a `i32` to a `i8`. Used heavily in serialization tasks.
pub fn force_i8(n: i32) -> i8 {
    n.try_into().unwrap()
}

/// Implementation of this trait on any type ensures that the respective type can be expressed
/// as a slice of bytes.
pub trait Slicable {
    fn as_slice(&self) -> &[u8];
}

/// Implementation of this trait on any type ensures that the respective type can be expressed
/// as a bytes vector.
pub trait Vectorizable {
    fn to_vec(self) -> Vec<u8>;
}
