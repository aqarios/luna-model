use crate::core::expression::BiasConstraints;
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::sol::Sample;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView<'a, Assignment, Bias>
where
    Assignment: AssignmentBaseTypes,
    Bias: BiasConstraints,
{
    /// The vector of variable assignments.
    pub sample: Sample<Assignment>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<&'a Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

impl<'a, Assignment, Bias> ResultView<'a, Assignment, Bias>
where
    Assignment: AssignmentBaseTypes,
    Bias: BiasConstraints,
{
    pub fn new(
        sample: Sample<Assignment>,
        obj_value: Option<Bias>,
        constraint_satisfaction: Option<&'a Vec<bool>>,
        feasible: Option<bool>,
    ) -> Self {
        Self {
            sample,
            obj_value,
            constraint_satisfaction,
            feasible,
        }
    }
}

pub struct OwnedResult<Assignment, Bias>
where
    Assignment: AssignmentBaseTypes,
    Bias: BiasConstraints,
{
    /// The vector of variable assignments.
    pub sample: Sample<Assignment>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}
