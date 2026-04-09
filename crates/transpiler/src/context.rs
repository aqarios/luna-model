use lunamodel_error::LunaModelResult;

use crate::analysis::{AnalysisKey, AnalysisManager};

/// Context provided to passes during execution.
///
/// This is how passes access analyses and other infrastructure.
pub struct PassContext<'a> {
    analysis_manager: &'a AnalysisManager,
}

impl<'a> PassContext<'a> {
    pub fn new(analysis_manager: &'a AnalysisManager) -> Self {
        Self { analysis_manager }
    }

    /// Get an analysis result
    pub fn get_analysis<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.analysis_manager.get(key)
    }

    /// Require an analysis result (error if missing)
    pub fn require_analysis<T: Send + Sync + 'static>(
        &self,
        key: &AnalysisKey<T>,
    ) -> LunaModelResult<&T> {
        self.analysis_manager.require(key)
    }

    pub fn manager(&self) -> &AnalysisManager {
        self.analysis_manager
    }
}
