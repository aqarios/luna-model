use hashbrown::HashMap;

use super::sol::Solution;
use crate::{
    core::{
        solution::{
            result::ResultView,
            sample::{SampleView, Samples, SamplesIterator},
        },
        VarAssignment,
    },
    types::VarIndex,
};

impl Solution {
    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn best(&self) -> Option<ResultView> {
        self.best_sample_idx.map(|idx| ResultView::new(self, idx))
    }

    pub fn get_result_view(&self, idx: usize) -> Option<ResultView> {
        if idx >= self.n_samples {
            None
        } else {
            Some(ResultView::new(self, idx))
        }
    }

    pub fn get_sample_view(&self, idx: usize) -> Option<SampleView> {
        if idx >= self.n_samples {
            None
        } else {
            Some(SampleView::new(self, idx))
        }
    }

    pub fn get_assignment(&self, row: usize, col: usize) -> Option<VarAssignment> {
        self.samples.get(col).and_then(|column| column.get(row))
    }

    pub fn iter_samples(&self) -> SamplesIterator {
        SamplesIterator::new(self)
    }

    pub fn samples(&self) -> Samples {
        Samples(&self)
    }
}

impl Solution {
    pub fn varname_to_pos(&self) -> HashMap<String, VarIndex> {
        let mut map = HashMap::with_capacity(self.variable_names.len());
        for (i, vname) in self.variable_names.iter().enumerate() {
            map.insert(vname.to_string(), i.into());
        }
        map
    }

    pub fn var_indices(&self) -> Vec<VarIndex> {
        self.samples.iter().map(|col| col.var_index()).collect()
    }
}
