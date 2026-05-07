use lunamodel_core::Solution as CoreSolution;
use lunamodel_serializer::prelude::*;
use napi::bindgen_prelude::*;
use napi_derive::napi;

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
}

