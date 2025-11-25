mod access;
mod creation;
mod deepclone;
mod equality;

pub mod term;

use crate::ArcEnv;
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
