use crate::core::writer::SolutionWriter;
use crate::core::{
    RcSolution, ResultView, SampleIterator, SamplesIterator, ValueByIndex, VarAssignment,
};
use crate::types::VarIndex;
use derive_more::{Deref, DerefMut};
use either::{Either, Left, Right};
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Samples(pub RcSolution);

#[derive(Debug, Clone)]
pub struct OwnedSample {
    pub variable_names: Vec<String>,
    pub actual: Rc<Vec<VarAssignment>>, // todo: maybe remove this `RC`
}
impl OwnedSample {
    pub fn new(variable_names: Vec<String>, actual: Rc<Vec<VarAssignment>>) -> Self {
        Self {
            variable_names,
            actual,
        }
    }

    pub fn len(&self) -> usize {
        self.actual.len()
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Sample(pub Either<ResultView, OwnedSample>);

impl Samples {
    pub fn get_sample(&self, row_idx: usize) -> Option<Sample> {
        self.get_result_view(row_idx).map(|x| Sample(Left(x)))
    }

    pub fn get_assignment(&self, row_idx: usize, col_idx: usize) -> Option<VarAssignment> {
        self.0.get_assignment(row_idx, col_idx)
    }

    pub fn iter(&self) -> SamplesIterator {
        SamplesIterator::new(RcSolution::clone(&self.0))
    }
}

impl Sample {
    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment> {
        match &self.0 {
            Left(r) => r.get_assignment(col_idx),
            Right(r) => r.actual.get(col_idx).map(|&x| x),
        }
    }

    pub fn iter(&self) -> SampleIterator {
        SampleIterator::new(self.0.clone())
    }

    pub fn index_for_variable_name(&self, varname: &String) -> Option<usize> {
        match &self.0 {
            Left(rv) => rv.sol.variable_names.iter().position(|e| e == varname),
            Right(os) => os.variable_names.iter().position(|e| e == varname),
        }
    }

    pub fn to_map(&self) -> HashMap<String, VarAssignment> {
        match &self.0 {
            Left(rv) => rv
                .iter()
                .zip(rv.sol.variable_names.iter())
                .map(|(v, s)| (s.clone(), v.clone()))
                .collect(),
            Right(os) => os
                .actual
                .iter()
                .zip(os.variable_names.iter())
                .map(|(v, s)| (s.clone(), v.clone()))
                .collect(),
        }
    }
}

impl ValueByIndex<VarIndex> for Sample {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        match &self.0 {
            Left(r) => r.value_by_index(index),
            Right(s) => s.actual[<VarIndex as Into<usize>>::into(index)],
        }
    }
}

impl IntoIterator for Sample {
    type Item = VarAssignment;
    type IntoIter = SampleIterator;

    fn into_iter(self) -> Self::IntoIter {
        SampleIterator::new(self.0)
    }
}

impl Display for Samples {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_samples(self.clone(), &self.counts)
            .to_string();
        f.write_str(&s)
    }
}

impl Display for Sample {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new().write_sample(self.clone()).to_string();
        f.write_str(&s)
    }
}
