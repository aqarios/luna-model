//! Read-only and mutable pass execution contexts.

use crate::{
    analysis::{AnalysisKey, AnalysisManager},
    error::TranspileKindResult,
};

/// Context provided to passes during execution.
///
/// This is how passes access analyses and other infrastructure.
pub struct PassContext<'a> {
    analysis_manager: &'a AnalysisManager,
}

impl<'a> PassContext<'a> {
    /// Creates a new pass context backed by the given analysis manager.
    pub fn new(analysis_manager: &'a AnalysisManager) -> Self {
        Self { analysis_manager }
    }

    /// Returns an available analysis result if it has already been computed.
    pub fn get_analysis<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.analysis_manager.get(key)
    }

    /// Require an analysis result (error if missing)
    pub fn require_analysis<T: Send + Sync + 'static>(
        &self,
        key: &AnalysisKey<T>,
    ) -> TranspileKindResult<&T> {
        self.analysis_manager.require(key)
    }

    /// Returns the underlying analysis manager.
    pub fn manager(&self) -> &AnalysisManager {
        self.analysis_manager
    }
}
