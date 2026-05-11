//! Constructors for Python variables.

use lunamodel_core::prelude::LazyBounds;
use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{exceptions::PyTypeError, prelude::*};

use super::PyVariable;
use crate::{
    args::{PyBoundsArg, PyEnvArg},
    bounds::{BoundValue, BoundsContent},
    environment::PyEnvironment,
    types::PyVtype,
};

#[unwindable]
#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, vtype, bounds=None, env=None, lower=BoundValue::None, upper=BoundValue::None))]
    fn py_new(
        name: String,
        vtype: PyVtype,
        bounds: Option<PyBoundsArg>,
        env: Option<PyEnvArg>,
        lower: BoundValue,
        upper: BoundValue,
    ) -> PyResult<Self> {
        if vtype == PyVtype::InvertedBinary {
            return Err(LunaModelError::UnsupportedOperation(
                "cannot create an inverted binary variable directly. Use the '.inv` method or the '~' operator.".into(),
            ))?;
        };

        let bounds = match (bounds, lower, upper) {
            (Some(_), BoundValue::Value(_), BoundValue::Value(_))
            | (Some(_), BoundValue::Unbounded, BoundValue::Value(_))
            | (Some(_), BoundValue::Value(_), BoundValue::Unbounded)
            | (Some(_), BoundValue::Unbounded, BoundValue::Unbounded)
            | (Some(_), BoundValue::None, BoundValue::Unbounded)
            | (Some(_), BoundValue::Unbounded, BoundValue::None)
            | (Some(_), BoundValue::Value(_), BoundValue::None)
            | (Some(_), BoundValue::None, BoundValue::Value(_)) => {
                return Err(PyTypeError::new_err(
                    "`bounds` cannot be combined with `lower` or `upper`; pass either `bounds=` or `lower=`/`upper=`",
                ));
            }
            (Some(bs), BoundValue::None, BoundValue::None) => {
                let bc: &BoundsContent = &bs.0.read_arc();
                Some(bc.clone().into())
            }
            (None, BoundValue::None, BoundValue::None) => None,
            (None, l, u) => Some(LazyBounds::new(l.into(), u.into())),
        };
        let mut penv: PyEnvironment = env.try_into()?;
        let vref = penv.env.insert(&name, vtype.into(), bounds)?;
        Ok(PyVariable::new(vref))
    }
}
