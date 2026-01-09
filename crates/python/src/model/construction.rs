use lunamodel_core::{Environment, Model};
use lunamodel_types::Sense;
use pyo3::{PyResult, pymethods};

use crate::PyEnvironment;

use super::PyModel;

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
