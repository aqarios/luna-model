use pyo3::ffi::c_str;
use pyo3::prelude::*;

#[pyclass(unsendable, name = "CqmTranslator", module = "aqmodels.translator")]
pub struct PyCqmTranslator {}

#[pymethods]
impl PyCqmTranslator {
    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq<'a>(py: Python<'a>, model: PyObject) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(
            py,
            c_str!(
                "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from aqmodels._core import translator

def extract(model):
    lp = translator.LpTranslator.from_aq(model)
    return dimod_lp.loads(lp)
"
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("extract")?
        .into();
        let args = (model,);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }

    #[staticmethod]
    #[pyo3(signature=(cqm))]
    fn to_aq(py: Python, cqm: PyObject) -> PyResult<PyObject> {
        let extractor: PyObject = PyModule::from_code(
            py,
            c_str!(
                "
from dimod import ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from aqmodels._core import translator

def extract(cqm):
    if not isinstance(cqm, ConstrainedQuadraticModel):
        raise TypeError(f'Expected cqm to be of type CQM, received: {type(cqm)}')
    cqm_lp = dimod_lp.dumps(cqm)
    return translator.LpTranslator.to_aq(cqm_lp)
"
            ),
            c_str!(""),
            c_str!(""),
        )?
        .getattr("extract")?
        .into();
        let args = (cqm,);
        let result = extractor.call1(py, args)?;
        Ok(result)
    }
}
