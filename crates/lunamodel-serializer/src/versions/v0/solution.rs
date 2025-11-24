use crate::encode::Decoder;

pub struct SerSolution {}

/// Makes a [Solution] decodable for V0.
impl Decoder<Solution, ()> for SerSolutionV0 {}
