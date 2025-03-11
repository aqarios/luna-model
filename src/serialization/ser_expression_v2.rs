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

#[derive(Clone, PartialEq, Message)]
pub struct SerializableExpression {
    // skip some stuff at this data is already in the model.
    // and this should be used only in the model serialization
    /// The offset of the expression
    #[prost(fixed64, tag = "1")]
    num_variables: u64,
    #[prost(bool, repeated, tag = "2")]
    active_variables: Vec<bool>,
    #[prost(double, tag = "3")]
    offset: f64,
    /// The linear terms of the expression
    #[prost(double, repeated, tag = "4")]
    linear: Vec<f64>,
    /// The sparse quadratic matrix
    #[prost(fixed32, repeated, tag = "5")]
    quadratic: Vec<u32>,
    #[prost(fixed32, repeated, tag = "6")]
    quadratic_count: Vec<u32>,
    #[prost(double, repeated, tag = "7")]
    quadratic_values: Vec<f64>,
    /// The higher order biases as a flat vector.
    #[prost(string, repeated, tag = "8")]
    higher_order: Vec<String>,
    #[prost(string, repeated, tag = "9")]
    higher_order_values: Vec<f64>,
}

impl SerializableExpression {
    pub fn new(expression: Ref<'_, Expression<VarId, f64>>) -> Self {
        let (quadratic, quadratic_count, quadratic_values) = Self::build_quadratic(&expression);
        let (higher_order, higher_order_values) = Self::build_higher_order(&expression);

        Self {
            active_variables: expression.active.clone(),
            num_variables: expression.num_variables() as u64,
            offset: expression.offset,
            linear: expression.linear.to_vec().clone(),
            quadratic,
            quadratic_count,
            quadratic_values,
            higher_order,
            higher_order_values,
        }
    }

    fn build_quadratic(
        expression: &Ref<'_, Expression<VarId, f64>>,
    ) -> (Vec<u32>, Vec<u32>, Vec<f64>) {
        let mut qs: Vec<u32> = Vec::new();
        let mut qc: Vec<u32> = Vec::new();
        let mut qvs: Vec<f64> = Vec::new();

        // expression
        //     .quadratic
        //     .as_ref()
        //     .map(|q| {
        //         q.iter_flat_positioned()
        //             .map(|((u_idx, v_idx), u, v, b)| {
        //                 SerializableTwoVarTerm::new(u_idx as u64, v_idx as u64, u.0, v.0, b)
        //             })
        //             .collect::<Vec<SerializableTwoVarTerm>>()
        //     })
        //     .map_or(None, |e| Some(SerializableQuadratic::new(e)))
        (qs, qc, qvs)
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
