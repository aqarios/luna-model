use std::sync::Arc;

use luna_model::transpiler::{AnalysisManager, PassContext};

#[repr(transparent)]
pub struct PyPassContext(pub Arc<AnalysisManager>);

impl PyPassContext {
    pub fn inner(&self) -> Arc<AnalysisManager> {
        Arc::clone(&self.0)
    }
}

impl From<AnalysisManager> for PyPassContext {
    fn from(value: AnalysisManager) -> Self {
        Self(Arc::new(value))
    }
}

impl<'c> From<&'c PyPassContext> for PassContext<'c> {
    fn from(value: &'c PyPassContext) -> Self {
        Self::new(&value.0)
    }
}

capsule_wrapper! {
    wrapper: PyPassContext,
    public: PassContext,
    inner: Arc<AnalysisManager>,
    attr: "_c",
    from_py: "_from_pyctx",
}
