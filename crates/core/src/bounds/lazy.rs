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
    /// Missing endpoints fall back to the type-specific defaults. `NaN` is
    /// rejected, and `-inf` as lower or `+inf` as upper bound means
    /// [`Bound::Unbounded`]. Everything else (`lower > upper`, fractional
    /// bounds on integers, ...) is model data and passed through to the solver.
    fn concretize(self, vtype: &Vtype) -> LunaModelResult<Bounds> {
        match (&vtype, &self) {
            (Vtype::Binary | Vtype::Spin, Some(_)) => Err(LunaModelError::InvalidBounds(
                format!("cannot set bounds for vtype {vtype}").into(),
            )),
            _ => Ok(()),
        }?;
        let default_bounds = Bounds::default_for(vtype);
        let Some(LazyBounds { lower, upper }) = self else {
            return Ok(default_bounds);
        };
        let lower = match lower {
            Some(Bound::Bounded(l)) if l == f64::NEG_INFINITY => Bound::Unbounded,
            Some(l) => check_not_nan(l, "lower")?,
            None => default_bounds.lower,
        };
        let upper = match upper {
            Some(Bound::Bounded(u)) if u == f64::INFINITY => Bound::Unbounded,
            Some(u) => check_not_nan(u, "upper")?,
            None => default_bounds.upper,
        };
        Ok(Bounds::new(lower, upper))
    }
}

/// Rejects `NaN` endpoints, which have no meaning as a bound.
fn check_not_nan(bound: Bound, which: &str) -> LunaModelResult<Bound> {
    match bound {
        Bound::Bounded(b) if b.is_nan() => Err(LunaModelError::InvalidBounds(
            format!("{which} bound cannot be NaN").into(),
        )),
        _ => Ok(bound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concretize(lower: Option<f64>, upper: Option<f64>, vtype: Vtype) -> LunaModelResult<Bounds> {
        Some(LazyBounds::new(
            lower.map(Bound::Bounded),
            upper.map(Bound::Bounded),
        ))
        .concretize(&vtype)
    }

    #[test]
    fn nan_is_rejected() {
        for vtype in [Vtype::Real, Vtype::Integer] {
            assert!(concretize(Some(f64::NAN), None, vtype).is_err());
            assert!(concretize(None, Some(f64::NAN), vtype).is_err());
        }
    }

    #[test]
    fn infinity_outward_is_unbounded() -> LunaModelResult<()> {
        for vtype in [Vtype::Real, Vtype::Integer] {
            let b = concretize(Some(f64::NEG_INFINITY), Some(f64::INFINITY), vtype)?;
            assert_eq!(b, Bounds::new(Bound::Unbounded, Bound::Unbounded));
        }
        Ok(())
    }

    #[test]
    fn infinity_inward_is_kept() -> LunaModelResult<()> {
        let b = concretize(Some(f64::INFINITY), Some(f64::NEG_INFINITY), Vtype::Real)?;
        assert_eq!(
            b,
            Bounds::new(
                Bound::Bounded(f64::INFINITY),
                Bound::Bounded(f64::NEG_INFINITY)
            )
        );
        Ok(())
    }

    #[test]
    fn solver_side_cases_pass_through() -> LunaModelResult<()> {
        let b = concretize(Some(0.5), Some(0.29 * 100.0), Vtype::Integer)?;
        assert_eq!(
            b,
            Bounds::new(Bound::Bounded(0.5), Bound::Bounded(0.29 * 100.0))
        );
        let b = concretize(Some(5.0), Some(1.0), Vtype::Real)?;
        assert_eq!(b, Bounds::new(Bound::Bounded(5.0), Bound::Bounded(1.0)));
        Ok(())
    }

    #[test]
    fn missing_side_uses_default() -> LunaModelResult<()> {
        let b = concretize(None, Some(-3.0), Vtype::Integer)?;
        assert_eq!(b, Bounds::new(Bound::Bounded(0.0), Bound::Bounded(-3.0)));
        Ok(())
    }
}
