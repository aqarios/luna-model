use prost::Message;

use crate::core::Expression;

use super::utils::force_u32;

/// Representation of a bytes encodable/decodable Expression.
#[derive(Clone, PartialEq, Message)]
pub struct HashExpr {
    /// The number of variables in the expression.
    #[prost(uint32, tag = "1")]
    pub num_variables: u32,
    /// A vector of booleans indicating which variables are active in the expression
    /// and which are not.
    #[prost(bool, repeated, tag = "2")]
    pub active: Vec<bool>,

    /// The constant offset of the expression.
    #[prost(double, tag = "3")]
    pub offset: f64,
    /// The linear term of the expression.
    #[prost(double, repeated, tag = "4")]
    pub linear: Vec<f64>,

    /// The size of the quadratic term. This is either 0 or equal to the number of
    /// variables in the expression.
    #[prost(uint32, tag = "5")]
    pub quad_size: u32,
    /// The variable indices with a non-empty neighborhood, i.e., the variable indices
    /// which have at least one quadratic interaction.
    #[prost(uint32, repeated, tag = "6")]
    pub quad_neighborhood_indices: Vec<u32>,
    /// The indices of all variables in any neighborhood as a single concatenated vector.
    #[prost(uint32, repeated, tag = "7")]
    pub quad_neighborhoods: Vec<u32>,
    /// The biases for all quadratic interactions as a single concatenated vector.
    /// This vector's length is equal to the length og the `quad_neighborhoods` vector.
    #[prost(double, repeated, tag = "8")]
    pub quad_neighborhoods_values: Vec<f64>,
    /// The size of the neighborhood for each variable in the `quad_neighborhood_indices`
    /// vector. Required to recover the neighborhoods for all variables during decoding.
    #[prost(uint32, repeated, tag = "9")]
    pub quad_neighborhoods_len: Vec<u32>,

    /// The size of the higher order term, i.e., how many elements the higher order
    /// term consists of. This is especially useful during decoding, as the appropriate
    /// sized HashMap can be created improving write performances significantly.
    #[prost(uint32, tag = "10")]
    pub ho_size: u32,
    /// All biases in the higher order term concatenated to a single vector.
    #[prost(double, repeated, tag = "11")]
    pub ho_values: Vec<f64>,
    /// All variable inidices of all higher order interactions stored in the higher
    /// order term represented as a single concatenated vector.
    #[prost(uint32, repeated, tag = "12")]
    pub ho_indices: Vec<u32>,
    /// The number of elements in each of the higher order terms as a single concatenated
    /// vector. The length of this vector is equal to the `ho_size` variable. This vector
    /// is required to recover the correct higher order term during decoding. Each value
    /// indicates how many variables interact for each element in the term. The sum of
    /// all elements has to be equal to the length of the ho_indices vector.
    #[prost(uint32, repeated, tag = "13")]
    pub ho_lens: Vec<u32>,
}

impl HashExpr {
    pub fn build(expr: &Expression) -> Vec<u8> {
        let mut serexpr = HashExpr {
            num_variables: force_u32(expr.num_variables),
            active: expr.active.clone(),
            offset: expr.offset,
            linear: expr.linear.to_vec(expr.num_variables),
            quad_size: u32::default(),
            quad_neighborhood_indices: Vec::new(),
            quad_neighborhoods: Vec::new(),
            quad_neighborhoods_values: Vec::new(),
            quad_neighborhoods_len: Vec::new(),
            ho_size: u32::default(),
            ho_values: Vec::new(),
            ho_indices: Vec::new(),
            ho_lens: Vec::new(),
        };

        if let Some(quad) = &expr.quadratic {
            serexpr.quad_size = force_u32(quad.len());
            for t in quad.iter() {
                if !t.neighborhood.is_empty() {
                    // only store data if the neighborhood is not empty.
                    serexpr.quad_neighborhood_indices.push(force_u32(t.index.into()));
                    serexpr
                        .quad_neighborhoods_len
                        .push(force_u32(t.neighborhood.len()));
                    t.neighborhood.iter().for_each(|e| {
                        serexpr.quad_neighborhoods.push(e.index.0);
                        serexpr.quad_neighborhoods_values.push(e.bias);
                    });
                }
            }
        }

        if let Some(ho) = &expr.higher_order {
            serexpr.ho_size = force_u32(ho.len());
            for (ids, bias) in ho.iter_contrib() {
                serexpr.ho_lens.push(force_u32(ids.len()));
                serexpr.ho_values.push(*bias);
                ids.iter().for_each(|id| {
                    serexpr.ho_indices.push(id.0);
                });
            }
        }
        serexpr.encode_to_vec()
    }
}
