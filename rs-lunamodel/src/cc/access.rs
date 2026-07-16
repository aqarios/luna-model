use lunamodel_types::{Comparator, Vtype};

use crate::{Constraint, Result, TryIndex, VarRef};

use super::ConstraintCollection;

impl ConstraintCollection {
    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        out!(cc_vtypes(@self))
    }

    pub fn ctypes(&self) -> Result<Vec<Comparator>> {
        out!(cc_ctypes(@self))
    }

    pub fn vars(&self) -> Result<Vec<VarRef>> {
        out!(cc_vars(@self))
    }

    pub fn complete_vars(&self) -> Result<Vec<VarRef>> {
        out!(cc_complete_vars(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(cc_is_empty(@self))
    }

    pub fn len(&self) -> Result<usize> {
        out!(cc_len(@self))
    }

    pub fn get(&self, key: &str) -> Result<Constraint> {
        out!(cc_get(@self, !key))
    }

    pub fn contains(&self, key: &str) -> Result<bool> {
        out!(cc_contains(@self, !key))
    }

    pub fn names(&self) -> Result<Vec<String>> {
        out!(cc_names(@self))
    }

    pub fn iter(&self) -> Result<std::vec::IntoIter<(String, Constraint)>> {
        let values = self
            .names()?
            .into_iter()
            .map(|name| Ok((name.clone(), self.get(&name)?)))
            .collect::<Result<Vec<_>>>()?;
        Ok(values.into_iter())
    }
}

impl TryIndex<&str> for ConstraintCollection {
    type Output = Constraint;
    fn try_index(&self, index: &str) -> Result<Self::Output> {
        self.get(index)
    }
}

impl IntoIterator for ConstraintCollection {
    type Item = (String, Constraint);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .expect("failed to iterate a host-backed ConstraintCollection")
    }
}

