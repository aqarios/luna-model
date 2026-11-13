//! Any text: the MPS reader must not panic, and what it reads must round-trip.
#![no_main]

use libfuzzer_sys::fuzz_target;
use lunamodel_fuzz::assert_file_roundtrips;
use lunamodel_translate::model::MpsTranslator;

fuzz_target!(|text: &str| {
    if let Ok(model) = MpsTranslator::translate(text.to_string()) {
        assert_file_roundtrips(&model);
    }
});
