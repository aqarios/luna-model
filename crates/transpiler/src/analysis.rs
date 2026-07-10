//! Typed analysis keys and analysis storage.

use std::{any::Any, collections::HashMap, fmt::Debug, marker::PhantomData, sync::Arc};

use crate::error::{TranspileErrorKind, TranspileKindResult};

/// Typed key for accessing analysis results.
/// The type parameter ensures compile-time type safety.
#[derive(Debug)]
pub struct AnalysisKey<T: 'static> {
    pub(crate) name: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> AnalysisKey<T> {
    /// Creates a new typed analysis key.
    pub const fn new(name: String) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

/// Type-safe storage for analysis results keyed by [`AnalysisKey`].
#[derive(Clone, Default, Debug)]
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
    pub fn require<T: Send + Sync + 'static>(
        &self,
        key: &AnalysisKey<T>,
    ) -> TranspileKindResult<&T> {
        self.get(key)
            .ok_or_else(|| TranspileErrorKind::MissingAnalysis {
                name: key.name.clone(),
            })
    }

    /// Stores an analysis result under its typed key.
    pub fn insert<T: Send + Sync + 'static>(&mut self, key: &AnalysisKey<T>, value: T) {
        self.results.insert(key.name.clone(), Arc::new(value));
    }

    /// Invalidates analysis entries by name.
    pub fn invalidate_many(&mut self, keys: &[String]) {
        for key in keys {
            self.results.remove(key);
        }
    }
}
