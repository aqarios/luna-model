use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;
use lunamodel_unwind::*;
use pyo3::{IntoPyObjectExt, prelude::*};

use super::PyExpression;
use crate::variable::PyVariable;

#[pyclass(subclass)]
pub struct PyConstant();

#[unwindable]
#[pymethods]
impl PyConstant {
    fn __str__(&self) -> String {
        "Constant()".into()
    }
}

#[pyclass(subclass)]
pub struct PyLinear(pub PyVariable);

#[unwindable]
#[pymethods]
impl PyLinear {
    #[getter]
    fn var(&self) -> PyVariable {
        self.0.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("var",)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Linear({})", self.0.v.name()?))
    }
}

#[pyclass(subclass)]
pub struct PyQuadratic(pub (PyVariable, PyVariable));

#[unwindable]
#[pymethods]
impl PyQuadratic {
    #[getter]
    fn var_a(&self) -> PyVariable {
        self.0.0.clone()
    }
    #[getter]
    fn var_b(&self) -> PyVariable {
        self.0.1.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("var_a", "var_b")
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Quadratic({}, {})",
            self.0.0.v.name()?,
            self.0.1.v.name()?
        ))
    }
}

#[pyclass(subclass)]
pub struct PyHigherOrder(pub Vec<PyVariable>);

#[unwindable]
#[pymethods]
impl PyHigherOrder {
    #[getter]
    fn vars(&self) -> Vec<PyVariable> {
        self.0.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("vars",)
    }

    fn __str__(&self) -> PyResult<String> {
        let vs: LunaModelResult<Vec<_>> = self.0.iter().map(|v| v.v.name()).collect();
        Ok(format!("HigherOrder({})", vs?.join(", ")))
    }
}

#[pyclass]
pub struct PyExpressionIterator {
    items: Vec<(Vec<PyVariable>, Bias)>,
    current_idx: usize,
}

impl PyExpressionIterator {
    pub fn new(expr: &PyExpression) -> Self {
        let items = expr.read_with(|e| {
            e.items()
                .map(|(vs, b)| (vs.into_iter().map(PyVariable::new).collect(), b))
                .collect()
        });
        Self {
            items,
            current_idx: 0,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyExpressionIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> PyResult<Option<(Py<PyAny>, Bias)>> {
        let res = slf.items.get(slf.current_idx).map_or_else(
            || Ok::<Option<(pyo3::Py<pyo3::PyAny>, f64)>, PyErr>(None),
            |(vars, b)| {
                let item = match &vars[..] {
                    [] => PyConstant().into_py_any(py),
                    [a] => PyLinear(a.clone()).into_py_any(py),
                    [a, b] => PyQuadratic((a.clone(), b.clone())).into_py_any(py),
                    _ => PyHigherOrder(vars.to_vec()).into_py_any(py),
                }?;
                Ok(Some((item, *b)))
            },
        );
        slf.current_idx += 1;
        res
    }
}
