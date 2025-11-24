use lunamodel_core::Expression;

use crate::{encode::BytesEncodable, utils::force_u32};

use prost::Message;
use super::SerExpression;

/// Makes the SerExpression conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerExpression {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerExpression {
    /// Fills the serializable expression based on an instance of Expression.
    fn fill(mut self, expression: &Expression) -> Self {
        self.num_variables = force_u32(expression.num_variables());
        self.active = expression.active.iter().map(|b| *b).collect();
        self.offset = expression.offset;
        self.linear = expression.linear.to_vec(expression.active.len());

        if let Some(quad) = &expression.quadratic {
            self.quad_size = force_u32(expression.active.len());
            for t in quad.iter() {
                if !t.neighborhood.is_empty() {
                    // only store data if the neighborhood is not empty.
                    self.quad_neighborhood_indices
                        .push(force_u32(t.index.into()));
                    self.quad_neighborhoods_len
                        .push(force_u32(t.neighborhood.len()));
                    t.neighborhood.iter().for_each(|e| {
                        self.quad_neighborhoods.push(e.index.0);
                        self.quad_neighborhoods_values.push(e.bias);
                    });
                }
            }
        }

        if let Some(ho) = &expression.higher_order {
            self.ho_size = force_u32(ho.len());
            for (ids, bias) in ho.iter_contrib() {
                self.ho_lens.push(force_u32(ids.len()));
                self.ho_values.push(*bias);
                ids.iter().for_each(|id| {
                    self.ho_indices.push(id.0);
                });
            }
        }

        self
    }
}
