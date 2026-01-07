use std::ops::Index;

use indexmap::IndexMap;
use itertools::Itertools;

use crate::solution::Column;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Samples(pub IndexMap<String, Column>);

impl Samples {
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Column> {
        self.0.get_mut(key)
    }

    pub fn insert(&mut self, key: String, value: Column) -> Option<Column> {
        self.0.insert(key, value)
    }

    pub fn variable_names(&self) -> Vec<String> {
        self.0.keys().cloned().collect_vec()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Column)> {
        self.0.iter()
    }
}

impl Index<&str> for Samples {
    type Output = Column;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

impl From<IndexMap<String, Column>> for Samples {
    fn from(value: IndexMap<String, Column>) -> Self {
        Self(value)
    }
}
