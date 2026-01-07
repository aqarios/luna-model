use crate::variable::PyVariable;

use super::PySolution;
use lunamodel_types::Vtype;
use pyo3::{FromPyObject, PyAny, PyErr, PyResult, exceptions::PyTypeError, pymethods};

#[derive(Debug, Clone)]
pub enum VarKey {
    Str(String),
    Var(PyVariable),
}

impl From<String> for VarKey {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

#[pymethods]
impl PySolution {
    fn add_var(&mut self, var: VarKey, data: Vec<f64>, vtype: Option<Vtype>) -> PyResult<()> {
        let (vn, vt) = match &var {
            VarKey::Str(name) => (name.clone(), vtype.unwrap_or_else(|| Vtype::Binary)),
            VarKey::Var(v) => (v.v.name()?, v.v.vtype()?),
        };
        self.s.write_arc().add_col(vt, vn, data);
        Ok(())
    }

    fn add_vars(
        &mut self,
        vars: Vec<VarKey>,
        data: Vec<Vec<f64>>,
        vtype: Vec<Option<Vtype>>,
    ) -> PyResult<()> {
        for ((v, d), vt) in vars.into_iter().zip(data).zip(vtype) {
            self.add_var(v, d, vt)?;
        }
        Ok(())
    }

    fn remove_var(&mut self, var: VarKey) -> PyResult<()> {
        let vn = match &var {
            VarKey::Str(name) => name.clone(),
            VarKey::Var(v) => v.v.name()?,
        };
        self.s.write_arc().remove_col(vn);
        Ok(())
    }

    fn remove_vars(&mut self, vars: Vec<VarKey>) -> PyResult<()> {
        for v in vars {
            self.remove_var(v)?;
        }
        Ok(())
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for VarKey {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<String>() {
            Ok(VarKey::Str(s))
        } else if let Ok(v) = obj.extract::<PyVariable>() {
            Ok(VarKey::Var(v))
        } else {
            Err(PyTypeError::new_err("keys have to be 'str' or 'Variable'"))
        }
    }
}
