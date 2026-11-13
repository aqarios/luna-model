//! Checks shared by the fuzz targets.

use lunamodel_core::{Model, prelude::ContentEquality};
use lunamodel_translate::model::{LpTranslator, MpsTranslator};

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
