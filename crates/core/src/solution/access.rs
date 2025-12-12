use std::ops::Index;

use lunamodel_types::Bias;

use super::{Solution, result::ResultView, sample::SampleView};

impl Solution {
    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn best(&self) -> Option<ResultView<'_>> {
        unimplemented!()
    }

    pub fn assignment(&self, sample: usize, var: &str) -> Option<&Bias> {
        match sample >= self.len() {
            true => None,
            false => Some(&self[(sample, var)]),
        }
    }

    pub fn result(&self, index: usize) -> Option<ResultView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    // pub fn results(&self) -> impl Iterator<Item = ResultView<'_>> {
    //     unimplemented!()
    // }

    pub fn sample(&self, index: usize) -> Option<SampleView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    // pub fn samples(&self) -> impl Iterator<Item = SampleView<'_>> {
    //     unimplemented!()
    // }
}

impl Index<(usize, &str)> for Solution {
    type Output = Bias;

    fn index(&self, index: (usize, &str)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}
