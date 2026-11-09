//! Any bytes: decoding an expression against a fixed environment must not panic,
//! and what decodes must encode stably.
#![no_main]

use libfuzzer_sys::fuzz_target;
use lunamodel_core::Expression;
use lunamodel_fuzz::{assert_encoding_stable, decode, expression_env};

fuzz_target!(|bytes: &[u8]| {
    let env = expression_env();
    if let Ok(expr) = decode::<Expression, _>(bytes, env.clone()) {
        assert_encoding_stable(&expr, env);
    }
});
