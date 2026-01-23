use prost::Message;

use lunamodel_core::Expression;

/// Representation of a bytes encodable/decodable Expression
/// to compute the hash from.
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
        let maxidx = *expr.vars().map(|v| v.id()).max().get_or_insert(0) as usize;
        let num_vars = match maxidx {
            0 => 0,
            m => m + 1,
        };

        let mut linear = vec![0.0; num_vars];
        let mut active = vec![false; num_vars];

        let mut quad_size = 0;
        let mut quad_neighborhood_indices = Vec::new();
        let mut quad_neighborhoods = Vec::new();
        let mut quad_neighborhoods_values = Vec::new();
        let mut quad_neighborhoods_len = Vec::new();

        let mut ho_size = 0;
        let mut ho_values = Vec::new();
        let mut ho_indices = Vec::new();
        let mut ho_lens = Vec::new();

        for (u, bias) in expr.linear_items() {
            active[u.id() as usize] = true;
            linear[u.id() as usize] = bias;
        }

        if let Some(quad) = &expr.quadratic {
            quad_size = active.len() as u32;
            for (vidx, neigborhood) in quad.iter() {
                if !neigborhood.is_empty() {
                    active[vidx as usize] = true;
                    quad_neighborhood_indices.push(vidx);
                    quad_neighborhoods_len.push(neigborhood.len() as u32);
                    for (uidx, bias) in neigborhood.iter() {
                        active[uidx as usize] = true;
                        quad_neighborhoods.push(uidx);
                        quad_neighborhoods_values.push(bias);
                    }
                }
            }
        }

        if let Some(ho) = &expr.higher_order {
            ho_size = ho.len() as u32;
            for (ids, bias) in ho.iter_contrib() {
                ho_lens.push(ids.len() as u32);
                ho_values.push(bias);
                for &id in ids.iter() {
                    active[id as usize] = true;
                    ho_indices.push(id);
                }
            }
        }

        let o = HashExpr {
            num_variables: expr.num_vars() as u32,
            active,
            offset: expr.offset,
            linear,
            quad_size,
            quad_neighborhood_indices,
            quad_neighborhoods,
            quad_neighborhoods_values,
            quad_neighborhoods_len,
            ho_size,
            ho_values,
            ho_indices,
            ho_lens,
        };
        o.encode_to_vec()
    }
}
