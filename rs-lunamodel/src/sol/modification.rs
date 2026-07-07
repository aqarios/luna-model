// pub sol_remove_cols: unsafe extern "C" fn(*mut LmSol, *const *const c_char, usize) -> LmStatus,
// pub sol_aggregate: unsafe extern "C" fn(*mut LmSol) -> LmStatus,

use lunamodel_types::Vtype;

use crate::{Result, Solution};

impl Solution {
    pub fn add_empty_binary(&mut self, var: &str) -> Result<()> {
        check!(sol_add_empty_binary(@mut self, !var))
    }

    pub fn add_empty_spin(&mut self, var: &str) -> Result<()> {
        check!(sol_add_empty_spin(@mut self, !var))
    }

    pub fn add_empty_integer(&mut self, var: &str) -> Result<()> {
        check!(sol_add_empty_integer(@mut self, !var))
    }

    pub fn add_empty_real(&mut self, var: &str) -> Result<()> {
        check!(sol_add_empty_real(@mut self, !var))
    }

    pub fn add_binary(&mut self, var: &str, data: Vec<f64>, tol: Option<f64>) -> Result<()> {
        check!(sol_add_binary(@mut self, !var, data.as_ptr(), data.len(), ?tol))
    }

    pub fn add_spin(&mut self, var: &str, data: Vec<f64>, tol: Option<f64>) -> Result<()> {
        check!(sol_add_spin(@mut self, !var, data.as_ptr(), data.len(), ?tol))
    }

    pub fn add_integer(&mut self, var: &str, data: Vec<f64>, tol: Option<f64>) -> Result<()> {
        check!(sol_add_integer(@mut self, !var, data.as_ptr(), data.len(), ?tol))
    }

    pub fn add_real(&mut self, var: &str, data: Vec<f64>) -> Result<()> {
        check!(sol_add_real(@mut self, !var, data.as_ptr(), data.len()))
    }

    pub fn add_col(
        &mut self,
        vtype: Vtype,
        var: &str,
        data: Vec<f64>,
        tol: Option<f64>,
    ) -> Result<()> {
        check!(sol_add_col(@mut self, ~vtype, !var, data.as_ptr(), data.len(), ?tol))
    }

    pub fn remove_col(&mut self, var: &str) -> Result<()> {
        check!(sol_remove_col(@mut self, !var))
    }

    pub fn remove_cols(&mut self, vars: &[String]) -> Result<()> {
        check!(sol_remove_cols(@mut self, *vars))
    }
}
