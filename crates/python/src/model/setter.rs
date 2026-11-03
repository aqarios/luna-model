//! In-place setters for Python models.

use std::{ops::Mul, sync::Arc};

use lunamodel_core::{ConstraintCollection, Expression};
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::{
    PyExprContent, PyModelMetadata, args::PyColArg, constraint::PyConstraintCollectionContent,
    types::PySense, utils::OpsOther,
};

#[unwindable]
#[pymethods]
impl PyModel {
    #[setter]
    fn set_name(&mut self, name: String) {
        self.m.write_arc().name = name;
    }

    #[setter]
    fn set_sense(&mut self, sense: PySense) {
        self.m.write_arc().sense = sense.into();
    }

    #[setter]
    fn set_objective(&mut self, obj: OpsOther) -> PyResult<()> {
        // `model.objective += ...` modifies the model through the getter's view
        // and then assigns that same view back. Copying it would be a no-op that
        // costs O(size of the objective) on every `+=`.
        if let OpsOther::Expr(e) = &obj
            && let PyExprContent::Model(m) = &e.0.expr
            && Arc::ptr_eq(m, &self.m)
        {
            return Ok(());
        }
        let new_obj = match obj {
            OpsOther::Expr(e) => e.into(),
            OpsOther::Var(v) => (&v.v).mul(1.0)?,
            OpsOther::Num(n) => Expression::constant(self.m.read_arc().environment.clone(), n),
        };
        self.m.write_arc().set_objective(new_obj, None)?;
        Ok(())
    }

    #[setter]
    fn set_constraints(&mut self, coll: PyColArg) -> PyResult<()> {
        // `model.constraints += ...` modifies the model through the getter's view
        // and then assigns that same view back. Copying it would be a no-op that
        // costs O(n) on every `+=`.
        if let PyConstraintCollectionContent::Model(m) = coll.0.inner()
            && Arc::ptr_eq(m, &self.m)
        {
            return Ok(());
        }
        let col: ConstraintCollection = (&coll.0).into();
        self.m.write_arc().set_constraints(col)?;
        Ok(())
    }

    #[pyo3(name = "set_sense")]
    fn set_sense_py(&mut self, sense: PySense) {
        self.m.write_arc().sense = sense.into();
    }

    #[setter]
    #[pyo3(name = "_metadata")]
    fn set_metadata(&mut self, metadata: PyModelMetadata) {
        self._metadata = metadata;
    }
}
