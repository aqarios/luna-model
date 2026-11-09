//! The parsed form of a model file, shared by the LP and MPS readers.

use std::{borrow::Cow, collections::HashMap};

use lunamodel_error::LunaModelError;
use lunamodel_types::{Comparator, Sense, Vtype};

/// A parsed model file. Variables are referred to by id: their index in `vars`.
#[derive(Default)]
pub struct Problem<'a> {
    pub name: Option<&'a str>,
    pub sense: Sense,
    pub vars: Vec<&'a str>, // in order of first appearance
    pub objective: Expr,
    pub constraints: Vec<Constraint<'a>>,
    pub bounds: Vec<(Option<f64>, Option<f64>)>, // per variable, as written
    pub vtypes: Vec<Vtype>,                      // per variable
    ids: HashMap<&'a str, usize>,
}

impl<'a> Problem<'a> {
    /// The id of variable `name`; a new name gets the next id.
    pub fn var_id(&mut self, name: &'a str) -> usize {
        let Self {
            ids,
            vars,
            bounds,
            vtypes,
            ..
        } = self;
        *ids.entry(name).or_insert_with(|| {
            vars.push(name);
            bounds.push((None, None));
            vtypes.push(Vtype::Real);
            vars.len() - 1
        })
    }
}

#[derive(Default, Clone)]
pub struct Expr {
    pub offset: f64,
    pub linear: Vec<(usize, f64)>,
    pub quadratic: Vec<(usize, usize, f64)>,
}

impl Expr {
    /// Appends `other`, multiplied by `factor`.
    pub fn add_scaled(&mut self, other: Expr, factor: f64) {
        self.offset += factor * other.offset;
        let linear = other.linear.into_iter().map(|(v, c)| (v, factor * c));
        let quadratic = other
            .quadratic
            .into_iter()
            .map(|(u, v, c)| (u, v, factor * c));
        self.linear.extend(linear);
        self.quadratic.extend(quadratic);
    }
}

#[derive(Clone)]
pub struct Constraint<'a> {
    pub name: Option<Cow<'a, str>>,
    pub lhs: Expr,
    pub comparator: Comparator,
    pub rhs: f64,
}

/// Translation error that names the line it happened on.
pub fn error(line: usize, msg: String) -> LunaModelError {
    LunaModelError::Translation(format!("line {line}: {msg}").into())
}
