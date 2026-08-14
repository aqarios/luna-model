//! Version 0 decoding for models.

use std::str::FromStr;

use lunamodel_core::{ArcEnv, Environment, prelude::Model};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Sense;
use prost::Message;

use crate::encode::{BytesDecodable, Decodable};
use crate::versionize::Unversionizable;

use super::SerModel;

impl BytesDecodable<Model> for SerModel {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<Model> {
        Self::decode(bytes)?.extract()
    }
}

impl SerModel {
    fn extract(&self) -> LunaModelResult<Model> {
        let sense = Sense::from_str(&self.sense)
            .map_err(|e| LunaModelError::Decoding(e.to_string().into()))?;
        let env: Environment = self.environment.as_slice().unversionize().decode(())?;
        let arcenv = ArcEnv::from(env);
        let mut model = Model::with_env(Some(self.name.clone()), Some(sense), arcenv.clone());
        model.objective = self
            .objective
            .as_slice()
            .unversionize()
            .decode(arcenv.clone())?;
        model.constraints = self
            .constraints
            .as_slice()
            .unversionize()
            .decode(arcenv.clone())?;
        Ok(model)
    }
}
