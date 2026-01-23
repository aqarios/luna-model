use lunamodel_types::Sense;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyModel;
use crate::{PyConstraintCollection, PyExpression};

#[unwindable]
#[pymethods]
impl PyModel {
    #[setter]
    fn set_name(&mut self, name: String) {
        self.m.write_arc().name = name;
    }

    #[setter]
    fn set_sense(&mut self, sense: Sense) {
        self.m.write_arc().sense = sense;
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
    fn set_sense_py(&mut self, sense: Sense) {
        self.m.write_arc().sense = sense;
    }
}
