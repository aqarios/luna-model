use std::ops::Index;

use lunamodel_types::Bias;

use super::{Solution, result::ResultView, sample::SampleView};

const RTOL: f64 = f64::EPSILON;
const ATOL: f64 = f64::EPSILON;

fn is_close(a: f64, b: f64, rtol: f64, atol: f64) -> bool {
    let diff = (a - b).abs();
    if diff <= atol {
        return true;
    }

    let largest = a.abs().max(b.abs());
    diff <= largest * rtol
}

impl Solution {
    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn best(&self) -> Option<Vec<ResultView<'_>>> {
        match (&self.feasible, &self.obj_values) {
            (Some(f), Some(ov)) => {
                let min = *ov
                    .iter()
                    .zip(f)
                    .filter(|(_, f)| **f)
                    .map(|(a, _)| a)
                    .min_by(|&a, &b| a.total_cmp(b))
                    .unwrap();
                let allmins: Vec<usize> = ov
                    .iter()
                    .zip(f)
                    .enumerate()
                    .filter(|(_, (_, f))| **f)
                    .map(|(idx, (val, _))| (idx, val))
                    .filter(|(_, val)| is_close(**val, min, RTOL, ATOL))
                    .map(|(idx, _)| idx)
                    .collect();
                let views = allmins
                    .iter()
                    .map(|idx| ResultView::new(&self, *idx))
                    .collect();
                Some(views)
            }
            _ => None,
        }
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

    pub fn results<'s>(&'s self) -> impl Iterator<Item = ResultView<'s>> {
        (0..self.len()).map(move |i| (self, i).into())
    }

    pub fn sample(&self, index: usize) -> Option<SampleView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    // pub fn samples(&self) -> impl Iterator<Item = SampleView<'_>> {
    //     unimplemented!()
    // }

    pub fn variable_names(&self) -> Vec<String> {
        self.samples.keys().cloned().collect()
    }
}

impl Index<(usize, &str)> for Solution {
    type Output = Bias;

    fn index(&self, index: (usize, &str)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}
