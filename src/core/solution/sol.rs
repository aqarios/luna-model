use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::timing::Timing;
use num::{NumCast, ToPrimitive};
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub enum VarAssignment<AssignmentTypes>
where
    AssignmentTypes: AssignmentBaseTypes,
{
    Binary(AssignmentTypes::BinaryType),
    Spin(AssignmentTypes::SpinType),
    Integer(AssignmentTypes::IntegerType),
    Real(AssignmentTypes::RealType),
}

/// The different assignments to a variable in the single samples
#[derive(Debug, Clone)]
pub enum SampleCol<AssignmentTypes>
where
    AssignmentTypes: AssignmentBaseTypes,
{
    Binary(Vec<AssignmentTypes::BinaryType>),
    Spin(Vec<AssignmentTypes::SpinType>),
    Integer(Vec<AssignmentTypes::IntegerType>),
    Real(Vec<AssignmentTypes::RealType>),
}

impl<Bias, AssignmentTypes> Mul<Bias> for VarAssignment<AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Output = Bias;

    fn mul(self, rhs: Bias) -> Self::Output {
        match self {
            VarAssignment::Binary(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Spin(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Integer(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Real(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
        }
    }
}

impl<AssignmentTypes> SampleCol<AssignmentTypes>
where
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn push<N: ToPrimitive>(&mut self, assignment: N) -> Result<(), ()> {
        match self {
            Self::Binary(xs) => {
                xs.push(<AssignmentTypes::BinaryType as NumCast>::from(assignment).unwrap());
                Ok(())
            }
            Self::Spin(xs) => {
                xs.push(<AssignmentTypes::SpinType as NumCast>::from(assignment).unwrap());
                Ok(())
            }
            Self::Integer(xs) => {
                xs.push(<AssignmentTypes::IntegerType as NumCast>::from(assignment).unwrap());
                Ok(())
            }
            Self::Real(xs) => {
                xs.push(<AssignmentTypes::RealType as NumCast>::from(assignment).unwrap());
                Ok(())
            }
        }
    }

    pub fn get<Bias: BiasConstraints>(
        &self,
        index: usize,
    ) -> Option<VarAssignment<AssignmentTypes>> {
        match self {
            Self::Binary(col) => col.get(index).map(|&x| VarAssignment::Binary(x)),
            Self::Spin(col) => col.get(index).map(|&x| VarAssignment::Spin(x)),
            Self::Integer(col) => col.get(index).map(|&x| VarAssignment::Integer(x)),
            Self::Real(col) => col.get(index).map(|&x| VarAssignment::Real(x)),
        }
    }
}

/// The solutions object for AQMs. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
#[derive(Debug, Clone, Default)]
pub struct Solution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// A collection of samples. Each inner vec corresponds to all assignments to a single variable
    /// across different samples. `samples.len()` can be expected to always correspond exactly to
    /// the number of results available in the solution.
    pub samples: Vec<SampleCol<AssignmentTypes>>,
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
    pub constraints: Vec<Vec<bool>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub feasible: Vec<bool>,
    // /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    // /// of a lhs.
    pub best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Private attribute to keep track of the current number of samples
    n_samples: usize,
}

impl<Bias, AssignmentTypes> Solution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn len(&self) -> usize {
        self.n_samples
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// This method does not check whether the sample is already part of the solution as for now the
    /// solution translator is expected to do the aggregation.
    pub fn extend<T: Copy + NumCast>(
        &mut self,
        sample: Vec<T>,
        num_occurrences: usize,
    ) -> Result<&mut Self, ()> {
        self.add_sample(sample)?;
        self.num_occurrences.push(num_occurrences);
        Ok(self)
    }

    fn add_sample<T: Copy + NumCast>(&mut self, sample: Vec<T>) -> Result<(), ()> {
        if sample.len() != self.samples.len() {
            Err(())
        } else {
            for (i, &a) in sample.iter().enumerate() {
                self.samples[i].push(a)?;
            }
            Ok(())
        }
    }

    pub fn get_assignment<Idx>(&self, row: Idx, col: Idx) -> Option<VarAssignment<AssignmentTypes>>
    where
        Idx: IndexConstraints,
    {
        self.samples
            .get(col.into())
            .and_then(|col| col.get::<Bias>(row.into()))
    }
}
