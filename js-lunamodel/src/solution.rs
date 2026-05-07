use std::collections::HashMap;

use lunamodel_core::Solution as CoreSolution;
use lunamodel_serializer::prelude::*;
use napi::bindgen_prelude::{Error, Result, Status, Uint8Array};
use napi_derive::napi;

use crate::timing::JsTiming;

#[napi(js_name = "Solution")]
pub struct JsSolution {
    inner: CoreSolution,
}

#[napi]
impl JsSolution {
    #[napi(factory, js_name = "deserialize")]
    pub fn deserialize(data: Uint8Array) -> Result<Self> {
        let inner = deserialize_solution(data.as_ref())?;
        Ok(Self { inner })
    }

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

    #[napi(getter)]
    pub fn raw_energies(&self) -> Option<Vec<f64>> {
        self.inner.raw_energies.clone()
    }

    #[napi(getter)]
    pub fn obj_values(&self) -> Option<Vec<f64>> {
        self.inner.obj_values.clone()
    }

    #[napi(getter)]
    pub fn feasible(&self) -> Option<Vec<bool>> {
        self.inner.feasible.clone()
    }

    #[napi(getter)]
    pub fn constraints(&self) -> HashMap<String, Vec<bool>> {
        self.inner.constraints.clone()
    }

    #[napi(getter)]
    pub fn variable_bounds(&self) -> HashMap<String, Vec<bool>> {
        self.inner.variable_bounds.clone()
    }

    #[napi(getter)]
    pub fn timing(&self) -> Option<JsTiming> {
        self.inner.timing.map(|t| t.into())
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
        .map_err(deserialize_error)
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

        assert!(
            err.reason
                .contains("failed to deserialize LunaModel Solution")
        );
    }

    #[test]
    fn counts_reject_values_that_do_not_fit_js_integer_array_type() {
        let mut inner = CoreSolution::default();
        inner.counts = vec![u32::MAX as usize + 1];
        let solution = JsSolution { inner };

        let err = solution.counts().unwrap_err();

        assert!(
            err.reason
                .contains("is too large to return as a JavaScript number")
        );
    }
}
