use indexmap::{
    IndexMap,
    map::{IntoIter, Iter},
};
use lunamodel_types::Specs;
#[cfg(feature = "py")]
use pyo3::{Py, PyAny, Python};

use crate::passes::analysis::MinConstraintValues;

use super::passes::{analysis::MaxBias, special::IfElseInfo, transformation::BinarySpinInfo};

#[derive(Debug)]
pub enum AnalysisCacheElement {
    IfElseInfoAnalysis(IfElseInfo),
    MaxBiasAnalysis(MaxBias),
    BinarySpinInfoAnalysis(BinarySpinInfo),
    MinValueInConstraintAnalysis(MinConstraintValues),
    SpecsAnalysis(Specs),
    #[cfg(feature = "py")]
    PyAnalysis(Py<PyAny>),
}

impl Clone for AnalysisCacheElement {
    fn clone(&self) -> Self {
        match self {
            AnalysisCacheElement::MaxBiasAnalysis(v) => {
                AnalysisCacheElement::MaxBiasAnalysis(v.clone())
            }
            AnalysisCacheElement::BinarySpinInfoAnalysis(v) => {
                AnalysisCacheElement::BinarySpinInfoAnalysis(v.clone())
            }
            AnalysisCacheElement::IfElseInfoAnalysis(v) => {
                AnalysisCacheElement::IfElseInfoAnalysis(v.clone())
            }
            AnalysisCacheElement::MinValueInConstraintAnalysis(v) => {
                AnalysisCacheElement::MinValueInConstraintAnalysis(v.clone())
            }
            AnalysisCacheElement::SpecsAnalysis(v) => {
                AnalysisCacheElement::SpecsAnalysis(v.clone())
            }
            #[cfg(feature = "py")]
            AnalysisCacheElement::PyAnalysis(v) => {
                Python::attach(|py| AnalysisCacheElement::PyAnalysis(v.clone_ref(py)))
            }
        }
    }
}

#[cfg(feature = "py")]
impl AnalysisCacheElement {
    pub fn clone_py(&self, py: Python) -> Self {
        match self {
            AnalysisCacheElement::MaxBiasAnalysis(v) => {
                AnalysisCacheElement::MaxBiasAnalysis(v.clone())
            }
            AnalysisCacheElement::BinarySpinInfoAnalysis(v) => {
                AnalysisCacheElement::BinarySpinInfoAnalysis(v.clone())
            }
            AnalysisCacheElement::IfElseInfoAnalysis(v) => {
                AnalysisCacheElement::IfElseInfoAnalysis(v.clone())
            }
            AnalysisCacheElement::MinValueInConstraintAnalysis(v) => {
                AnalysisCacheElement::MinValueInConstraintAnalysis(v.clone())
            }
            AnalysisCacheElement::SpecsAnalysis(v) => {
                AnalysisCacheElement::SpecsAnalysis(v.clone())
            }
            AnalysisCacheElement::PyAnalysis(v) => {
                AnalysisCacheElement::PyAnalysis(v.clone_ref(py))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reason {
    Overridden,
    Invalidated,
}

#[derive(Debug, Clone)]
pub struct AnalysisCache {
    store: IndexMap<String, AnalysisCacheElement>,
    history: Vec<(String, Reason, AnalysisCacheElement)>,
}

impl AnalysisCache {
    pub fn new() -> AnalysisCache {
        AnalysisCache {
            store: IndexMap::new(),
            history: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: &str, item: AnalysisCacheElement) {
        match self.store.insert(name.to_owned(), item) {
            Some(old) => self
                .history
                .push((name.to_owned(), Reason::Overridden, old)),
            _ => {}
        }
    }

    pub fn insert_from(&mut self, cache: Self) {
        for (key, element) in cache.into_iter() {
            self.insert(&key, element);
        }
    }

    pub fn get(&self, name: &str) -> Option<&AnalysisCacheElement> {
        self.store.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut AnalysisCacheElement> {
        self.store.get_mut(name)
    }

    pub fn get_history(&self, name: &str) -> Vec<(&AnalysisCacheElement, &Reason)> {
        self.history
            .iter()
            .rev()
            .filter(|(k, _, _)| k == name)
            .filter_map(|(_, r, v)| Some((v, r)))
            .collect()
    }

    pub fn invalidate(&mut self, names: &[&str]) {
        names.iter().for_each(|&x| {
            if let Some(v) = self.store.shift_remove(x) {
                self.history.push((x.to_owned(), Reason::Invalidated, v))
            }
        });
    }

    pub fn iter(&self) -> Iter<'_, String, AnalysisCacheElement> {
        self.store.iter()
    }

    pub fn into_iter(self) -> IntoIter<String, AnalysisCacheElement> {
        self.store.into_iter()
    }

    #[cfg(feature = "py")]
    pub fn clone_py(&self, py: Python) -> Self {
        Self {
            store: self
                .store
                .iter()
                .map(|(k, v)| (k.clone(), v.clone_py(py)))
                .collect(),
            history: self
                .history
                .iter()
                .map(|(k, r, e)| (k.clone(), *r, e.clone_py(py)))
                .collect(),
        }
    }
}
