use lunamodel_core::{Expression, ops::LmAddAssign, prelude::LazyBounds};
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::{PyConstraint, PyExpression, PyVariable, bounds::BoundValue, types::{PySense, PyVtype}};

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
        self.m.write_arc().set_objective(expression.into(), sense.map(|s| s.into()))
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        let expr: Expression = expression.into();
        Ok(self.m.write_arc().objective.add_assign(expr)?)
    }
}
