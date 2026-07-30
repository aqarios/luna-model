//! Bulk per-sample solution lookup, dispatched through the C ABI.
//!
//! Mirrors the native `luna-model` `ops::SolutionLookup`: build once from an
//! [`Env`] and a [`Solution`], then reuse across many samples via [`update`]
//! and [`write_sol`] — avoiding one ABI round-trip per value in hot per-sample
//! loops such as samplers.
//!
//! [`update`]: SolutionLookup::update
//! [`write_sol`]: SolutionLookup::write_sol

use lunamodel_capi::LmSolutionLookup;
use rs_lunamodel_derive::RawHandle;

use crate::{Env, Result, SampleView, Solution};

/// Owning handle to a LunaModel bulk per-sample lookup table (frees on drop).
#[derive(RawHandle)]
pub struct SolutionLookup {
    handle: *mut LmSolutionLookup,
}

impl Drop for SolutionLookup {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, sol_lookup_free)
        }
    }
}

impl SolutionLookup {
    /// Builds a lookup table over `env`'s variables for `sol`.
    pub fn new(env: &Env, sol: &Solution) -> Result<Self> {
        out!(sol_lookup_new(@env, @sol))
    }

    /// Loads one sample row's values into the lookup table, in place.
    pub fn update(&mut self, sample: &SampleView) -> Result<()> {
        check!(sol_lookup_update(@mut self, @sample))
    }

    /// Returns the current lookup values, indexed by variable id.
    pub fn values(&self) -> Result<Vec<f64>> {
        out_slice!(sol_lookup_values(@self))
    }

    /// Overwrites row `idx` of `sol` with `values` (indexed by variable id).
    ///
    /// `values` is typically a caller-owned copy of a prior [`values`](Self::values)
    /// call, mutated locally (e.g. by a sampler's hot per-flip loop) without any
    /// further ABI round-trips, then written back once per sample here.
    pub fn write_sol(&mut self, idx: u32, values: &[f64], sol: &mut Solution) -> Result<()> {
        check!(sol_lookup_write_sol(
            @mut self,
            idx,
            values.as_ptr(),
            values.len(),
            @mut sol
        ))
    }
}
