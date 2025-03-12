use super::utils::force_u32;
use crate::core::{
    expression::ExpressionBaseCreation,
    term::{
        types::{OneVarTerm, OneVarTermConstruction},
        HigherOrder, Linear, Quadratic,
    },
    Environment, Expression, ExpressionBase, VarId,
};
use prost::Message;
use std::{cell::RefCell, rc::Rc};

struct Quad {
    /// If quadratic is some
    size: u32,
    /// the indices of the neighbors as a vector.
    neighborhood_indices: Vec<u32>,
    /// the neighboring indices as a vector.
    neighborhoods: Vec<u32>,
    /// the values of the neighborhood as a vector.
    neighborhoods_values: Vec<f64>,
    /// the length of the neighborhoods as a vector.
    neighborhoods_len: Vec<u32>,
}

impl Quad {
    fn default() -> Self {
        Self {
            size: 0,
            neighborhood_indices: Vec::new(),
            neighborhoods: Vec::new(),
            neighborhoods_values: Vec::new(),
            neighborhoods_len: Vec::new(),
        }
    }
}

struct Ho {
    size: usize,
    /// Higher Order values stored in the hashmap.
    values: Vec<f64>,
    /// Higher Order indices stored in the hashmap concatenated.
    indices: Vec<u32>,
    /// Number of keys per higher order entry
    lens: Vec<u32>,
}

impl Ho {
    fn default() -> Self {
        Self {
            size: 0,
            values: Vec::new(),
            indices: Vec::new(),
            lens: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerExpression {
    #[prost(uint64, tag = "1")]
    num_variables: u64,
    #[prost(bool, repeated, tag = "2")]
    active: Vec<bool>,

    #[prost(double, tag = "3")]
    offset: f64,
    #[prost(double, repeated, tag = "4")]
    linear: Vec<f64>,

    /// If the expression has a quadratic term or not
    #[prost(uint32, tag = "5")]
    quad_size: u32,
    /// Which indices actually have a neighborhood.
    #[prost(uint32, repeated, tag = "6")]
    quad_neighborhood_indices: Vec<u32>,
    /// the indices of the neighbors as a vector.
    #[prost(uint32, repeated, tag = "7")]
    quad_neighborhoods: Vec<u32>,
    /// the values of the neighborhood as a vector.
    #[prost(double, repeated, tag = "8")]
    quad_neighborhoods_values: Vec<f64>,
    /// the length of the neighborhoods as a vector.
    #[prost(uint32, repeated, tag = "9")]
    quad_neighborhoods_len: Vec<u32>,

    /// If the expression has a quadratic term or not
    #[prost(uint64, tag = "10")]
    ho_size: u64,
    /// Higher Order values stored in the hashmap.
    #[prost(double, repeated, tag = "11")]
    ho_values: Vec<f64>,
    /// Higher Order indices stored in the hashmap concatenated.
    #[prost(uint32, repeated, tag = "12")]
    ho_indices: Vec<u32>,
    /// Number of keys per higher order entry
    #[prost(uint32, repeated, tag = "13")]
    ho_lens: Vec<u32>,
}

impl SerExpression {
    pub fn new(expression: &Expression<VarId, f64>) -> Self {
        let quad = Self::encode_quadratic(&expression.quadratic);
        let ho = Self::encode_higher_order(&expression.higher_order);
        Self {
            num_variables: expression.num_variables() as u64,
            active: expression.active.clone(),
            offset: expression.offset,
            linear: expression.linear.to_vec().clone(),
            quad_size: quad.size,
            quad_neighborhood_indices: quad.neighborhood_indices,
            quad_neighborhoods: quad.neighborhoods,
            quad_neighborhoods_values: quad.neighborhoods_values,
            quad_neighborhoods_len: quad.neighborhoods_len,
            ho_size: ho.size as u64,
            ho_values: ho.values,
            ho_indices: ho.indices,
            ho_lens: ho.lens,
        }
    }

    fn encode_quadratic(quadratic: &Option<Quadratic<VarId, f64>>) -> Quad {
        let mut out = Quad::default();
        if let Some(quad) = &quadratic {
            out.size = force_u32(quad.len());
            for (u, neighborhood) in quad.iter() {
                if !neighborhood.is_empty() {
                    // only store data if the neighborhood is not empty.
                    out.neighborhood_indices.push(force_u32(u));
                    out.neighborhoods_len.push(force_u32(neighborhood.len()));
                    neighborhood.iter().for_each(|e| {
                        out.neighborhoods.push(e.index.0);
                        out.neighborhoods_values.push(e.bias);
                    });
                }
            }
        }
        out
    }

    fn decode_quadratic(&self) -> Option<Quadratic<VarId, f64>> {
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

    fn encode_higher_order(higher_order: &Option<HigherOrder<VarId, f64>>) -> Ho {
        if higher_order.is_none() {
            return Ho::default();
        }

        let highero = higher_order.as_ref().unwrap();
        let mut ho = Ho::default();
        ho.size = highero.len();
        for (ids, bias) in highero.iter_contrib() {
            ho.lens.push(force_u32(ids.len()));
            ho.values.push(*bias);
            ids.iter().for_each(|id| {
                ho.indices.push(id.0);
            });
        }
        ho
    }

    fn decode_higher_order(&self) -> Option<HigherOrder<VarId, f64>> {
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

    pub fn extract(&self, env: Rc<RefCell<Environment<VarId>>>) -> Expression<VarId, f64> {
        let mut expr = Expression::empty(Rc::clone(&env));
        expr.num_variables = self.num_variables as usize;
        expr.active = self.active.clone();
        expr.offset = self.offset;
        expr.linear = Linear::new(self.linear.clone()); // might be optimizable with mem copies.
        expr.quadratic = self.decode_quadratic();
        expr.higher_order = self.decode_higher_order();
        expr
    }
}
