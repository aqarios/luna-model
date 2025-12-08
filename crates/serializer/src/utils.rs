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
