use crate::core::solution::timing::Timing;
use crate::core::traits::{ContentEquality, FilterByMask};
use crate::core::writer::SolutionWriter;
use crate::core::{ResultIterator, ResultView, Samples, Sense, SharedEnvironment, VarRef, Vtype};
use crate::errors::{
    ComputationErr, SampleColCreationErr, SampleIncompatibleVtypeErr, SampleIncorrectLengthErr,
    SolutionCreationErr,
};
use crate::types::{
    Bias, BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
    VarIndex,
};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use itertools::Itertools;
use num::{NumCast, ToPrimitive};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Mul;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum VarAssignment {
    Binary(BinaryAssignmentType),
    Spin(SpinAssignmentType),
    Integer(IntegerAssignmentType),
    Real(RealAssignmentType),
}

#[derive(Debug, Clone, Copy)]
pub enum PrintLayout {
    Row,
    Col,
}

#[derive(Debug, Clone, Copy)]
pub enum ShowMetadata {
    Before,
    After,
    Hide,
}

impl VarAssignment {
    pub fn to_bias(&self) -> Bias {
        match self {
            VarAssignment::Binary(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Spin(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Integer(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Real(col) => <Bias as NumCast>::from(*col).unwrap(),
        }
    }
}

impl Default for VarAssignment {
    fn default() -> Self {
        VarAssignment::Binary(BinaryAssignmentType::default())
    }
}

impl Display for VarAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VarAssignment::Binary(x) => write!(f, "{x}"),
            VarAssignment::Spin(x) => write!(f, "{x}"),
            VarAssignment::Integer(x) => write!(f, "{x}"),
            VarAssignment::Real(x) => write!(f, "{x:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleColElement<T> {
    pub data: Vec<T>,
    pub varid: VarIndex,
}

impl<T> SampleColElement<T> {
    pub fn new(varid: VarIndex, data: Vec<T>) -> Self {
        Self { data, varid }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> IntoIterator for SampleColElement<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Clone> FilterByMask<T> for SampleColElement<T> {
    fn filter_by_mask(&self, mask: &Vec<bool>) -> Vec<T> {
        self.data.filter_by_mask(mask)
    }
}

/// The different assignments to a variable in the single samples
#[derive(Debug, Clone, PartialEq)]
pub enum SampleCol {
    Binary(SampleColElement<BinaryAssignmentType>),
    Spin(SampleColElement<SpinAssignmentType>),
    Integer(SampleColElement<IntegerAssignmentType>),
    Real(SampleColElement<RealAssignmentType>),
}

impl SampleCol {
    pub fn new<N: NumCast + Copy>(
        data: &[N],
        varid: VarIndex,
        vtype: Vtype,
    ) -> Result<SampleCol, SampleColCreationErr> {
        match vtype {
            Vtype::Binary => Ok(SampleCol::Binary(Self::make_samplecol_elem(varid, data)?)),
            Vtype::Spin => Ok(SampleCol::Spin(Self::make_samplecol_elem(varid, data)?)),
            Vtype::Real => Ok(SampleCol::Real(Self::make_samplecol_elem(varid, data)?)),
            Vtype::Integer => Ok(SampleCol::Integer(Self::make_samplecol_elem(varid, data)?)),
            Vtype::__Ghost => Err(SampleColCreationErr::new(
                "cannot create a sample column for ghost variables.",
            )),
        }
    }

    fn make_samplecol_elem<T: NumCast, N: NumCast + Copy>(
        varid: VarIndex,
        data: &[N],
    ) -> Result<SampleColElement<T>, SampleColCreationErr> {
        Ok(SampleColElement::new(
            varid,
            Self::make_sample_col_element_contents(data)?,
        ))
    }

    fn make_sample_col_element_contents<T: NumCast, N: NumCast + Copy>(
        data: &[N],
    ) -> Result<Vec<T>, SampleColCreationErr> {
        data.iter()
            .map(|e| <T as NumCast>::from(*e).ok_or_else(|| SampleColCreationErr::default()))
            .collect()
    }
}

impl Mul<Bias> for VarAssignment {
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

impl SampleCol {
    pub fn push<N: ToPrimitive>(
        &mut self,
        assignment: N,
    ) -> Result<(), SampleIncompatibleVtypeErr> {
        match self {
            Self::Binary(xs) => match <BinaryAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Spin(xs) => match <SpinAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Integer(xs) => match <IntegerAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Real(xs) => match <RealAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
        };
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<VarAssignment> {
        match self {
            Self::Binary(col) => col.get(index).map(|&x| VarAssignment::Binary(x)),
            Self::Spin(col) => col.get(index).map(|&x| VarAssignment::Spin(x)),
            Self::Integer(col) => col.get(index).map(|&x| VarAssignment::Integer(x)),
            Self::Real(col) => col.get(index).map(|&x| VarAssignment::Real(x)),
        }
    }

    pub fn as_vec(&self) -> Vec<VarAssignment> {
        // todo: do this without `collect` instead, and use some other return typle like `impl Iter`
        match self {
            SampleCol::Binary(bins) => bins.iter().map(|&x| VarAssignment::Binary(x)).collect(),
            SampleCol::Spin(spins) => spins.iter().map(|&x| VarAssignment::Spin(x)).collect(),
            SampleCol::Integer(ints) => ints.iter().map(|&x| VarAssignment::Integer(x)).collect(),
            SampleCol::Real(reals) => reals.iter().map(|&x| VarAssignment::Real(x)).collect(),
        }
    }
}

/// The solutions object for AQMs. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
#[derive(Debug, Clone, Default)]
pub struct Solution {
    /// A collection of samples. Each inner vec corresponds to all assignments to a single variable
    /// across different samples. `samples.len()` can be expected to always correspond exactly to
    /// the number of results available in the solution.
    pub samples: Vec<SampleCol>,
    /// How often each result occurs in the solution. `counts.len()` can be expected to
    /// always be equal to `samples.len()`
    pub counts: Vec<usize>,
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
    /// Boolean flag for each sample whether it's feasible, i.e., whether all bounds are satisfied.
    /// May be empty for solutions that haven't yet been evaluated.
    pub variable_bounds: Vec<Option<Vec<bool>>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub feasible: Vec<Option<bool>>,
    /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    /// of a lhs.
    pub best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Keeps track of the current number of unique samples.
    pub n_samples: usize,
    /// The names of all variables present in the solution.
    pub variable_names: Vec<String>,
    /// The model's optimization sense the solution was created with.
    pub sense: Sense,

    pub index_map: HashMap<usize, usize>,
}

impl Solution {
    pub fn with_sense(sense: Sense) -> Solution {
        let mut out = Self::default();
        out.sense = sense;
        out
    }

    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn add_column(&mut self, col: SampleCol) {
        self.samples.push(col);
    }

    pub fn create_columns(&mut self, env: &SharedEnvironment, capacity: usize) {
        let mut running_index = 0;
        for (idx, v) in env.borrow().all_variables().enumerate() {
            match v.vtype {
                Vtype::Binary => {
                    self.add_column(SampleCol::Binary(SampleColElement::new(
                        idx.into(),
                        Vec::with_capacity(capacity),
                    )));
                    if running_index != idx {
                        self.index_map.insert(idx, running_index);
                    }
                    running_index += 1;
                }
                Vtype::Spin => {
                    self.add_column(SampleCol::Spin(SampleColElement::new(
                        idx.into(),
                        Vec::with_capacity(capacity),
                    )));
                    if running_index != idx {
                        self.index_map.insert(idx, running_index);
                    }
                    running_index += 1;
                }
                Vtype::Integer => {
                    self.add_column(SampleCol::Integer(SampleColElement::new(
                        idx.into(),
                        Vec::with_capacity(capacity),
                    )));
                    if running_index != idx {
                        self.index_map.insert(idx, running_index);
                    }
                    running_index += 1
                }
                Vtype::Real => {
                    self.add_column(SampleCol::Real(SampleColElement::new(
                        idx.into(),
                        Vec::with_capacity(capacity),
                    )));
                    if running_index != idx {
                        self.index_map.insert(idx, running_index);
                    }
                    running_index += 1
                }
                Vtype::__Ghost => (),
            }
        }
    }

    pub fn map_varidx(&self, varidx: usize) -> usize {
        match self.index_map.get(&varidx) {
            Some(mapped) => *mapped,
            None => varidx,
        }
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// This method does not check whether the sample is already part of the solution as for now the
    /// solution translator is expected to do the aggregation.
    pub fn extend<S: Copy + NumCast, E: Copy + NumCast>(
        &mut self,
        sample: &Vec<S>,
        counts: usize,
        energy: Option<E>,
    ) -> Result<&mut Self, SolutionCreationErr> {
        self.add_sample(sample)?;
        self.counts.push(counts);
        self.raw_energies
            .push(energy.and_then(|e| <Bias as NumCast>::from(e)));
        self.obj_values.push(None);
        self.constraints.push(None);
        self.variable_bounds.push(None);
        self.feasible.push(None);
        self.n_samples += 1;
        Ok(self)
    }

    fn add_sample<T: Copy + NumCast>(
        &mut self,
        sample: &Vec<T>,
    ) -> Result<(), SolutionCreationErr> {
        if sample.len() != self.samples.len() {
            Err(SampleIncorrectLengthErr)?
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
        constraints: Vec<bool>,
        variable_bounds: Vec<bool>,
    ) {
        self.obj_values[sample_idx] = obj_value;
        if self.feasible.len() != self.n_samples {
            self.feasible = vec![None; self.n_samples]
        }
        if self.variable_bounds.len() != self.n_samples {
            self.variable_bounds = vec![None; self.n_samples]
        }
        if self.constraints.len() != self.n_samples {
            self.constraints = vec![None; self.n_samples]
        }
        self.variable_bounds[sample_idx] = Some(variable_bounds.clone());
        self.constraints[sample_idx] = Some(constraints.clone());
        self.feasible[sample_idx] =
            Some(constraints.iter().all(|&b| b) && variable_bounds.iter().all(|&b| b));
        let curr_sample_feasible = self.feasible[sample_idx].is_some_and(|b| b);
        match self.best_sample_idx {
            None => {
                if curr_sample_feasible && obj_value.is_some() {
                    self.best_sample_idx = Some(sample_idx)
                }
            }
            Some(i) => {
                if let (Some(old), Some(new)) = (self.obj_values[i], obj_value) {
                    if new < old && self.sense == Sense::Min && curr_sample_feasible
                        || new > old && self.sense == Sense::Max && curr_sample_feasible
                    {
                        self.best_sample_idx = Some(sample_idx);
                    }
                }
            }
        }
    }

    pub fn get_assignment(&self, row_idx: usize, col_idx: usize) -> Option<VarAssignment> {
        self.samples.get(col_idx).and_then(|col| col.get(row_idx))
    }

    pub fn best(&self) -> Option<ResultView> {
        self.best_sample_idx
            .map(|idx| ResultView::new(SharedSolution(Rc::new(RefCell::new(self.clone()))), idx))
    }

    pub fn filter_samples(&self, mask: &Vec<bool>) -> Self {
        if self.n_samples != mask.len() {
            panic!(
                "Filter sample should only be called internally and provide mask with correct len"
            )
        }
        let mut sol = Self::default();
        sol.samples = self
            .samples
            .iter()
            .map(|col| match col {
                SampleCol::Binary(b) => {
                    SampleCol::Binary(SampleColElement::new(b.varid, b.data.filter_by_mask(mask)))
                }
                SampleCol::Spin(s) => {
                    SampleCol::Spin(SampleColElement::new(s.varid, s.filter_by_mask(mask)))
                }
                SampleCol::Integer(i) => {
                    SampleCol::Integer(SampleColElement::new(i.varid, i.filter_by_mask(mask)))
                }
                SampleCol::Real(r) => {
                    SampleCol::Real(SampleColElement::new(r.varid, r.filter_by_mask(mask)))
                }
            })
            .collect();
        sol.sense = self.sense;
        sol.timing = self.timing;
        sol.variable_names = self.variable_names.clone();
        sol.counts = self.counts.filter_by_mask(mask);
        sol.obj_values = self.obj_values.filter_by_mask(mask);
        sol.raw_energies = self.raw_energies.filter_by_mask(mask);
        sol.constraints = self.constraints.filter_by_mask(mask);
        sol.variable_bounds = self.variable_bounds.filter_by_mask(mask);
        sol.feasible = self.feasible.filter_by_mask(mask);
        sol.n_samples = sol.counts.len();
        sol.ensure_best_sample_idx();
        sol
    }

    fn ensure_best_sample_idx(&mut self) {
        self.best_sample_idx = self.feasible.iter().zip(&self.obj_values).enumerate().fold(
            None,
            |acc, (idx, (&feas, &obj))| match (acc, feas, obj) {
                (None, Some(_), Some(_)) => Some(idx),
                (Some(a), Some(f), Some(o)) => {
                    let best_obj = self.obj_values[a].unwrap();
                    if f && (self.sense == Sense::Min && o < best_obj
                        || self.sense == Sense::Max && o > best_obj)
                    {
                        Some(idx)
                    } else {
                        acc
                    }
                }
                (a, _, _) => a,
            },
        )
    }
}

pub enum VarKey<'a> {
    Name(String),
    Var(&'a VarRef),
}

impl Solution {
    pub fn add_samplecol<N: NumCast + Copy>(
        &mut self,
        var: VarKey,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), SampleColCreationErr> {
        match var {
            VarKey::Name(varname) => self.add_samplecol_for_varname(varname, data, vtype),
            VarKey::Var(var) => self.add_samplecol_for_var(&var, data, vtype),
        }
    }

    pub fn add_samplecol_for_var<N: NumCast + Copy>(
        &mut self,
        var: &VarRef,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), SampleColCreationErr> {
        let varname = var
            .env
            .borrow()
            .get_for_index(var.id)
            .map_err(|e| SampleColCreationErr::new(&e.to_string()))?
            .name
            .clone();
        self.index_map.insert(var.id.into(), self.samples.len());
        self.variable_names.push(varname);
        self.add_column(SampleCol::new(data, var.id, vtype)?);
        // todo: adjust other values and fix logic after restructuring the solution
        // internally.
        Ok(())
    }
    pub fn add_samplecol_for_varname<N: NumCast + Copy>(
        &mut self,
        varname: String,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), SampleColCreationErr> {
        let varid = self.variable_names.len();
        self.index_map.insert(varid, self.samples.len());
        self.add_column(SampleCol::new(
            data,
            varid.into(),
            vtype,
        )?);
        self.variable_names.push(varname);
        // todo: adjust other values and fix logic after restructuring the solution
        // internally.
        Ok(())
    }

    pub fn remove_samplecol(&mut self, var: VarKey) {
        match var {
            VarKey::Var(var) => self.remove_samplecol_for_var(var),
            VarKey::Name(varname) => self.remove_samplecol_for_varname(varname),
        }
    }

    pub fn remove_samplecol_for_var(&mut self, var: &VarRef) {
        let env = var.env.borrow();
        let variable = env.get_for_index(var.id);
        match variable {
            Ok(variable) => {
                let id: usize = var.id.into();
                self.index_map.remove(&id);
                self.remove_samplecol_for_varname(variable.name.clone())
            },
            Err(_) => (),
        }
    }

    pub fn remove_samplecol_for_varname(&mut self, varname: String) {
        let index = self.variable_names.iter().find_position(|&n| *n == varname);
        match index {
            Some((idx, _)) => {
                self.index_map.remove(&idx);
                let _ = self.variable_names.remove(idx);
                let _ = self.samples.remove(idx);
            },
            None => (),
        };
    }
}

// Convenience functions
impl Solution {
    pub fn expectation_value(&self) -> Result<Bias, ComputationErr> {
        // equivalent to doing np.average(solution.obj_values, weights=solution.counts)
        let mut weight_sum: f64 = 0.0;
        let mut weighted_sum: Bias = Bias::default();

        for (idx, (&ov, &c)) in self.obj_values.iter().zip(&self.counts).enumerate() {
            if ov.is_none() {
                return Err(ComputationErr(format!(
                    "obj_values contains a 'None' value at position '{idx}'."
                )));
            }
            let obj_val = ov.unwrap();
            weight_sum += c as f64;
            weighted_sum += obj_val * c as f64;
        }

        Ok(weighted_sum / weight_sum)
    }

    pub fn feasibility_ratio(&self) -> Result<Bias, ComputationErr> {
        let mut n_feasible = 0;
        let mut n_total = 0;

        for (idx, (&feas, &c)) in self.feasible.iter().zip(&self.counts).enumerate() {
            if feas.is_none() {
                return Err(ComputationErr(format!(
                    "feasible contains a 'None' value at position '{idx}'."
                )));
            }
            if feas.unwrap() {
                n_feasible += c;
            }
            n_total += c;
        }

        Ok(n_feasible as f64 / n_total as f64)
    }

    pub fn highest_constraint_violations(&self) -> Result<Option<usize>, ComputationErr> {
        let mut n_violations = vec![0; self.constraints.len()];
        for (idx, (satisfied, &count)) in self.constraints.iter().zip(&self.counts).enumerate() {
            if satisfied.is_none() {
                return Err(ComputationErr(format!(
                    "feasible contains a 'None' value at position '{idx}'."
                )));
            }
            satisfied
                .as_ref()
                .unwrap()
                .iter()
                .zip(&mut n_violations)
                .filter(|(&sat, _)| !sat)
                .for_each(|(_, n)| *n += count)
        }

        Ok(n_violations
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .map(|(idx, _)| idx))
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct SharedSolution(pub Rc<RefCell<Solution>>);

impl SharedSolution {
    pub fn from(sol: Solution) -> Self {
        Self(Rc::new(RefCell::new(sol)))
    }
}

impl SharedSolution {
    pub fn get_result_view(&self, row_idx: usize) -> Option<ResultView> {
        if row_idx >= self.borrow().n_samples {
            None
        } else {
            Some(ResultView::new(self.clone(), row_idx))
        }
    }

    pub fn iter_results(&self) -> ResultIterator {
        ResultIterator::new(SharedSolution::clone(&self))
    }

    pub fn samples(&self) -> Samples {
        Samples(SharedSolution::clone(&self))
    }

    pub fn best(&self) -> Option<ResultView> {
        self.borrow()
            .best_sample_idx
            .map(|idx| ResultView::new(self.clone(), idx))
    }
}

impl Clone for SharedSolution {
    fn clone(&self) -> Self {
        SharedSolution(Rc::clone(&self.0))
    }
}

impl Into<Rc<RefCell<Solution>>> for SharedSolution {
    fn into(self) -> Rc<RefCell<Solution>> {
        self.0
    }
}

impl PartialEq for SharedSolution {
    fn eq(&self, other: &Self) -> bool {
        let lhs = &self.borrow();
        let rhs = &other.borrow();

        lhs.samples == rhs.samples
            && lhs.counts == rhs.counts
            && lhs.obj_values == rhs.obj_values
            && lhs.raw_energies == rhs.raw_energies
            && lhs.constraints == rhs.constraints
            && lhs.variable_bounds == rhs.variable_bounds
            && lhs.feasible == rhs.feasible
            && lhs.best_sample_idx == rhs.best_sample_idx
            && lhs.timing == rhs.timing
            && lhs.n_samples == rhs.n_samples
            && lhs.variable_names == rhs.variable_names
            && lhs.sense == rhs.sense
    }
}

impl Display for SharedSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_solution(SharedSolution::clone(&self))
            .to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Solution {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.samples == other.samples
            && self.counts == other.counts
            && self.obj_values == other.obj_values
            && self.raw_energies == other.raw_energies
            && self.constraints == other.constraints
            && self.variable_bounds == other.variable_bounds
            && self.feasible == other.feasible
            && self.best_sample_idx == other.best_sample_idx
            && self.timing == other.timing
            && self.n_samples == other.n_samples
            && self.variable_names == other.variable_names
    }
}
