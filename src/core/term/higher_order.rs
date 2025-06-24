use crate::types::{Bias, VarIndex};
use hashbrown::{hash_map::Iter, HashMap};
use itertools::Itertools;
use std::{
    ops::{Index, IndexMut, MulAssign, Neg},
    str::FromStr,
};

static DELIMITER: &str = "-";

#[derive(Clone, Debug)]
pub struct HigherOrder {
    pub biases: HashMap<String, Bias>,
    default_bias: Bias,
}

impl HigherOrder {
    pub fn default() -> Self {
        Self {
            biases: HashMap::default(),
            default_bias: Bias::default(),
        }
    }

    pub fn new(biases: HashMap<String, Bias>) -> Self {
        Self {
            biases,
            default_bias: Bias::default(),
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            biases: HashMap::with_capacity(size),
            default_bias: Bias::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    pub fn make_key(index: &Vec<VarIndex>) -> String {
        let mut indices = index.clone();
        indices.sort();
        indices
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    }

    fn key_contributions(key: &String) -> Vec<VarIndex> {
        key.split(DELIMITER)
            .map(|s| VarIndex::from_str(s).ok().unwrap())
            .collect()
        // ok().unwrap() instead of unwrap() to get rid of the error for now. needs
        // fixing ??
    }

    pub fn is_empty(&self) -> bool {
        self.biases.len() == 0
    }

    pub fn iter(&self) -> Iter<String, Bias> {
        self.biases.iter()
    }

    pub fn iter_contrib(&self) -> impl Iterator<Item = (Vec<VarIndex>, &Bias)> {
        self.biases
            .iter()
            .map(|(key, bias)| (HigherOrder::key_contributions(&key), bias))
    }

    pub fn resize(&mut self, _: usize) {}

    pub fn cleanup(&mut self) {
        self.biases.retain(|_, bias| *bias != Bias::default());
    }
}

impl MulAssign<Bias> for HigherOrder {
    fn mul_assign(&mut self, rhs: Bias) {
        for (_, value) in self.biases.iter_mut() {
            *value *= rhs;
        }
    }
}

impl Index<&Vec<VarIndex>> for HigherOrder {
    type Output = Bias;
    fn index(&self, index: &Vec<VarIndex>) -> &Self::Output {
        let key = Self::make_key(index);
        self.biases.get(&key).unwrap_or(&self.default_bias)
    }
}

impl IndexMut<&Vec<VarIndex>> for HigherOrder {
    fn index_mut(&mut self, index: &Vec<VarIndex>) -> &mut Self::Output {
        let key = Self::make_key(index);
        if !self.biases.contains_key(&key) {
            self.biases.insert(key.to_string(), Bias::default());
        }
        self.biases.get_mut(&key).unwrap()
    }
}

impl Index<&String> for HigherOrder {
    type Output = Bias;

    fn index(&self, index: &String) -> &Self::Output {
        self.biases.get(index).unwrap_or(&self.default_bias)
    }
}

impl IndexMut<&String> for HigherOrder {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        if !self.biases.contains_key(index) {
            self.biases.insert(index.to_string(), Bias::default());
        }
        self.biases.get_mut(index).unwrap()
    }
}

impl PartialEq for HigherOrder {
    fn eq(&self, other: &Self) -> bool {
        let mut all_keys = self.biases.keys().collect_vec();
        all_keys.append(&mut other.biases.keys().collect_vec());
        for &key in all_keys.iter() {
            if self[key] != other[key] {
                return false;
            }
        }
        true
    }
}

impl HigherOrder {
    fn negate(&self) -> Self {
        HigherOrder::new(
            self.biases
                .iter()
                .map(|(key, value)| (key.clone(), -*value))
                .collect(),
        )
    }
}

impl Neg for HigherOrder {
    type Output = HigherOrder;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &HigherOrder {
    type Output = HigherOrder;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
