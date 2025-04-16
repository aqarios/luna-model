use crate::core::expression::BiasConstraints;
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::timing::Timing;
use crate::core::{ResultIterator, ResultView, Samples};
use crate::errors::{IncorrectVtypeError, SampleIncorrectLengthError, SolutionCreatorErr};
use derive_more::{Deref, DerefMut};
use num::{NumCast, ToPrimitive};
use std::ops::Mul;
use std::rc::Rc;

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

impl<AssignmentTypes> Default for VarAssignment<AssignmentTypes>
where
    AssignmentTypes: AssignmentBaseTypes,
{
    fn default() -> Self {
        VarAssignment::Binary(AssignmentTypes::BinaryType::default())
    }
}

/// The different assignments to a variable in the single samples
#[derive(Debug, Clone, PartialEq)]
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
    pub fn push<N: ToPrimitive>(&mut self, assignment: N) -> Result<(), IncorrectVtypeError> {
        match self {
            Self::Binary(xs) => match <AssignmentTypes::BinaryType as NumCast>::from(assignment) {
                None => return Err(IncorrectVtypeError),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Spin(xs) => match <AssignmentTypes::SpinType as NumCast>::from(assignment) {
                None => return Err(IncorrectVtypeError),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Integer(xs) => {
                match <AssignmentTypes::IntegerType as NumCast>::from(assignment) {
                    None => return Err(IncorrectVtypeError),
                    Some(x) => {
                        xs.push(x);
                    }
                }
            }
            Self::Real(xs) => match <AssignmentTypes::RealType as NumCast>::from(assignment) {
                None => return Err(IncorrectVtypeError),
                Some(x) => {
                    xs.push(x);
                }
            },
        };
        Ok(())
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
    pub obj_values: Vec<Option<Bias>>,
    /// Objetive values as computed by the solver. May be empty if the solver does not provide
    /// energies in its solution format. May be different from `obj_values`, e.g., because an offset
    /// was neglected, or the AQM was transformed before being solved.
    pub raw_energies: Vec<Option<Bias>>,
    /// Boolean flag for each single constraint whether it's satisfied. Each inner vec corresponds
    /// to one sample, i.e., `constraints[i]` corresponds to `samples[i]`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub constraints: Vec<Option<Vec<bool>>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub feasible: Vec<Option<bool>>,
    // /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    // /// of a lhs.
    pub best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Keeps track of the current number of samples.
    pub n_samples: usize,
}

impl<Bias, AssignmentTypes> Solution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn add_column(&mut self, col: SampleCol<AssignmentTypes>) {
        self.samples.push(col);
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// This method does not check whether the sample is already part of the solution as for now the
    /// solution translator is expected to do the aggregation.
    pub fn extend<S: Copy + NumCast, E: Copy + NumCast>(
        &mut self,
        sample: Vec<S>,
        num_occurrences: usize,
        energy: Option<E>,
    ) -> Result<&mut Self, SolutionCreatorErr> {
        self.add_sample(sample)?;
        self.num_occurrences.push(num_occurrences);
        self.raw_energies
            .push(energy.and_then(|e| <Bias as NumCast>::from(e)));
        self.obj_values.push(None);
        self.constraints.push(None);
        self.feasible.push(None);
        self.n_samples += 1;
        Ok(self)
    }

    fn add_sample<T: Copy + NumCast>(&mut self, sample: Vec<T>) -> Result<(), SolutionCreatorErr> {
        if sample.len() != self.samples.len() {
            Err(SampleIncorrectLengthError)?
        } else {
            for (i, &a) in sample.iter().enumerate() {
                self.samples[i].push(a)?;
            }
            Ok(())
        }
    }

    pub fn add_sample_evaluation(
        &mut self,
        sample_idx: usize,
        obj_value: Option<Bias>,
        constraints: Option<Vec<bool>>,
        sense_is_minimize: bool,
    ) {
        self.obj_values[sample_idx] = obj_value;
        if let Some(constr) = constraints.as_ref() {
            if self.feasible.len() != self.n_samples {
                self.feasible = vec![None; self.n_samples]
            }
            if self.constraints.len() != self.n_samples {
                self.constraints = vec![None; self.n_samples]
            }
            self.feasible[sample_idx] = Some(constr.iter().all(|&b| b));
            self.constraints[sample_idx] = Some(constr.clone());
        }
        match self.best_sample_idx {
            None => {}
            Some(i) => match (self.obj_values[i], obj_value) {
                (Some(old), Some(new)) => {
                    if new < old && sense_is_minimize || new > old && !sense_is_minimize {
                        self.best_sample_idx = Some(sample_idx);
                    }
                }
                _ => {}
            },
        }
    }

    pub fn get_assignment(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> Option<VarAssignment<AssignmentTypes>> {
        self.samples
            .get(col_idx)
            .and_then(|col| col.get::<Bias>(row_idx))
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct RcSolution<Bias, AssignmentTypes>(pub Rc<Solution<Bias, AssignmentTypes>>)
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes;

impl<Bias, AssignmentTypes> RcSolution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn get_result_view(&self, row_idx: usize) -> Option<ResultView<Bias, AssignmentTypes>> {
        if row_idx >= self.0.n_samples {
            None
        } else {
            Some(ResultView::new(self.clone(), row_idx))
        }
    }

    pub fn iter_results(&self) -> ResultIterator<Bias, AssignmentTypes> {
        ResultIterator::new(RcSolution::clone(&self))
    }

    pub fn samples(&self) -> Samples<Bias, AssignmentTypes> {
        Samples(RcSolution::clone(&self))
    }
}

impl<Bias, AssignmentTypes> Clone for RcSolution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn clone(&self) -> Self {
        RcSolution(Rc::clone(&self.0))
    }
}

impl<Bias, AssignmentTypes> Into<Rc<Solution<Bias, AssignmentTypes>>>
    for RcSolution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn into(self) -> Rc<Solution<Bias, AssignmentTypes>> {
        self.0
    }
}

impl<Bias, AssignmentTypes> PartialEq for RcSolution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let lhs = &self.0;
        let rhs = &other.0;

        lhs.samples == rhs.samples
            && lhs.num_occurrences == rhs.num_occurrences
            && lhs.obj_values == rhs.obj_values
            && lhs.raw_energies == rhs.raw_energies
            && lhs.constraints == rhs.constraints
            && lhs.feasible == rhs.feasible
            && lhs.best_sample_idx == rhs.best_sample_idx
            && lhs.timing == rhs.timing
            && lhs.n_samples == rhs.n_samples
    }
}
