use std::collections::HashMap;

use lunamodel_core::Solution as CoreSolution;
use lunamodel_error::LunaModelError;
use lunamodel_serializer::prelude::*;
use napi::bindgen_prelude::{Error, Result, Status, Uint8Array};
use napi_derive::napi;

use crate::error::map_luna_error;
use crate::timing::JsTiming;

/// Column-oriented solution data for model evaluation or solver results.
///
/// A solution is independent of the original model and stores all variable data
/// by variable name. JavaScript solutions are created from LunaModel's binary
/// serializer with `Solution.deserialize()`.
#[napi(js_name = "Solution")]
pub struct JsSolution {
    inner: CoreSolution,
}

#[napi]
impl JsSolution {
    /// Decode a LunaModel solution from serialized binary bytes.
    ///
    /// This is the JavaScript alias for Python's `Solution.decode()` /
    /// `Solution.deserialize()` path. `data` must contain bytes produced by the
    /// existing LunaModel `Solution` serializer.
    #[napi(factory, js_name = "deserialize")]
    pub fn deserialize(data: Uint8Array) -> Result<Self> {
        let inner = deserialize_solution(data.as_ref())?;
        Ok(Self { inner })
    }

    /// Number of occurrences for each stored sample row.
    ///
    /// This matches the Python `counts` property.
    #[napi(getter)]
    pub fn counts(&self) -> Result<Vec<u32>> {
        self.inner
            .counts
            .iter()
            .map(|count| {
                u32::try_from(*count).map_err(|_| {
                    Error::new(
                        Status::GenericFailure,
                        format!("count {count} is too large to return as a JavaScript number"),
                    )
                })
            })
            .collect()
    }

    /// Objective values as computed by the solver.
    ///
    /// Returns `null` if the solver did not provide raw energies. This matches
    /// the Python `raw_energies` property.
    #[napi(getter)]
    pub fn raw_energies(&self) -> Option<Vec<f64>> {
        self.inner.raw_energies.clone()
    }

    /// Objective values as computed by the corresponding model.
    ///
    /// Returns `null` for solutions that have not yet been evaluated. This
    /// matches the Python `obj_values` property.
    #[napi(getter)]
    pub fn obj_values(&self) -> Option<Vec<f64>> {
        self.inner.obj_values.clone()
    }

    /// Feasibility flag for each stored sample row.
    ///
    /// A value is `true` when all constraints and variable bounds are satisfied
    /// for that sample. Returns `null` for solutions without feasibility data.
    #[napi(getter)]
    pub fn feasible(&self) -> Option<Vec<bool>> {
        self.inner.feasible.clone()
    }

    /// Per-constraint feasibility flags keyed by constraint name.
    ///
    /// Each vector is aligned with the stored sample rows.
    #[napi(getter)]
    pub fn constraints(&self) -> HashMap<String, Vec<bool>> {
        self.inner.constraints.clone()
    }

    /// Per-variable bound feasibility flags keyed by variable name.
    ///
    /// Each vector is aligned with the stored sample rows.
    #[napi(getter)]
    pub fn variable_bounds(&self) -> HashMap<String, Vec<bool>> {
        self.inner.variable_bounds.clone()
    }

    /// Runtime metrics carried by this solution.
    ///
    /// Returns `null` if no timing metadata is available. This corresponds to
    /// Python's `runtime` property.
    #[napi(getter)]
    pub fn timing(&self) -> Option<JsTiming> {
        self.inner.timing.map(|t| t.into())
    }

    /// Fraction of total sample mass marked as feasible.
    ///
    /// Computes the count-weighted ratio of feasible samples to all samples.
    /// Throws if feasibility data is not available.
    #[napi]
    pub fn feasibility_ratio(&self) -> Result<f64> {
        self.inner.feasibility_ratio().map_err(map_luna_error)
    }

    /// Return a new solution containing only feasible sample rows.
    ///
    /// Throws if feasibility data is not available. Filtering feasible samples
    /// is not possible on a non-evaluated solution.
    #[napi]
    pub fn filter_feasible(&self) -> Result<Self> {
        if let Some(feasibles) = &self.inner.feasible {
            Ok(Self {
                inner: self
                    .inner
                    .filter_by_mask(feasibles)
                    .map_err(map_luna_error)?,
            })
        } else {
            Err(map_luna_error(LunaModelError::Computation(
                "filter_feasible is not possible on non-evaluated solution.".into(),
            )))
        }
    }
}

impl JsSolution {
    #[allow(dead_code)]
    pub(crate) fn inner(&self) -> &CoreSolution {
        &self.inner
    }
}

fn deserialize_solution(data: &[u8]) -> Result<CoreSolution> {
    data.unversionize()
        .decompress()
        .map_err(deserialize_error)?
        .decode(())
        .map_err(map_luna_error)
}

fn deserialize_error<E: std::fmt::Display>(err: E) -> Error {
    Error::from_reason(format!("failed to deserialize LunaModel Solution: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_encoded_solution() {
        let expected = CoreSolution::default();
        let bytes = expected.encode(Some(false), None).unwrap();

        let actual = deserialize_solution(bytes.as_slice()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn rejects_invalid_solution_bytes() {
        let err = deserialize_solution(&[1, 2, 3]).unwrap_err();

        assert!(err.reason.contains("decoding failed"));
    }

    #[test]
    fn counts_reject_values_that_do_not_fit_js_integer_array_type() {
        let inner = CoreSolution {
            counts: vec![u32::MAX as usize + 1],
            ..Default::default()
        };
        let solution = JsSolution { inner };

        let err = solution.counts().unwrap_err();

        assert!(
            err.reason
                .contains("is too large to return as a JavaScript number")
        );
    }

    #[test]
    fn feasibility_ratio_maps_core_error_to_napi_error() {
        let solution = JsSolution {
            inner: CoreSolution::default(),
        };

        let err = solution.feasibility_ratio().unwrap_err();

        assert!(
            err.reason
                .contains("error during computation: feasible is not set")
        );
    }
}
