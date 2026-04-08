use std::{any::Any, collections::HashMap, marker::PhantomData, sync::Arc};

use lunamodel_error::LunaModelResult;

use crate::error::TransformationError;

/// Typed key for accessing analysis results.
/// The type parameter ensures compile-time type safety.
pub struct AnalysisKey<T: 'static> {
    pub(crate) name: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> AnalysisKey<T> {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

/// Type-safe analysis storage
#[derive(Clone, Default)]
pub struct AnalysisManager {
    results: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl AnalysisManager {
    /// Get an analysis result (returns None if not computed)
    pub fn get<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.results
            .get(&key.name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Get an analysis result (error if not available)
    pub fn require<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> LunaModelResult<&T> {
        self.get(key).ok_or_else(|| {
            TransformationError::MissingAnalysis {
                name: key.name.clone(),
            }
            .into()
        })
    }

    /// Store an analysis result
    pub fn insert<T: Send + Sync + 'static>(&mut self, key: &AnalysisKey<T>, value: T) {
        self.results.insert(key.name.clone(), Arc::new(value));
    }

    /// Invalidate analysis entries by key.
    pub fn invalidate_many(&mut self, keys: &[String]) {
        for key in keys {
            self.results.remove(key);
        }
    }
}
