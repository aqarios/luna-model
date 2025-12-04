use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;
use pyo3::{IntoPyObjectExt, prelude::*};

use super::{PyExpression, content::PyExprContent as PyEC};
use crate::variable::PyVariable;

#[pyclass]
pub struct PyConstant();

#[pymethods]
impl PyConstant {
    fn __str__(&self) -> String {
        "Constant()".into()
    }
}

#[pyclass]
pub struct PyLinear(pub PyVariable);

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
        Ok(format!("Linear({})", self.0.v.name()?).into())
    }
}

#[pyclass]
pub struct PyQuadratic(pub (PyVariable, PyVariable));

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
        Ok(format!("Quadratic({}, {})", self.0.0.v.name()?, self.0.1.v.name()?).into())
    }
}

#[pyclass]
pub struct PyHigherOrder(pub Vec<PyVariable>);

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
        let items = match &expr.expr {
            PyEC::Expr(expr) => expr
                .read_arc()
                .items()
                .map(|(vs, b)| (vs.into_iter().map(|v| PyVariable::new(v)).collect(), b))
                .collect(),
            PyEC::Model(m) => m
                .read_arc()
                .objective
                .items()
                .map(|(vs, b)| (vs.into_iter().map(|v| PyVariable::new(v)).collect(), b))
                .collect(),
        };
        Self {
            items,
            current_idx: 0,
        }
    }
}

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
