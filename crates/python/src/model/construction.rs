use lunamodel_core::{Environment, Model};
use lunamodel_types::Sense;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyModel {
    #[new]
    fn pynew(name: Option<String>, sense: Sense, env: Option<PyEnvironment>) -> PyResult<Self> {
        let env: PyEnvironment = env
            .try_into()
            .unwrap_or_else(|_| PyEnvironment::new(Environment::default()));
        Ok(Model::with_env(name, Some(sense), env.env).into())
    }
}
