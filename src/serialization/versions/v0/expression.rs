use crate::{
    core::{
        environment::SharedEnvironment,
        expression::ExpressionBaseCreation,
        term::{
            types::{OneVarTerm, OneVarTermConstruction},
            HigherOrder, Linear, Quadratic,
        },
        Expression, VarId,
    },
    serialization::encodable::{BytesDecodable, BytesEncodable, DecodeError},
};
use prost::Message;

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
impl BytesEncodable for SerExpression {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerExpression conform with the requirements for it to be a Decodable.
impl BytesDecodable<Expression, SharedEnvironment> for SerExpression {
    fn decode_from_bytes(
        bytes: &[u8],
        payload: SharedEnvironment,
    ) -> Result<Expression, DecodeError> {
        Ok(Self::decode(bytes)?.extract(payload))
    }
}
impl SerExpression {
    fn decode_quadratic(&self) -> Option<Quadratic> {
        if self.quad_size == 0 {
            return None;
        }
        let mut quad = Quadratic::new(self.quad_size as usize);
        let mut start = 0;
        for (u, len) in self
            .quad_neighborhood_indices
            .iter()
            .zip(&self.quad_neighborhoods_len)
        {
            let end = start + len;
            for i in start..end {
                quad[*u as usize].push(OneVarTerm::new(
                    VarId(self.quad_neighborhoods[i as usize]),
                    self.quad_neighborhoods_values[i as usize],
                ));
            }
            start = end;
        }

        Some(quad)
    }

    fn decode_higher_order(&self) -> Option<HigherOrder> {
        if self.ho_size == 0 {
            return None;
        }

        let mut ho = HigherOrder::with_size(self.ho_size as usize);

        let mut start: usize = 0;
        for (len, value) in self.ho_lens.iter().zip(&self.ho_values) {
            let end = start + (*len as usize);
            let contribs = self.ho_indices[start..end]
                .iter()
                .map(|u| VarId(*u))
                .collect::<Vec<VarId>>();
            ho[&contribs] = *value;
            start = end;
        }

        Some(ho)
    }

    /// Extracts the data from self to and instance of Expression with Index VarId and
    /// Bias f64.
    pub fn extract(&self, env: SharedEnvironment) -> Expression {
        let mut expr = Expression::empty(env);
        expr.num_variables = self.num_variables as usize;
        expr.active = self.active.clone();
        expr.offset = self.offset;
        expr.linear = Linear::new(self.linear.clone()); // todo(team): might be optimizable with mem copies. See somewhere in code where I do something similar.
        expr.quadratic = self.decode_quadratic();
        expr.higher_order = self.decode_higher_order();
        expr
    }
}
