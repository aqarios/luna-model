use crate::{core::ConcreteEnvId, errors::VariableCreationErr};
use std::fmt::{Debug, Display, Formatter};

use super::{bounds::display_bound, Bounds, Vtype};

/// The variable of a model containing it's name, type, bounds and the environment association.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub vtype: Vtype,
    pub bounds: Bounds,
    pub env_id: ConcreteEnvId,
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
            env_id: ConcreteEnvId::default(),
        }
    }
    /// Create a new variable based on the name, optional type and bounds and an environment
    /// id.
    pub fn new(
        name: String,
        vtype: Option<&Vtype>,
        bounds: Option<Bounds>,
        env_id: ConcreteEnvId,
    ) -> Result<Self, VariableCreationErr> {
        let vtype = vtype.map_or(Vtype::default(), |t| *t);
        match (vtype, bounds.is_some()) {
            (Vtype::Binary, true) | (Vtype::Spin, true) => Err(VariableCreationErr::new(format!(
                "bounds cannot be set for variable of type {}.",
                vtype
            ))),
            _ => Ok(()),
        }?;
        let default_bounds = Bounds::default(&vtype);
        let bounds = bounds.map_or(default_bounds, |b| match (b.lower, b.upper) {
            (Some(_), Some(_)) => b,
            (Some(l), None) => Bounds::new(Some(l), default_bounds.upper),
            (None, Some(u)) => Bounds::new(default_bounds.lower, Some(u)),
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
            let default = Bounds::default(&self.vtype);
            let has_lower = self.bounds.lower != default.lower;
            let has_upper = self.bounds.upper != default.upper;
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
