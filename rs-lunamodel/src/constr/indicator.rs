use crate::{Result, VarRef, traits::LmTryInto};

use super::Constraint;

/// A constraint that only has to hold for one value of a binary variable.
///
/// Reads as `variable == value => constraint`.
#[derive(Debug)]
pub struct Indicator {
    /// The binary variable the constraint depends on.
    pub variable: VarRef,
    /// The value of `variable` for which `constraint` has to hold.
    pub value: bool,
    /// The constraint that has to hold while `variable` equals `value`.
    pub constraint: Constraint,
}

impl Constraint {
    /// Reads this constraint as an indicator constraint, if it has that form.
    ///
    /// A constraint whose terms all share a common binary factor is read as an
    /// [`Indicator`] on that binary. The inner constraint has the factor removed.
    ///
    /// Returns `Ok(None)` if the constraint does not have indicator form.
    pub fn indicator(&self) -> Result<Option<Indicator>> {
        let (mut variable, mut constraint) = (std::ptr::null_mut(), std::ptr::null_mut());
        let mut value = false;
        check!(constr_indicator(@self, &mut variable, &mut value, &mut constraint))?;
        // Both handles are wrapped before either is dropped, so neither leaks.
        let variable: Option<VarRef> = variable.lm_try_into()?;
        let constraint: Option<Constraint> = constraint.lm_try_into()?;
        Ok(variable
            .zip(constraint)
            .map(|(variable, constraint)| Indicator {
                variable,
                value,
                constraint,
            }))
    }

    /// Builds the constraint that only has to hold while the binary `z` equals
    /// `value` (`true` for 1, `false` for 0). An inverted binary stands for the
    /// other value of its base. [`indicator`](Self::indicator) reads it back.
    pub fn when(&self, z: &VarRef, value: bool) -> Result<Constraint> {
        out!(constr_when(@self, @z, value))
    }
}
