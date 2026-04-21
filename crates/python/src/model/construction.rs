use lunamodel_core::{Environment, Model};
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyModel;
use crate::{PyEnvironment, args::PyEnvArg, types::PySense};

#[unwindable]
#[pymethods]
impl PyModel {
    #[new]
    fn pynew(name: Option<String>, sense: PySense, env: Option<PyEnvArg>) -> PyResult<Self> {
        let env: PyEnvironment = env
            .try_into()
            .unwrap_or_else(|_| PyEnvironment::new(Environment::default()));
        Ok(Model::with_env(name, Some(sense.into()), env.env).into())
    }
}
