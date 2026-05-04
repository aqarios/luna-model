//! Lightweight variable references bound to a shared environment.

use std::{
    fmt::{Debug, Display},
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{bounds::Bounds, environment::ArcEnv, traits::ContentEquality};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{VarIdx, Vtype};

use super::var::Variable;

/// Cheap handle to a variable stored inside an [`ArcEnv`].
///
/// `VarRef` is the type that expressions and most APIs pass around. It carries a
/// variable index plus the shared environment needed to resolve metadata on
/// demand. This keeps symbolic structures compact while still allowing rich
/// inspection when needed.
#[derive(Clone)]
pub struct VarRef {
    pub(crate) id: VarIdx,
    /// Shared environment that owns the referenced variable.
    pub env: ArcEnv,
}

impl VarRef {
    /// Creates a new variable reference from a raw environment index.
    pub fn new(id: VarIdx, env: ArcEnv) -> VarRef {
        Self { id, env }
    }

    /// Returns the raw variable index inside the environment.
    pub fn id(&self) -> VarIdx {
        self.id
    }

    /// Verifies that the referenced variable still exists.
    ///
    /// This is useful because `VarRef` values can outlive variable removal from
    /// the environment. Many methods perform the same lookup implicitly, but
    /// `check_living` makes the liveness check explicit at call sites that want
    /// to fail early.
    pub fn check_living(&self) -> LunaModelResult<()> {
        _ = self.env.read_arc().get(self.id)?;
        Ok(())
    }

    /// Returns the current variable name.
    pub fn name(&self) -> LunaModelResult<String> {
        Ok(self.env.read_arc().get(self.id)?.name.0.clone())
    }

    /// Returns the variable type.
    pub fn vtype(&self) -> LunaModelResult<Vtype> {
        Ok(self.env.read_arc().get(self.id)?.vtype)
    }

    /// Returns a copy of the concrete bounds stored on the variable.
    pub fn bounds(&self) -> LunaModelResult<Bounds> {
        Ok(self.env.read_arc().get(self.id)?.bounds)
    }

    /// Returns the inverted companion variable if one exists.
    pub fn inverted(&self) -> LunaModelResult<Option<VarRef>> {
        Ok(self
            .env
            .read_arc()
            .get(self.id)?
            .inverted
            .map(|id| VarRef::new(id, self.env.clone())))
    }

    /// Returns a stable hash derived from the variable name.
    ///
    /// The hash intentionally uses the resolved name rather than the raw index so
    /// it remains meaningful across equivalent environments.
    pub fn hash(&self) -> LunaModelResult<u64> {
        let mut state = DefaultHasher::new();
        self.name()?.hash(&mut state);
        Ok(state.finish())
    }
}

impl PartialEq for VarRef {
    /// Compares the variable index and the semantic contents of the environment.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.env.equal_contents(&other.env)
    }
}

impl Display for VarRef {
    /// Formats the variable by name when it is still alive in the environment.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(name) = self.name() {
            f.write_str(&format!("Var({})", name))
        } else {
            f.write_str("<deleted>")
        }
    }
}

impl Debug for VarRef {
    /// Formats a compact debug view with raw id and environment id.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Var(id={}, env_id={})",
            self.id,
            self.env.read_arc().id
        ))
    }
}

impl TryInto<Variable> for &VarRef {
    type Error = LunaModelError;

    /// Materializes the referenced environment entry as an owned [`Variable`].
    fn try_into(self) -> Result<Variable, Self::Error> {
        Ok(self.env.read_arc().get(self.id)?.clone())
    }
}
