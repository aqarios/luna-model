use std::ops::{Index, IndexMut};

pub struct Linear<Bias> {
    biases: Vec<Bias>,
}

impl<Bias> Linear<Bias>
where
    Bias: Default + Clone,
{
    pub fn default() -> Self {
        Self { biases: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    pub fn resize(&mut self, new_len: usize) {
        self.biases.resize(new_len, Bias::default());
    }
}

impl<Bias> From<&Vec<Bias>> for Linear<Bias>
where
    Bias: Clone,
{
    fn from(value: &Vec<Bias>) -> Self {
        Self {
            biases: value.to_vec(),
        }
    }
}

impl<Bias> Index<usize> for Linear<Bias> {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        &self.biases[index]
    }
}

impl<Bias> IndexMut<usize> for Linear<Bias> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.biases[index]
    }
}
