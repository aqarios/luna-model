use std::path::PathBuf;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

pub struct LpTranslator;

impl LpTranslator {
    pub fn translate(file: String) -> LunaModelResult<Model> {
        _ = file;
        unimplemented!("LpTranslator logic is not yet implemented!")
    }

    pub fn back_translate(
        model: &Model,
        filepath: Option<PathBuf>,
    ) -> LunaModelResult<Option<String>> {
        _ = model;
        _ = filepath;
        unimplemented!("LpTranslator logic (from model) is not yet implemented!")
    }
}
