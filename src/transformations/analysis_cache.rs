#[cfg(feature = "py")]
use pyo3::{Py, PyAny, Python};

use super::passes::max_bias::MaxBias;
use std::{collections::hash_map::HashMap, fmt::Debug};

pub enum AnalysisCacheElement {
    MaxBiasAnalysis(MaxBias),

    #[cfg(feature = "py")]
    PyAnalysis(Py<PyAny>),
}

impl AnalysisCacheElement {
    #[cfg(feature = "py")]
    pub fn clone_py(&self, py: Python) -> Self {
        match self {
            Self::MaxBiasAnalysis(v) => Self::MaxBiasAnalysis(v.clone()),
            #[cfg(feature = "py")]
            Self::PyAnalysis(v) => Self::PyAnalysis(v.clone_ref(py)),
        }
    }
}

pub struct AnalysisCache {
    store: HashMap<String, AnalysisCacheElement>,
    history: Vec<(String, Reason, AnalysisCacheElement)>,
}

impl AnalysisCache {
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

#[derive(Debug, Clone, Copy)]
pub enum Reason {
    Overridden,
    Invalidated,
}

impl AnalysisCache {
    pub fn new() -> AnalysisCache {
        AnalysisCache {
            store: HashMap::new(),
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
            if let Some(v) = self.store.remove(x) {
                self.history.push((x.to_owned(), Reason::Invalidated, v))
            }
        });
    }
}
