use std::fmt::{Display, Formatter};

use super::{SampleIterator, VarAssignment};
use crate::{
    core::{
        solution::{result::ResultView, sol::Solution},
        writer::SolutionWriter,
        ValueByIndex,
    },
    types::VarIndex,
};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;

pub enum Sample<'a> {
    View(SampleView<'a>),
    Owned(SampleOwned),
}

impl<'a> Sample<'a> {
    pub fn variable_names(&self) -> &[String] {
        match self {
            Self::View(view) => &view.sol.variable_names,
            Self::Owned(owned) => &owned.variable_names,
        }
    }

    /// The length of the sample is equal to the number of
    /// variable assignments or the number of variables in
    /// the solution, so also equal to the number of cols.
    pub fn len(&self) -> usize {
        match self {
            Self::View(view) => view.sol.samples.len(),
            Self::Owned(owned) => owned.actual.len(),
        }
    }

    pub fn iter(&'a self) -> SampleIterator<'a> {
        SampleIterator::new(self)
    }

    pub fn index_for_variable_name(&self, varname: &str) -> Option<usize> {
        match self {
            Self::View(view) => view.sol.variable_names.iter().position(|e| e == varname),
            Self::Owned(owned) => owned.variable_names.iter().position(|e| e == varname),
        }
    }

    pub fn to_map(&self) -> HashMap<String, VarAssignment> {
        match &self {
            Self::View(view) => self
                .iter()
                .zip(view.sol.variable_names.iter())
                .map(|(v, s)| (s.clone(), v.clone()))
                .collect(),
            Self::Owned(os) => os
                .actual
                .iter()
                .zip(os.variable_names.iter())
                .map(|(v, s)| (s.clone(), v.clone()))
                .collect(),
        }
    }

    // pub fn index_map(&self) -> HashMap<VarIndex, VarIndex> {
    //     match self {
    //         Self::View(view) => view.res.sol.varidx_to_pos(),
    //         Self::Owned(owned) => owned.index_map(),
    //     }
    // }
    pub fn varname_to_pos(&self) -> HashMap<String, VarIndex> {
        match self {
            Self::View(view) => view.res.sol.varname_to_pos(),
            Self::Owned(owned) => owned.varname_to_pos(),
        }
    }

    pub fn var_indices(&self) -> Vec<VarIndex> {
        match self {
            Self::View(view) => view.res.sol.var_indices(),
            Self::Owned(owned) => owned.var_indices.clone(),
        }
    }
}

impl<'a> ValueByIndex<VarIndex> for Sample<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        match self {
            Self::View(view) => view.value_by_index(index),
            Self::Owned(owned) => owned.value_by_index(index),
        }
    }
}

impl<'a> Display for Sample<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new().write_sample(self).to_string();
        f.write_str(&s)
    }
}

// VIEW
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SampleView<'a> {
    pub res: ResultView<'a>,
}

impl<'a> SampleView<'a> {
    pub fn new(sol: &'a Solution, row: usize) -> Self {
        Self {
            res: ResultView::new(sol, row),
        }
    }
}

impl<'a> ValueByIndex<VarIndex> for SampleView<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        self.res.value_by_index(index)
    }
}

// OWNED
#[derive(Debug, Clone)]
pub struct SampleOwned {
    pub variable_names: Vec<String>,
    pub actual: Vec<VarAssignment>,
    pub var_indices: Vec<VarIndex>,
}

impl SampleOwned {
    pub fn new(
        variable_names: Vec<String>,
        actual: Vec<VarAssignment>,
        var_indices: Vec<VarIndex>,
    ) -> Self {
        Self {
            variable_names,
            actual,
            var_indices,
        }
    }

    pub fn to_map(&self) -> HashMap<&String, &VarAssignment> {
        self.variable_names.iter().zip(&self.actual).collect()
    }

    pub fn varname_to_pos(&self) -> HashMap<String, VarIndex> {
        let mut map = HashMap::with_capacity(self.variable_names.len());
        for (i, name) in self.variable_names.iter().enumerate() {
            map.insert(name.to_string(), i.into());
        }
        map
    }
}

impl<'a> ValueByIndex<VarIndex> for SampleOwned {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        let idx: usize = index.into();
        self.actual[idx]
    }
}

impl Display for SampleOwned {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(&Sample::Owned(self.clone()))
            .to_string();
        f.write_str(&s)
    }
}
