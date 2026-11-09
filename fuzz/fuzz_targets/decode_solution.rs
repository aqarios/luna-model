//! Any bytes: decoding a solution must not panic, and what decodes must encode stably.
#![no_main]

use libfuzzer_sys::fuzz_target;
use lunamodel_core::Solution;
use lunamodel_fuzz::{assert_encoding_stable, decode};

fuzz_target!(|bytes: &[u8]| {
    if let Ok(solution) = decode::<Solution, _>(bytes, ()) {
        assert_encoding_stable(&solution, ());
    }
});
