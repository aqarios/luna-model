use crate::core::expression::BiasConstraints;
use crate::core::solution::base::{AssignmentBaseTypes, AssignmentConstraints};
use crate::core::solution::res::ResultView;
use crate::core::solution::timing::Timing;
use crate::core::ConcreteBias;
use std::fmt::Binary;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub enum VarAssignment<Assignment>
where
    Assignment: AssignmentBaseTypes,
{
    Binary(Assignment::BinaryType),
    Spin(Assignment::SpinType),
    Integer(Assignment::IntegerType),
    Real(Assignment::RealType),
}

impl<Assignment> VarAssignment<Assignment>
where
    Assignment: AssignmentBaseTypes,
{
    #[inline]
    fn extract_inner(&self) -> &dyn AssignmentConstraints {
        match self {
            VarAssignment::Binary(x) => x,
            VarAssignment::Spin(x) => x,
            VarAssignment::Integer(x) => x,
            VarAssignment::Real(x) => x,
        }
    }
}

impl<Assignment> Mul for VarAssignment<Assignment>
where
    Assignment: AssignmentBaseTypes,
{
    type Output = ConcreteBias;

    fn mul(self, rhs: ConcreteBias) -> Self::Output {
        match self {
            VarAssignment::Binary(x) => x.into() * rhs,
            VarAssignment::Spin(x) => x.into() * rhs,
            VarAssignment::Integer(x) => x.into() * rhs,
            VarAssignment::Real(x) => x.into() * rhs,
        }
    }
}

pub type Sample<Assignment: AssignmentBaseTypes> = Vec<VarAssignment<Assignment>>;

/// The different assignments to a variable in the single samples
#[derive(Debug, Clone)]
pub enum VarAssignments<Assignment>
where
    Assignment: AssignmentBaseTypes,
{
    Binaries(Vec<Assignment::BinaryType>),
    Spins(Vec<Assignment::SpinType>),
    Integers(Vec<Assignment::IntegerType>),
    Reals(Vec<Assignment::RealType>),
}

impl<Assignment: AssignmentBaseTypes> VarAssignments<Assignment> {
    pub fn push(&mut self, assignment: VarAssignment<Assignment>) -> Result<(), ()> {
        match (self, assignment) {
            (VarAssignments::Binaries(xs), VarAssignment::Binary(x)) => {
                xs.push(x);
                Ok(())
            }
            (VarAssignments::Spins(xs), VarAssignment::Spin(x)) => {
                xs.push(x);
                Ok(())
            }
            (VarAssignments::Integers(xs), VarAssignment::Integer(x)) => {
                xs.push(x);
                Ok(())
            }
            (VarAssignments::Reals(xs), VarAssignment::Real(x)) => {
                xs.push(x);
                Ok(())
            }
            (_, _) => Err(()),
        }
    }
    pub fn push2(&mut self, assignment: VarAssignment<Assignment>) -> Result<(), ()> {
        todo!()
    }

    #[inline]
    fn extract_inner(&self) -> &Vec<dyn AssignmentConstraints> {
        match self {
            VarAssignments::Binaries(xs) => xs,
            VarAssignments::Spins(xs) => xs,
            VarAssignments::Integers(xs) => xs,
            VarAssignments::Reals(xs) => xs,
        }
    }

    #[inline]
    fn map_to_var_assignment(&self, x: &dyn AssignmentConstraints) -> VarAssignment<Assignment> {
        match self {
            VarAssignments::Binaries(_) => VarAssignment::Binary(x),
            VarAssignments::Spins(_) => VarAssignment::Spin(x),
            VarAssignments::Integers(_) => VarAssignment::Integer(x),
            VarAssignments::Reals(_) => VarAssignment::Real(x),
        }
    }

    // pub fn get(&self, index: usize) -> Option<VarAssignment<Assignment>> {
    //     match self {
    //         VarAssignments::Binaries(xs) => match xs.get(index) {
    //             None => None,
    //             Some(&x) => Some(VarAssignment::Binary(x)),
    //         },
    //         VarAssignments::Spins(xs) => match xs.get(index) {
    //             None => None,
    //             Some(&x) => Some(VarAssignment::Spin(x)),
    //         },
    //         VarAssignments::Integers(xs) => match xs.get(index) {
    //             None => None,
    //             Some(&x) => Some(VarAssignment::Integer(x)),
    //         },
    //         VarAssignments::Reals(xs) => match xs.get(index) {
    //             None => None,
    //             Some(&x) => Some(VarAssignment::Real(x)),
    //         },
    //     }
    // }

    pub fn get(&self, index: usize) -> Option<VarAssignment<Assignment>> {
        self.extract_inner()
            .get(index)
            .map(|&x| self.map_to_var_assignment(x))
    }
}

/// The solutions object for AQMs. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
#[derive(Debug, Clone, Default)]
pub struct Solution<Bias, Assignment>
where
    Bias: BiasConstraints,
    Assignment: AssignmentBaseTypes,
{
    /// A collection of samples. Each inner vec corresponds to all assignments to a single variable
    /// across different samples. `samples.len()` can be expected to always correspond exactly to
    /// the number of results available in the solution.
    pub samples: Vec<VarAssignments<Assignment>>,
    /// How often each result occurs in the solution. `num_occurrences.len()` can be expected to
    /// always be equal to `samples.len()`
    pub num_occurrences: Vec<usize>,
    /// Objetive values as computed by the corresponding AQM. May be empty for solutions that
    /// haven't yet been evaluated.
    pub obj_values: Vec<Bias>,
    /// Objetive values as computed by the solver. May be empty if the solver does not provide
    /// energies in its solution format. May be different from `obj_values`, e.g., because an offset
    /// was neglected, or the AQM was transformed before being solved.
    pub raw_energies: Vec<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied. Each inner vec corresponds
    /// to one sample, i.e., `constraints[i]` corresponds to `samples[0]`. May be empty for
    /// solutions that haven't yet been evaluated.
    constraints: Vec<Vec<bool>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    feasible: Vec<bool>,
    // /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    // /// of a lhs.
    best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Private attribute to keep track of the current number of samples
    n_samples: usize,
}

impl<Bias, Assignment> Solution<Bias, Assignment>
where
    Bias: BiasConstraints,
    Assignment: AssignmentBaseTypes,
{
    pub fn position(&self, sample: &Sample<Assignment>) -> Option<usize> {
        // TODO: check whether this approach is more efficient than creating a temp HashMap
        self.samples.iter().position(|x| x == sample)
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    pub fn extend(
        &mut self,
        sample: Sample<Assignment>,
        num_occurrences: usize,
    ) -> Result<&mut Self, ()> {
        if let Some(idx) = self.position(&sample) {
            self.num_occurrences[idx] += num_occurrences;
        } else {
            self.add_sample(sample)?;
            self.num_occurrences.push(num_occurrences);
        }
        Ok(self)
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// In contrast to `extend`, this method does not check whether the sample is already part of
    /// the solution.
    pub fn extend_no_agg(
        &mut self,
        sample: Sample<Assignment>,
        num_occurrences: usize,
    ) -> Result<&mut Self, ()> {
        self.add_sample(sample)?;
        self.num_occurrences.push(num_occurrences);
        Ok(self)
    }

    fn add_sample(&mut self, sample: Sample<Assignment>) -> Result<(), ()> {
        if sample.len() != self.samples.len() {
            Err(())
        } else {
            for (i, a) in sample.iter().enumerate() {
                self.samples[i].push(*a);
            }
            Ok(())
        }
    }

    /// Iterate over the single results of the solution
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ResultView<Assignment, Bias>> + use<'_, Assignment, Bias> {
        (0..self.samples.len()).map(|i| self.get_result(i).unwrap())
    }

    pub fn get_result(&self, index: usize) -> Option<ResultView<Assignment, Bias>> {
        if index >= self.n_samples {
            return None;
        }
        let sample: Sample<_> = self.samples.iter().map(|x| x.get(index).unwrap()).collect();

        let obj_value = match (self.obj_values.get(index), self.raw_energies.get(index)) {
            (Some(&bias), _) => Some(bias),
            (_, Some(&bias)) => Some(bias),
            (_, _) => None,
        };
        let constraints = self.constraints.get(index);
        let feasible = self.feasible.get(index).map(|&b| b);

        Some(ResultView::new(sample, obj_value, constraints, feasible))
    }
}
