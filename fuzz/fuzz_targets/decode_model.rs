//! Any bytes: decoding a model must not panic, and what decodes must encode stably.
#![no_main]

use libfuzzer_sys::fuzz_target;
use lunamodel_core::Model;
use lunamodel_fuzz::{assert_encoding_stable, decode};

fuzz_target!(|bytes: &[u8]| {
    if let Ok(model) = decode::<Model, _>(bytes, ()) {
        assert_encoding_stable(&model, ());
    }
});
