use pyo3::Python;

use crate::cache::{AnalysisCacheElement, PyAnalysisCache};

impl PyAnalysisCache {
    pub fn _repr(&self, py: Python) -> String {
        let right = self.iter().map(|(k, _)| k.len()).max().unwrap_or(0) + 1;
        self.iter()
            .map(|(k, v)| {
                format!(
                    "{:>right$}: {}",
                    k,
                    match v {
                        AnalysisCacheElement::PyAnalysis(x) => x
                            .call_method0(py, "__repr__")
                            .unwrap()
                            .extract::<String>(py)
                            .unwrap(),
                        x => x.repr(),
                    }
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
