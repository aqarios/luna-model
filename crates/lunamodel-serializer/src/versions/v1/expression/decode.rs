use lunamodel_core::{ArcEnv, Expression};
use lunamodel_error::LunaModelResult;

use super::SerExpression;
use crate::encode::BytesDecodable;

/// Makes the SerExpression conform with the requirements for it to be a Decodable.
impl BytesDecodable<Expression, ArcEnv> for SerExpression {
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<Expression> {
        Ok(Self::decode(bytes)?.extract(payload))
    }
}
