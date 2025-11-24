mod decode;
mod encode;

use lunamodel_core::Expression;
use prost::Message;

use crate::encode::Creatable;

/// Representation of a bytes encodable/decodable Expression.
#[derive(Clone, PartialEq, Message)]
pub struct SerExpression {
    /// The number of variables in the expression.
    #[prost(uint32, tag = "1")]
    num_variables: u32,
    /// A vector of booleans indicating which variables are active in the expression
    /// and which are not.
    #[prost(bool, repeated, tag = "2")]
    active: Vec<bool>,

    /// The constant offset of the expression.
    #[prost(double, tag = "3")]
    offset: f64,
    /// The linear term of the expression.
    #[prost(double, repeated, tag = "4")]
    linear: Vec<f64>,

    /// The size of the quadratic term. This is either 0 or equal to the number of
    /// variables in the expression.
    #[prost(uint32, tag = "5")]
    quad_size: u32,
    /// The variable indices with a non-empty neighborhood, i.e., the variable indices
    /// which have at least one quadratic interaction.
    #[prost(uint32, repeated, tag = "6")]
    quad_neighborhood_indices: Vec<u32>,
    /// The indices of all variables in any neighborhood as a single concatenated vector.
    #[prost(uint32, repeated, tag = "7")]
    quad_neighborhoods: Vec<u32>,
    /// The biases for all quadratic interactions as a single concatenated vector.
    /// This vector's length is equal to the length og the `quad_neighborhoods` vector.
    #[prost(double, repeated, tag = "8")]
    quad_neighborhoods_values: Vec<f64>,
    /// The size of the neighborhood for each variable in the `quad_neighborhood_indices`
    /// vector. Required to recover the neighborhoods for all variables during decoding.
    #[prost(uint32, repeated, tag = "9")]
    quad_neighborhoods_len: Vec<u32>,

    /// The size of the higher order term, i.e., how many elements the higher order
    /// term consists of. This is especially useful during decoding, as the appropriate
    /// sized HashMap can be created improving write performances significantly.
    #[prost(uint32, tag = "10")]
    ho_size: u32,
    /// All biases in the higher order term concatenated to a single vector.
    #[prost(double, repeated, tag = "11")]
    ho_values: Vec<f64>,
    /// All variable inidices of all higher order interactions stored in the higher
    /// order term represented as a single concatenated vector.
    #[prost(uint32, repeated, tag = "12")]
    ho_indices: Vec<u32>,
    /// The number of elements in each of the higher order terms as a single concatenated
    /// vector. The length of this vector is equal to the `ho_size` variable. This vector
    /// is required to recover the correct higher order term during decoding. Each value
    /// indicates how many variables interact for each element in the term. The sum of
    /// all elements has to be equal to the length of the ho_indices vector.
    #[prost(uint32, repeated, tag = "13")]
    ho_lens: Vec<u32>,
}

/// Makes the SerExpression conform with the requirements for it to be an Encodable.
impl Creatable<Expression> for SerExpression {
    fn new(value: &Expression) -> Self {
        Self::default().fill(&value)
    }
}

impl SerExpression {
    /// Creates an empty serializable expression struct.
    fn default() -> Self {
        Self {
            num_variables: u32::default(),
            active: Vec::new(),
            offset: f64::default(),
            linear: Vec::new(),
            quad_size: u32::default(),
            quad_neighborhood_indices: Vec::new(),
            quad_neighborhoods: Vec::new(),
            quad_neighborhoods_values: Vec::new(),
            quad_neighborhoods_len: Vec::new(),
            ho_size: u32::default(),
            ho_values: Vec::new(),
            ho_indices: Vec::new(),
            ho_lens: Vec::new(),
        }
    }
}
