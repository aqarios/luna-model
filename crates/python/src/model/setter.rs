use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyModel;
use crate::{PyConstraintCollection, PyExpression, PyModelMetadata, types::PySense};

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
    fn set_objective(&mut self, obj: PyExpression) {
        self.m.write_arc().objective = obj.into();
    }

    #[setter]
    fn set_constraints(&mut self, coll: &PyConstraintCollection) {
        self.m.write_arc().constraints = coll.into();
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
