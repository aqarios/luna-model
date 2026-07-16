//! Owning wrapper over a LunaModel result_view handle.
//!
//! Mirrors the Rust `ResultView`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `ResultView` still return `Result` here.

use lunamodel_cext_vtable::LmResultView;
use rs_lunamodel_derive::RawHandle;

use std::collections::HashMap;

use crate::{Error, Result, SampleView, Solution, Status, TryIndex};

/// Owning handle to a LunaModel result_view (frees on drop).
#[derive(RawHandle)]
pub struct ResultView {
    handle: *mut LmResultView,
}

impl TryIndex<&str> for ResultView {
    type Output = f64;
    fn try_index(&self, index: &str) -> Result<Self::Output> {
        self.sample()?.try_index(index)
    }
}

impl ResultView {
    pub fn new(solution: &Solution, index: usize) -> Result<Option<Self>> {
        solution.result(index)
    }

    pub fn counts(&self) -> Result<usize> {
        out!(result_view_counts(@self))
    }

    pub fn obj_value(&self) -> Result<Option<f64>> {
        out_opt!(result_view_obj_value(@self))
    }

    pub fn raw_energy(&self) -> Result<Option<f64>> {
        out_opt!(result_view_raw_energy(@self))
    }

    pub fn feasible(&self) -> Result<Option<bool>> {
        out_opt!(result_view_feasible(@self))
    }

    pub fn constraints(&self) -> Result<Option<HashMap<String, bool>>> {
        let values: Option<HashMap<String, Vec<bool>>> = out_opt!(result_view_constraints(@self))?;
        Ok(values.map(|values| {
            values
                .into_iter()
                .filter_map(|(name, values)| values.into_iter().next().map(|value| (name, value)))
                .collect()
        }))
    }

    pub fn variable_bounds(&self) -> Result<Option<HashMap<String, bool>>> {
        let values: Option<HashMap<String, Vec<bool>>> =
            out_opt!(result_view_variable_bounds(@self))?;
        Ok(values.map(|values| {
            values
                .into_iter()
                .filter_map(|(name, values)| values.into_iter().next().map(|value| (name, value)))
                .collect()
        }))
    }

    pub fn sample(&self) -> Result<SampleView> {
        out!(result_view_sample(@self))
    }

    /// Returns this result row as a variable-name/value map.
    pub fn as_map(&self) -> Result<HashMap<String, f64>> {
        self.sample()?.as_map()
    }
}

impl TryFrom<(&Solution, usize)> for ResultView {
    type Error = Error;
    fn try_from((solution, index): (&Solution, usize)) -> Result<Self> {
        Self::new(solution, index)?.ok_or_else(|| Error {
            status: Status::Absent,
            message: format!("solution has no result row at index {index}"),
        })
    }
}

impl Drop for ResultView {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, result_view_free)
        }
    }
}
