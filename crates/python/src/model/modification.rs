//! Mutation operations for Python models.

use std::ops::Mul;

use lunamodel_core::{Expression, ops::LmAddAssign, prelude::LazyBounds};
use lunamodel_unwind::*;
use numpy::{
    IxDyn, ToPyArray,
    ndarray::{ArrayD, Dimension},
};
use pyo3::{
    Bound, FromPyObject, Py, PyAny, PyResult, Python, pymethods,
    types::{PyAnyMethods, PyModule},
};

use super::PyModel;
use crate::{
    PyExpression, PyVariable,
    args::PyCArg,
    bounds::BoundValue,
    constraint::utils::{ConstraintsIn, NameIn, add_many_constraint},
    types::{PySense, PyVtype},
    utils::OpsOther,
};

const DEFAULT_DELIM: &str = ",";

#[derive(Debug, FromPyObject)]
enum Shape {
    Tuple(Vec<usize>),
    Single(usize),
}

impl From<Shape> for Vec<usize> {
    fn from(val: Shape) -> Self {
        match val {
            Shape::Tuple(v) => v,
            Shape::Single(s) => vec![s],
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "Python-facing API mirrors external call shape"
)]
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
            .add_var(&name, vtype.unwrap_or(PyVtype::Binary).into(), bounds)?
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

        let varcls = PyModule::import(py, "luna_model")?
            .getattr("Variable")?
            .unbind();

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
            let out = adder(self, n, vtype, lower, upper).expect("cannot create variable");
            varcls
                .call_method1(py, "_from_pyvar", (out,))
                .expect("variable creation")
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
            .add_var_with_fallback(&name, vtype.unwrap_or(PyVtype::Binary).into(), bounds, None)?
            .into())
    }

    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyCArg, name: Option<String>) -> PyResult<()> {
        _ = self
            .m
            .write_arc()
            .constraints
            .add_constraint(constraint.c.read_arc().clone(), name)?;
        Ok(())
    }

    fn add_constraints(
        &mut self,
        py: Python,
        constraints: ConstraintsIn,
        name: Option<NameIn>,
    ) -> PyResult<Vec<String>> {
        Ok(add_many_constraint(
            py,
            &mut self.m.write_arc().constraints,
            constraints,
            name,
        )?)
    }

    #[pyo3(name = "set_objective", signature=(expression, sense=None))]
    fn set_objective_direct(
        &mut self,
        expression: OpsOther,
        sense: Option<PySense>,
    ) -> PyResult<()> {
        let new_obj = match expression {
            OpsOther::Expr(e) => e.into(),
            OpsOther::Var(v) => (&v.v).mul(1.0)?,
            OpsOther::Num(n) => Expression::constant(self.m.read_arc().environment.clone(), n),
        };
        self.m
            .write_arc()
            .set_objective(new_obj, sense.map(|s| s.into()));
        Ok(())
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        let expr: Expression = expression.into();
        Ok(self.m.write_arc().objective.add_assign(expr)?)
    }
}
