use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use prost::Message;

use crate::core::{
    expression::ExpressionBaseCreation,
    term::{
        types::{OneVarTerm, OneVarTermConstruction},
        Linear, Quadratic,
    },
    Environment, Expression, ExpressionBase, VarId,
};

// #[derive(Clone, PartialEq, Message)]
// pub struct SerializableOneVarTerm {
//     #[prost(fixed32, tag = "1")]
//     index: u32,
//     #[prost(double, tag = "2")]
//     bias: f64,
// }

#[derive(Clone, PartialEq, Message)]
pub struct SerializableTwoVarTerm {
    #[prost(fixed64, tag = "1")]
    u_idx: u64,
    #[prost(fixed64, tag = "2")]
    v_idx: u64,
    #[prost(fixed32, tag = "3")]
    u: u32,
    #[prost(fixed32, tag = "4")]
    v: u32,
    #[prost(double, tag = "5")]
    bias: f64,
}

impl SerializableTwoVarTerm {
    fn new(u_idx: u64, v_idx: u64, u: u32, v: u32, bias: f64) -> Self {
        Self {
            u_idx,
            v_idx,
            u,
            v,
            bias,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableQuadratic {
    #[prost(message, repeated, tag = "1")]
    terms: Vec<SerializableTwoVarTerm>,
}

impl SerializableQuadratic {
    fn new(terms: Vec<SerializableTwoVarTerm>) -> Self {
        Self { terms }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct HigherOrderElement {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(double, tag = "2")]
    bias: f64,
}

impl HigherOrderElement {
    fn new(key: String, bias: f64) -> Self {
        Self { key, bias }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableExpression {
    // skip some stuff at this data is already in the model.
    // and this should be used only in the model serialization
    /// The offset of the expression
    #[prost(double, tag = "1")]
    offset: f64,
    /// The linear terms of the expression
    #[prost(double, repeated, tag = "2")]
    linear: Vec<f64>,
    /// The sparse quadratic matrix
    #[prost(message, tag = "3")]
    quadratic: Option<SerializableQuadratic>,
    /// The higher order biases as a flat vector.
    #[prost(message, repeated, tag = "4")]
    higher_order: Vec<HigherOrderElement>,
    #[prost(bool, repeated, tag = "5")]
    active_variables: Vec<bool>,
    #[prost(fixed64, tag = "6")]
    num_variables: u64,
}

impl SerializableExpression {
    pub fn new(expression: Ref<'_, Expression<VarId, f64>>) -> Self {
        Self {
            offset: expression.offset,
            linear: expression.linear.to_vec().clone(),
            quadratic: Self::build_quadratic(&expression),
            higher_order: Self::build_higher_order(&expression),
            active_variables: expression.active.clone(),
            num_variables: expression.num_variables() as u64,
        }
    }

    fn build_quadratic(
        expression: &Ref<'_, Expression<VarId, f64>>,
    ) -> Option<SerializableQuadratic> {
        expression
            .quadratic
            .as_ref()
            .map(|q| {
                q.iter_flat_positioned()
                    .map(|((u_idx, v_idx), u, v, b)| {
                        SerializableTwoVarTerm::new(u_idx as u64, v_idx as u64, u.0, v.0, b)
                    })
                    .collect::<Vec<SerializableTwoVarTerm>>()
            })
            .map_or(None, |e| Some(SerializableQuadratic::new(e)))
    }

    fn build_higher_order(expression: &Ref<'_, Expression<VarId, f64>>) -> Vec<HigherOrderElement> {
        expression
            .higher_order
            .as_ref()
            .map(|ho| {
                ho.iter()
                    .map(|(key, bias)| HigherOrderElement::new(key.clone(), *bias))
                    .collect()
            })
            .map_or(Vec::default(), |b| b)
    }

    pub fn extract(&self, environment: Rc<RefCell<Environment<VarId>>>) -> Expression<VarId, f64> {
        let mut expr = Expression::new(Rc::clone(&environment));
        expr.offset = self.offset;
        expr.linear = Linear::new(self.linear.clone());
        expr.active = self.active_variables.clone();
        expr.num_variables = self.num_variables as usize;
        expr.quadratic = match &self.quadratic {
            Some(q) => {
                let mut adj = vec![Vec::<OneVarTerm<VarId, f64>>::new(); expr.num_variables];
                q.terms.iter().for_each(|term| {
                    adj[term.u_idx as usize].insert(
                        term.v_idx as usize,
                        OneVarTerm::new(VarId(term.v), term.bias),
                    );
                });
                Some(Quadratic::new_from(adj))
            }
            None => None,
        };
        match &self.higher_order.len() {
            0 => expr.higher_order = None,
            _ => {
                expr.enforce_higher_order();
                self.higher_order.iter().for_each(|h| {
                    expr.higher_order.as_mut().unwrap()[&h.key] = h.bias;
                })
            }
        };
        expr
    }
}
