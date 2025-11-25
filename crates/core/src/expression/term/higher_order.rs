use std::ops::{AddAssign, Index, IndexMut, MulAssign, Neg};

use hashbrown::HashMap;
use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};

static SEP: &str = "-";

#[derive(Debug, Clone)]
pub struct HigherOrder {
    entries: HashMap<String, Bias>,
}

impl HigherOrder {
    pub fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
            || self.entries.iter().map(|(_, b)| b).sum::<Bias>() == Bias::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, Bias)> {
        self.entries.iter().map(|(k, b)| (k, *b))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Bias)> {
        self.entries.iter_mut()
    }

    pub fn iter_contrib(&self) -> impl Iterator<Item = (Vec<VarIdx>, Bias)> {
        self.entries.iter().map(|(k, b)| (contribs(k), *b))
    }

    pub fn clean(&mut self) {
        self.entries.retain(|_, b| *b != Bias::default());
    }

    pub fn degree(&self) -> usize {
        self.iter_contrib().map(|(k, _)| k.len()).max().unwrap()
    }
}

impl MulAssign<Bias> for HigherOrder {
    fn mul_assign(&mut self, rhs: Bias) {
        for (_, bias) in self.iter_mut() {
            *bias *= rhs;
        }
    }
}

impl Index<&[VarIdx]> for HigherOrder {
    type Output = Bias;
    fn index(&self, index: &[VarIdx]) -> &Self::Output {
        &self[&key(index.to_vec())]
    }
}

impl IndexMut<&[VarIdx]> for HigherOrder {
    fn index_mut(&mut self, index: &[VarIdx]) -> &mut Self::Output {
        &mut self[&key(index.to_vec())]
    }
}

impl Index<&String> for HigherOrder {
    type Output = Bias;
    fn index(&self, index: &String) -> &Self::Output {
        self.entries.get(index).unwrap_or_else(|| &DEFAULT_BIAS)
    }
}

impl IndexMut<&String> for HigherOrder {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        if !self.entries.contains_key(index) {
            self.entries.insert(index.clone(), Bias::default());
        }
        self.entries.get_mut(index).unwrap()
    }
}

impl Neg for HigherOrder {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            entries: self.entries.into_iter().map(|(k, b)| (k, -b)).collect(),
        }
    }
}

impl PartialEq for HigherOrder {
    fn eq(&self, other: &Self) -> bool {
        let mut all: Vec<_> = self.entries.keys().collect();
        all.append(&mut other.entries.keys().collect());
        for &k in all.iter() {
            if self[k] != other[k] {
                return false;
            }
        }
        true
    }
}

fn contribs(str: &String) -> Vec<VarIdx> {
    str.split(SEP)
        .map(|s| s.parse::<VarIdx>().unwrap())
        .collect()
}

fn key(mut indices: Vec<VarIdx>) -> String {
    indices.sort();
    indices
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(SEP)
}

impl AddAssign<&HigherOrder> for HigherOrder {
    fn add_assign(&mut self, rhs: &HigherOrder) {
        for (contrib, bias) in rhs.iter() {
            self[contrib] += bias;
        }
    }
}

impl AddAssign<HigherOrder> for HigherOrder {
    fn add_assign(&mut self, rhs: HigherOrder) {
        self.add_assign(&rhs);
    }
}
