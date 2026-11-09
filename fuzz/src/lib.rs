//! Checks shared by the fuzz targets.

use lunamodel_core::{ArcEnv, Model, prelude::ContentEquality};
use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{
    Creatable, Decodable, Decompressable, Encodable, Unversionizable, Versioned,
};
use lunamodel_translate::model::{LpTranslator, MpsTranslator};
use lunamodel_types::Vtype;

/// A model the LP or MPS writer accepts must read back unchanged.
pub fn assert_file_roundtrips(model: &Model) {
    if let Ok(Some(lp)) = LpTranslator::back_translate(model, None) {
        let back = LpTranslator::translate(lp.clone()).unwrap_or_else(|e| panic!("{e}\n{lp}"));
        assert!(back.equal_contents(model), "LP changed the model:\n{lp}");
    }
    if let Ok(Some(mps)) = MpsTranslator::back_translate(model, None) {
        let back = MpsTranslator::translate(mps.clone()).unwrap_or_else(|e| panic!("{e}\n{mps}"));
        assert!(back.equal_contents(model), "MPS changed the model:\n{mps}");
    }
}

/// Decodes `bytes` the way the Python bindings do.
pub fn decode<T, P>(bytes: &[u8], payload: P) -> LunaModelResult<T>
where
    Versioned<Vec<u8>>: Decodable<T, Payload = P>,
{
    Ok(bytes.unversionize().decompress()?.decode(payload)?)
}

/// A decoded value must encode to bytes that decode and encode to the same bytes.
/// Comparing bytes instead of values keeps NaN, which equals nothing, comparable.
pub fn assert_encoding_stable<T, S, P>(value: &T, payload: P)
where
    T: Encodable<S>,
    S: Creatable<T>,
    Versioned<Vec<u8>>: Decodable<T, Payload = P>,
{
    let once = value.encode(None, None).unwrap();
    let back: T = decode(&once, payload).unwrap();
    assert_eq!(back.encode(None, None).unwrap(), once);
}

/// The environment `decode_expression` decodes against: one variable per type.
pub fn expression_env() -> ArcEnv {
    let env = ArcEnv::default();
    for (name, vtype) in [
        ("b", Vtype::Binary),
        ("s", Vtype::Spin),
        ("i", Vtype::Integer),
        ("r", Vtype::Real),
    ] {
        env.clone().insert(name, vtype, None).unwrap();
    }
    env
}
