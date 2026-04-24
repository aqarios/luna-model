//! Lazily validated bounds representation.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Vtype};

use crate::bounds::Bounds;

#[derive(Clone, Copy, Debug, Default)]
pub struct LazyBounds {
    /// Optional lower bound override.
    pub lower: Option<Bound>,
    /// Optional upper bound override.
    pub upper: Option<Bound>,
}

/// Converts partially specified bounds into concrete bounds for a variable type.
///
/// The concretization step is where type-specific validation happens. For
/// example, binary and spin variables reject explicit bounds because their
/// domains are fixed by definition.
pub trait Concretize {
    fn concretize(self, vtype: &Vtype) -> LunaModelResult<Bounds>;
}

impl LazyBounds {
    /// Creates a partially specified bounds object.
    pub fn new(lower: Option<Bound>, upper: Option<Bound>) -> Self {
        Self { lower, upper }
    }

    /// Returns the optional lower-bound override.
    pub fn lower(&self) -> Option<Bound> {
        self.lower
    }

    /// Returns the optional upper-bound override.
    pub fn upper(&self) -> Option<Bound> {
        self.upper
    }
}

impl Concretize for Option<LazyBounds> {
    /// Resolves optional caller-supplied bounds against the variable type defaults.
    ///
    /// Missing endpoints fall back to the type-specific defaults. Integer
    /// bounds are additionally checked for integral endpoints.
    fn concretize(self, vtype: &Vtype) -> LunaModelResult<Bounds> {
        match (&vtype, &self) {
            (Vtype::Binary | Vtype::Spin, Some(_)) => Err(LunaModelError::InvalidBounds(
                format!("cannot set bounds for vtype {vtype}").into(),
            )),
            _ => Ok(()),
        }?;
        let default_bounds = Bounds::default_for(vtype);
        let bounds = self.map_or(default_bounds, |b| match (b.lower, b.upper) {
            (Some(l), Some(u)) => Bounds::new(l, u),
            (Some(l), None) => Bounds::new(l, default_bounds.upper),
            (None, Some(u)) => Bounds::new(default_bounds.lower, u),
            (None, None) => default_bounds,
        });
        if *vtype == Vtype::Integer {
            check_integer_bounds(bounds)?;
        }
        Ok(bounds)
    }
}

/// Validates that bounded integer endpoints are integral.
fn check_integer_bounds(bounds: Bounds) -> Result<(), LunaModelError> {
    let maybeok = match bounds {
        Bounds {
            lower: Bound::Bounded(lower),
            upper: Bound::Bounded(upper),
        } => {
            let lpf = lower.fract() != 0.0;
            let upf = upper.fract() != 0.0;
            match (lpf, upf) {
                (true, true) => Err("lower and upper"),
                (true, false) => Err("lower"),
                (false, true) => Err("upper"),
                (false, false) => Ok(()),
            }
        }
        Bounds {
            lower: Bound::Unbounded,
            upper: Bound::Bounded(upper),
        } => {
            if upper.fract() != 0.0 {
                Err("upper")
            } else {
                Ok(())
            }
        }
        Bounds {
            lower: Bound::Bounded(lower),
            upper: Bound::Unbounded,
        } => {
            if lower.fract() != 0.0 {
                Err("upper")
            } else {
                Ok(())
            }
        }
        Bounds {
            lower: Bound::Unbounded,
            upper: Bound::Unbounded,
        } => Ok(()),
    };
    maybeok.map_err(|e| {
        LunaModelError::InvalidBounds(format!("Invalid {e} bound for integer variable.").into())
    })
}
