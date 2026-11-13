//! Model inspection helpers shared by the reader tests.
#![allow(dead_code)] // every test file uses a different subset

use lunamodel_core::{Expression, Model};
use lunamodel_types::{Bound, Comparator, Sense, Vtype};

pub fn bounds(model: &Model, name: &str) -> (Bound, Bound) {
    let b = model.environment.lookup(name).unwrap().bounds().unwrap();
    (b.lower, b.upper)
}

pub fn vtype(model: &Model, name: &str) -> Vtype {
    model.environment.lookup(name).unwrap().vtype().unwrap()
}

pub fn var_names(model: &Model) -> Vec<String> {
    let vars = model.environment.vars();
    vars.iter().map(|v| v.name().unwrap()).collect()
}

/// Terms as `("x", c)` or `("x*y", c)`, sorted by name; the constant is `("", c)`.
pub fn terms(expr: &Expression) -> Vec<(String, f64)> {
    let mut terms: Vec<_> = expr
        .items()
        .map(|(vars, c)| {
            let names: Vec<_> = vars.iter().map(|v| v.name().unwrap()).collect();
            (names.join("*"), c)
        })
        .collect();
    terms.sort_by(|a, b| a.0.cmp(&b.0));
    terms
}

pub fn objective(model: &Model) -> Vec<(String, f64)> {
    terms(model.objective())
}

/// `(name, comparator, rhs)` for every constraint, in model order.
pub fn constraints(model: &Model) -> Vec<(String, Comparator, f64)> {
    let constraints = model.constraints().iter();
    constraints
        .map(|(name, c)| (name.to_string(), c.comparator, c.rhs))
        .collect()
}

/// Builds `[(name, value)]` with owned names, for comparing against [`terms`].
pub fn expected(items: &[(&str, f64)]) -> Vec<(String, f64)> {
    items.iter().map(|&(n, c)| (n.to_string(), c)).collect()
}

/// `(name, comparator, rhs, lhs terms)` of a constraint.
pub type ConstraintContents = (String, Comparator, f64, Vec<(String, f64)>);

/// Everything a model file can express, for comparing a model with its round trip.
#[derive(Debug, PartialEq)]
pub struct Contents {
    pub sense: Sense,
    pub vars: Vec<(String, Vtype, (Bound, Bound))>,
    pub objective: Vec<(String, f64)>,
    pub constraints: Vec<ConstraintContents>,
}

pub fn contents(model: &Model) -> Contents {
    let names = var_names(model);
    let vars = names
        .into_iter()
        .map(|n| {
            let (vtype, bounds) = (vtype(model, &n), bounds(model, &n));
            (n, vtype, bounds)
        })
        .collect();
    let constraints = model.constraints().iter();
    let constraints = constraints
        .map(|(name, c)| (name.to_string(), c.comparator, c.rhs, terms(&c.lhs)))
        .collect();
    Contents {
        sense: model.sense,
        vars,
        objective: objective(model),
        constraints,
    }
}
