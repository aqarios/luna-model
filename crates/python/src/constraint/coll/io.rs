use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use crate::constraint::coll::PyConstraintCollectionContent;

use super::PyConstraintCollection;

#[pymethods]
impl PyConstraintCollection {
    fn __str__(&self) -> String {
        match &self.c {
            PyConstraintCollectionContent::Coll(c) => {
                format!("{}", c.read_arc().format(FormatOpt::Py))
            }
            PyConstraintCollectionContent::Model(m) => {
                format!("{}", m.read_arc().constraints.format(FormatOpt::Py))
            }
        }
    }

    fn __repr__(&self) -> String {
        match &self.c {
            PyConstraintCollectionContent::Coll(c) => {
                format!("{:?}", c.read_arc().format(FormatOpt::Py))
            }
            PyConstraintCollectionContent::Model(m) => {
                format!("{:?}", m.read_arc().constraints.format(FormatOpt::Py))
            }
        }
    }
}
