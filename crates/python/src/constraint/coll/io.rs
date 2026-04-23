use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyConstraintCollection;

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    fn __str__(&self) -> String {
        format!("{}", self.read().format(FormatOpt::Py))
    }

    // fn __repr__(&self) -> String {
    //     match &self.c {
    //         PyConstraintCollectionContent::Coll(c) => {
    //             format!("{:?}", c.read_arc().format(FormatOpt::Py))
    //         }
    //         PyConstraintCollectionContent::Model(m) => {
    //             format!("{:?}", m.read_arc().constraints.format(FormatOpt::Py))
    //         }
    //     }
    // }
}
