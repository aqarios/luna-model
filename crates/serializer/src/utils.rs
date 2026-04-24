use lunamodel_types::Vtype;

/// Implementation of this trait on any type ensures that the respective type can be expressed
/// as a slice of bytes.
pub trait Slicable {
    /// Borrows the value as raw bytes.
    fn as_slice(&self) -> &[u8];
}

/// Implementation of this trait on any type ensures that the respective type can be expressed
/// as a bytes vector.
pub trait Vectorizable {
    /// Converts the value into raw bytes.
    fn to_vec(self) -> Vec<u8>;
}

/// Encodes a variable type into the serializer's compact numeric representation.
pub fn vtype_to_u8(vtype: Vtype) -> u8 {
    match vtype {
        Vtype::Binary => 0,
        Vtype::Spin => 1,
        Vtype::Integer => 2,
        Vtype::Real => 3,
        Vtype::InvertedBinary => {
            panic!("solution must not contain variables for type InvertedBinary")
        }
    }
}

/// Decodes the serializer's compact numeric representation into a variable type.
pub fn u8_to_vtype(u: u8) -> Option<Vtype> {
    match u {
        0 => Some(Vtype::Binary),
        1 => Some(Vtype::Spin),
        2 => Some(Vtype::Integer),
        3 => Some(Vtype::Real),
        4 => None, // was ghost
        _ => unreachable!("invalid vtype number"),
    }
}
