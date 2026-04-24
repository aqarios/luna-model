//! Version 1 encoding for expressions.

use lunamodel_core::Expression;

use crate::encode::BytesEncodable;

use super::SerExpression;
use prost::Message;

/// Makes the SerExpression conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerExpression {
    /// Encodes the protobuf structure into raw bytes.
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerExpression {
    /// Fills the protobuf structure from the runtime expression.
    pub fn fill(mut self, expr: &Expression) -> Self {
        self.is_new = true;
        self.num_variables = expr.num_vars() as u32;
        self.offset = expr.offset;
        Self::fill_linear(&mut self.linear_indices, &mut self.linear_values, expr);
        Self::fill_quadratic(
            &mut self.quad_size,
            &mut self.quad_neighborhood_indices,
            &mut self.quad_neighborhoods,
            &mut self.quad_neighborhoods_values,
            &mut self.quad_neighborhoods_len,
            expr,
        );
        Self::fill_higher_order(
            &mut self.ho_size,
            &mut self.ho_values,
            &mut self.ho_indices,
            &mut self.ho_lens,
            expr,
        );
        self
    }

    /// Serializes the linear term into sparse index/value vectors.
    fn fill_linear(lin: &mut Vec<u32>, linvals: &mut Vec<f64>, expr: &Expression) {
        for (u, b) in expr.linear.iter() {
            lin.push(u);
            linvals.push(b);
        }
    }

    /// Serializes the quadratic term into flattened neighborhood vectors.
    fn fill_quadratic(
        qs: &mut u32,
        qni: &mut Vec<u32>,
        qn: &mut Vec<u32>,
        qnv: &mut Vec<f64>,
        qnl: &mut Vec<u32>,
        expr: &Expression,
    ) {
        if let Some(q) = &expr.quadratic {
            *qs = q.len() as u32;
            for (u, n) in q.iter() {
                if n.is_zero() {
                    continue;
                }
                qni.push(u);
                qnl.push(n.len() as u32);
                for (v, b) in n.iter() {
                    qn.push(v);
                    qnv.push(b);
                }
            }
        }
    }

    /// Serializes the higher-order term into flattened contribution vectors.
    fn fill_higher_order(
        hs: &mut u32,
        hv: &mut Vec<f64>,
        hi: &mut Vec<u32>,
        hl: &mut Vec<u32>,
        expr: &Expression,
    ) {
        if let Some(h) = &expr.higher_order {
            *hs = h.len() as u32;
            for (mut vs, b) in h.iter_contrib() {
                hv.push(b);
                hl.push(vs.len() as u32);
                hi.append(&mut vs);
            }
        }
    }
}
