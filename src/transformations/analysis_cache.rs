use super::passes::{binary_spin::BinarySpinInfo, max_bias::MaxBias};
use aqm_macros::register_caches;
use indexmap::{IndexMap, map::Iter};
use std::fmt::Debug;

register_caches!(MaxBias, BinarySpinInfo);

pub struct AnalysisCache {
    store: IndexMap<String, AnalysisCacheElement>,
    history: Vec<(String, Reason, AnalysisCacheElement)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Reason {
    Overridden,
    Invalidated,
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
        return self.store.iter();
    }
}
