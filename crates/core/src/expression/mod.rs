//! Symbolic expressions over LunaModel variables.
//!
//! Expressions are split into linear, quadratic, and higher-order term storage.
//! That representation reflects the operations the rest of the workspace cares
//! about: fast access to common low-degree forms while still supporting
//! arbitrary-degree algebra when needed.

mod access;
mod creation;
mod deepclone;
mod equality;

pub mod term;

use crate::{
    ArcEnv,
    traits::{DefaultEditable, Editable},
};
use lunamodel_types::Bias;
use std::fmt::Debug;
use term::{HigherOrder, Linear, Quadratic};

/// A symbolic expression over variables in an [`ArcEnv`].
///
/// The struct keeps separate storage for linear, quadratic, and higher-order
/// terms because much of the project needs to inspect or translate those cases
/// differently. The `offset` stores the constant term.
#[derive(Clone, Default)]
pub struct Expression {
    /// Shared environment backing all variable references in the expression.
    pub env: ArcEnv,
    /// The constant offset ([Bias]).
    pub offset: Bias,
    /// The [Linear] terms of this [Expression].
    pub linear: Linear,
    /// The [Quadratic] terms of this [Expression].
    pub quadratic: Option<Quadratic>,
    /// The [HigherOrder] terms of this [Expression].
    pub higher_order: Option<HigherOrder>,
}
impl Editable for Expression {}

impl Expression {
    /// Creates an empty expression bound to the given environment.
    ///
    /// The environment matters because every variable reference inside the
    /// expression is resolved through it. Expressions created from different
    /// environments may look structurally similar while still referring to
    /// different variables.
    pub fn new(env: ArcEnv) -> Self {
        Self {
            env,
            ..Default::default()
        }
    }
}

impl From<Bias> for Expression {
    /// Creates a constant expression from a bias value.
    fn from(bias: Bias) -> Self {
        Self::with(|e| e.offset = bias)
    }
}

impl From<usize> for Expression {
    /// Creates a constant expression from an unsigned integer.
    fn from(bias: usize) -> Self {
        Self::with(|e| e.offset = bias as Bias)
    }
}

impl From<Linear> for Expression {
    /// Wraps an already-built linear term collection as an expression.
    fn from(lin: Linear) -> Self {
        Self::with(|e| e.linear = lin)
    }
}

impl From<Quadratic> for Expression {
    /// Wraps an already-built quadratic term collection as an expression.
    fn from(q: Quadratic) -> Self {
        Self::with(|e| e.quadratic = Some(q))
    }
}

impl From<HigherOrder> for Expression {
    /// Wraps an already-built higher-order term collection as an expression.
    fn from(ho: HigherOrder) -> Self {
        Self::with(|e| e.higher_order = Some(ho))
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression")
            .field("envidx", &self.env.read_arc().id)
            .field("offset", &self.offset)
            .field("linear", &self.linear)
            .field("quadratic", &self.quadratic)
            .field("higher_order", &self.higher_order)
            .field("num_vars", &self.num_vars())
            .finish()
    }
}
