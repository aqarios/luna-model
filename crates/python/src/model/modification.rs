use lunamodel_core::prelude::LazyBounds;
use lunamodel_types::Vtype;
use pyo3::{PyResult, pymethods};

use crate::{PyVariable, bounds::BoundValue};

use super::PyModel;

#[pymethods]
impl PyModel {
    #[pyo3(signature = (name, vtype=None, lower=BoundValue::None, upper=BoundValue::None))]
    fn add_variable(
        &mut self,
        name: String,
        vtype: Option<Vtype>,
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
            .add_var(&name, vtype.unwrap_or_else(|| Vtype::Binary), bounds)?
            .into())
    }
}
