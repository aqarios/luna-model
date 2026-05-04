//! In-place setters for Python models.

use std::ops::Mul;

use lunamodel_core::Expression;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::{PyModelMetadata, args::PyColArg, types::PySense, utils::OpsOther};

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
        let new_obj = match obj {
            OpsOther::Expr(e) => e.into(),
            OpsOther::Var(v) => (&v.v).mul(1.0)?,
            OpsOther::Num(n) => Expression::constant(self.m.read_arc().environment.clone(), n),
        };
        self.m.write_arc().objective = new_obj;
        Ok(())
    }

    #[setter]
    fn set_constraints(&mut self, coll: PyColArg) {
        self.m.write_arc().constraints = (&coll.0).into();
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
