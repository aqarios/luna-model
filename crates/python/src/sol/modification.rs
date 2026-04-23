use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use crate::types::PyVtype;

use super::PySolution;
use crate::utils::VarKey;

#[unwindable]
#[pymethods]
impl PySolution {
    fn add_var(&mut self, var: VarKey, data: Vec<f64>, vtype: Option<PyVtype>) -> PyResult<()> {
        let (vn, vt) = match &var {
            VarKey::Str(name) => (
                name.clone(),
                vtype.unwrap_or(PyVtype::Binary).into(),
            ),
            VarKey::Var(v) => (v.v.name()?, v.v.vtype()?),
        };
        self.s.write_arc().add_col(vt, vn, data, None)?;
        Ok(())
    }

    fn add_vars(
        &mut self,
        vars: Vec<VarKey>,
        data: Vec<Vec<f64>>,
        vtypes: Option<Vec<Option<PyVtype>>>,
    ) -> PyResult<()> {
        let vtypes: Vec<_> = match vtypes {
            Some(vs) => vs,
            None => vec![None; vars.len()],
        };
        for ((v, d), vt) in vars.into_iter().zip(data).zip(vtypes) {
            self.add_var(v, d, vt)?;
        }
        Ok(())
    }

    fn remove_var(&mut self, var: VarKey) -> PyResult<()> {
        let vn = match &var {
            VarKey::Str(name) => name.clone(),
            VarKey::Var(v) => v.v.name()?,
        };
        self.s.write_arc().remove_col(&vn);
        Ok(())
    }

    fn remove_vars(&mut self, vars: Vec<VarKey>) -> PyResult<()> {
        for v in vars {
            self.remove_var(v)?;
        }
        Ok(())
    }
}
