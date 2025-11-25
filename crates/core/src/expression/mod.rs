mod access;
mod creation;
mod deepclone;
mod equality;

pub mod term;

use crate::{ArcEnv, traits::{DefaultEditable, Editable}};
use lunamodel_types::Bias;
use std::fmt::Debug;
use term::{HigherOrder, Linear, Quadratic};

/// A mathematical Expression of arbitrary degree.
#[derive(Debug, Clone, Default)]
pub struct Expression {
    /// The [Environment] as an [Arc<RwLock<_>>].
    pub env: ArcEnv,
    /// The constant offset ([Bias]).
    pub offset: Bias,
    /// The [Linear] terms of this [Expression].
    pub linear: Linear,
    /// The [Quadratic] terms of this [Expression].
    pub quadratic: Option<Quadratic>,
    /// The [HigherOrder] terms of this [Expression].
    pub higher_order: Option<HigherOrder>,
    /// The number of variables in this [Expression].
    pub num_vars: usize,
}
impl Editable for Expression {}

impl From<Bias> for Expression {
    fn from(bias: Bias) -> Self {
        Self::with(|e| e.offset = bias)
    }
}

impl From<usize> for Expression {
    fn from(bias: usize) -> Self {
        Self::with(|e| e.offset = bias as Bias)
    }
}

impl From<Linear> for Expression {
    fn from(lin: Linear) -> Self {
        Self::with(|e| e.linear = lin)
    }
}

impl From<Quadratic> for Expression {
    fn from(q: Quadratic) -> Self {
        Self::with(|e| e.quadratic = Some(q))
    }
}

impl From<HigherOrder> for Expression {
    fn from(ho: HigherOrder) -> Self {
        Self::with(|e| e.higher_order = Some(ho))
    }
}
