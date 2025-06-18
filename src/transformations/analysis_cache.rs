use std::boxed::Box;
use std::{any::Any, collections::hash_map::HashMap};

pub trait AnalysisResult: Any + Sync + Send + Clone {}

#[derive(Debug)]
pub struct AnalysisCache {
    store: HashMap<String, Box<dyn Any + Sync + Send>>,
    history: Vec<(String, Reason, Box<dyn Any + Sync + Send>)>,
}

#[derive(Debug)]
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

    pub fn insert<T: AnalysisResult>(&mut self, name: &str, item: T) {
        let x = Box::new(item);
        match self.store.insert(name.to_owned(), x) {
            Some(old) => self
                .history
                .push((name.to_owned(), Reason::Overridden, old)),
            _ => {}
        }
    }

    pub fn get<T: AnalysisResult>(&self, name: &str) -> Option<&T> {
        self.store
            .get(name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_mut<T: AnalysisResult>(&mut self, name: &str) -> Option<&mut T> {
        self.store
            .get_mut(name)
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    pub fn get_history<T: AnalysisResult>(&self, name: &str) -> Vec<(&T, &Reason)> {
        self.history
            .iter()
            .rev()
            .filter(|(k, _, _)| k == name)
            .filter_map(|(_, r, v)| v.downcast_ref::<T>().map(|x| (x, r)))
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
