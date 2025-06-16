use crate::{core::ContentEquality, errors::VariableCreationErr, types::EnvId};
use std::fmt::{Debug, Display, Formatter};

use super::{bounds::display_bound, Bounds, LazyBounds, Vtype};

/// The variable of a model containing it's name, type, bounds and the environment association.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub vtype: Vtype,
    pub bounds: Bounds,
    pub env_id: EnvId,
}

impl Variable {
    pub fn deep_clone(&self, id: EnvId) -> Self {
        Self {
            name: self.name.clone(),
            vtype: self.vtype,
            bounds: self.bounds.clone(),
            env_id: id,
        }
    }
}

impl Variable {
    /// Create a default variable.
    ///
    /// Currently, this creates an unnamed binary variable.
    /// Should not be used directly and should merely act as a placeholder.
    pub fn default() -> Self {
        Self {
            name: String::from(""),
            vtype: Vtype::default(),
            bounds: Bounds::default(&Vtype::default()),
            env_id: EnvId::default(),
        }
    }
    /// Create a new variable based on the name, optional type and bounds and an environment
    /// id.
    pub fn new(
        name: String,
        vtype: Option<Vtype>,
        bounds: Option<LazyBounds>,
        env_id: EnvId,
    ) -> Result<Self, VariableCreationErr> {
        let vtype = vtype.map_or(Vtype::default(), |t| t);
        match (vtype, bounds.is_some()) {
            (Vtype::Binary, true) | (Vtype::Spin, true) => {
                Err(VariableCreationErr::InvalidBounds(vtype))
            }
            _ => Ok(()),
        }?;
        let default_bounds = Bounds::default(&vtype);
        let bounds = bounds.map_or(default_bounds, |b| match (b.lower, b.upper) {
            (Some(l), Some(u)) => Bounds::new(l, u),
            (Some(l), None) => Bounds::new(l, default_bounds.upper),
            (None, Some(u)) => Bounds::new(default_bounds.lower, u),
            (None, None) => default_bounds,
        });
        Ok(Self {
            bounds,
            name,
            vtype,
            env_id,
        })
    }

    fn format_bounds(&self) -> String {
        let mut out = String::new();
        if matches!(self.vtype, Vtype::Integer | Vtype::Real) {
            let has_lower = self.bounds.lower.is_bounded();
            let has_upper = self.bounds.upper.is_bounded();
            if has_lower || has_upper {
                let mut bounds = vec![];
                if has_lower {
                    bounds.push(format!("lower: {}", display_bound(&self.bounds.lower)));
                }
                if has_upper {
                    bounds.push(format!("upper: {}", display_bound(&self.bounds.upper)));
                }
                out += &format!(" {{ {} }}", bounds.join(", "));
            }
        }
        out
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}{}", self.name, self.vtype, self.format_bounds())
    }
}

impl ContentEquality for Variable {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.name == other.name && self.vtype == other.vtype && self.bounds == other.bounds
    }
}
