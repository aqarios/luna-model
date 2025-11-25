mod deepclone;
mod creation;
mod equality;
mod access;

pub mod term;

use crate::ArcEnv;
use lunamodel_types::Bias;
use std::fmt::Debug;
use term::{HigherOrder, Linear, Quadratic};

/// A mathematical Expression of arbitrary degree.
#[derive(Debug, Clone, Default)]
pub struct Expression {
    /// The [Environment] as an [Arc<RwLock<_>>].
    env: ArcEnv,
    /// The constant offset ([Bias]).
    offset: Bias,
    /// The [Linear] terms of this [Expression].
    linear: Linear,
    /// The [Quadratic] terms of this [Expression].
    quadratic: Option<Quadratic>,
    /// The [HigherOrder] terms of this [Expression].
    higher_order: Option<HigherOrder>,
    /// The number of variables in this [Expression].
    num_vars: usize,
}
