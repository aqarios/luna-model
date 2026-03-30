use lunamodel_core::{Expression, ops::LmAddAssign, prelude::LazyBounds};
use lunamodel_unwind::*;
use numpy::{
    IxDyn, ToPyArray,
    ndarray::{ArrayD, Dimension},
};
use pyo3::{Bound, FromPyObject, IntoPyObjectExt, Py, PyAny, PyResult, Python, pymethods};

use super::PyModel;
use crate::{
    PyConstraint, PyExpression, PyVariable,
    bounds::BoundValue,
    types::{PySense, PyVtype},
};

const DEFAULT_DELIM: &str = ",";

#[derive(Debug, FromPyObject)]
enum Shape {
    Tuple(Vec<usize>),
    Single(usize),
}

impl Into<Vec<usize>> for Shape {
    fn into(self) -> Vec<usize> {
        match self {
            Self::Tuple(v) => v,
            Self::Single(s) => vec![s],
        }
    }
}

#[unwindable]
#[pymethods]
impl PyModel {
    #[pyo3(signature = (name, vtype=None, lower=BoundValue::None, upper=BoundValue::None))]
    fn add_variable(
        &mut self,
        name: String,
        vtype: Option<PyVtype>,
        lower: BoundValue,
        upper: BoundValue,
    ) -> PyResult<PyVariable> {
        let bounds = match (lower, upper) {
            (BoundValue::None, BoundValue::None) => None,
            (l, u) => Some(LazyBounds::new(l.into(), u.into())),
        };
        Ok(self
            .m
            .write_arc()
            .add_var(
                &name,
                vtype.unwrap_or_else(|| PyVtype::Binary).into(),
                bounds,
            )?
            .into())
    }

    #[pyo3(signature = (name, shape, vtype=None, lower=BoundValue::None, upper=BoundValue::None, with_fallback=false, delimiter=None))]
    fn add_variables<'py>(
        &mut self,
        py: Python<'py>,
        name: String,
        shape: Shape,
        vtype: Option<PyVtype>,
        lower: BoundValue,
        upper: BoundValue,
        with_fallback: bool,
        delimiter: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let delimiter = delimiter.unwrap_or_else(|| DEFAULT_DELIM.to_string());
        let shape: Vec<usize> = shape.into();
        let adder = match with_fallback {
            true => Self::add_variable_with_fallback,
            false => Self::add_variable,
        };
        let arr: ArrayD<Py<PyAny>> = ArrayD::from_shape_fn(IxDyn(&shape), |idx| {
            let mut indices = Vec::new();
            for i in 0..idx.ndim() {
                indices.push(idx[i]);
            }

            let n = format!(
                "{name}{}",
                indices
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(&delimiter)
            );
            adder(self, n, vtype, lower, upper)
                .expect("cannot create variable")
                .into_py_any(py)
                .expect("cannot convert to Any")
        });

        Ok(arr.to_pyarray(py).into_any())
    }

    #[pyo3(signature = (name, vtype=None, lower=BoundValue::None, upper=BoundValue::None))]
    fn add_variable_with_fallback(
        &mut self,
        name: String,
        vtype: Option<PyVtype>,
        lower: BoundValue,
        upper: BoundValue,
    ) -> PyResult<PyVariable> {
        let bounds = match (lower, upper) {
            (BoundValue::None, BoundValue::None) => None,
            (l, u) => Some(LazyBounds::new(l.into(), u.into())),
        };
        Ok(self
            .m
            .write_arc()
            .add_var_with_fallback(
                &name,
                vtype.unwrap_or_else(|| PyVtype::Binary).into(),
                bounds,
                None,
            )?
            .into())
    }

    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyConstraint, name: Option<String>) -> PyResult<()> {
        Ok(self
            .m
            .write_arc()
            .constraints
            .add_constraint(constraint.c.read_arc().clone(), name)?)
    }

    #[pyo3(name = "set_objective", signature=(expression, sense=None))]
    fn set_objective_direct(&mut self, expression: PyExpression, sense: Option<PySense>) {
        self.m
            .write_arc()
            .set_objective(expression.into(), sense.map(|s| s.into()))
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        let expr: Expression = expression.into();
        Ok(self.m.write_arc().objective.add_assign(expr)?)
    }
}
