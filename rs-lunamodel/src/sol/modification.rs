// pub sol_remove_cols: unsafe extern "C" fn(*mut LmSol, *const *const c_char, usize) -> LmStatus,
// pub sol_aggregate: unsafe extern "C" fn(*mut LmSol) -> LmStatus,

use indexmap::IndexMap;
use lunamodel_cext_vtable::{LmAssignment, LmColumn};
use lunamodel_types::{Sense, Vtype};

use crate::{
    Assignment, Column, Result, ResultView, Solution,
    traits::{LmFrom, ToPtrList},
};

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

    pub fn remove_col(&mut self, var: &str) -> Result<Option<Column>> {
        out!(sol_remove_col(@mut self, !var), |handle: *mut LmColumn| {
            if handle.is_null() { Ok(None) } else { Ok(Some(crate::traits::LmTryInto::lm_try_into(handle)?)) }
        })
    }

    pub fn remove_cols(&mut self, vars: &[String]) -> Result<Vec<Option<Column>>> {
        vars.iter().map(|var| self.remove_col(var)).collect()
    }

    pub fn iter_remove_cols(
        &mut self,
        vars: &[String],
    ) -> Result<std::vec::IntoIter<Option<Column>>> {
        Ok(self.remove_cols(vars)?.into_iter())
    }

    /// Retains the older bulk ABI operation for extension authors that do not
    /// need the removed columns.
    pub fn remove_cols_discarding(&mut self, vars: &[String]) -> Result<()> {
        check!(sol_remove_cols(@mut self, *vars))
    }

    pub fn aggregate(&mut self) -> Result<()> {
        check!(sol_aggregate(@mut self))
    }

    pub fn filter_by_mask(&self, mask: &[bool]) -> Result<Self> {
        out!(sol_filter_by_mask(@self, mask.as_ptr(), mask.len()))
    }

    pub fn filter<F>(&self, predicate: F) -> Result<Self>
    where
        F: Fn(&ResultView) -> Result<bool>,
    {
        let results = self.results()?;
        let mask = results.iter().map(predicate).collect::<Result<Vec<_>>>()?;
        self.filter_by_mask(&mask)
    }

    pub fn push(
        &mut self,
        sample: IndexMap<String, Assignment>,
        counts: usize,
        energy: Option<f64>,
    ) -> Result<()> {
        let (names, assignments): (Vec<_>, Vec<LmAssignment>) = sample
            .into_iter()
            .map(|(name, assignment)| (name, LmAssignment::lm_from(assignment)))
            .unzip();
        let names = names.as_slice().to_ptr_list()?;
        check!(sol_push(
            @mut self,
            names.as_ptr(),
            assignments.as_ptr(),
            assignments.len(),
            counts,
            ?energy
        ))
    }

    pub fn set_sense(mut self, sense: Sense) -> Result<Self> {
        check!(sol_set_sense(@mut self, ~sense))?;
        Ok(self)
    }
}
