use lunamodel_core::prelude::*;
use lunamodel_error::LunaModelResult;

use prost::Message;

use super::SerExpression;
use crate::encode::BytesDecodable;

/// Makes the SerExpression conform with the requirements for it to be a Decodable.
impl BytesDecodable<Expression, ArcEnv> for SerExpression {
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<Expression> {
        Ok(Self::decode(bytes)?.extract(payload))
    }
}

impl SerExpression {
    fn decode_quadratic(&self) -> Option<Quadratic> {
        if self.quad_size == 0 {
            return None;
        }
        let mut quad = Quadratic::default();
        let mut start = 0;
        for (u, len) in self
            .quad_neighborhood_indices
            .iter()
            .zip(&self.quad_neighborhoods_len)
        {
            let end = start + len;
            for i in start..end {
                quad[*u as usize].push(OneVarTerm::new(
                    VarId(self.quad_neighborhoods[i as usize]),
                    self.quad_neighborhoods_values[i as usize],
                ));
            }
            start = end;
        }

        Some(quad)
    }

    fn decode_higher_order(&self) -> Option<HigherOrder> {
        if self.ho_size == 0 {
            return None;
        }

        let mut ho = HigherOrder::with_size(self.ho_size as usize);

        let mut start: usize = 0;
        for (len, value) in self.ho_lens.iter().zip(&self.ho_values) {
            let end = start + (*len as usize);
            let contribs = self.ho_indices[start..end]
                .iter()
                .map(|u| VarId(*u))
                .collect::<Vec<VarId>>();
            ho[&contribs] = *value;
            start = end;
        }

        Some(ho)
    }

    /// Extracts the data from self to and instance of Expression with Index VarId and
    /// Bias f64.
    pub fn extract(self, env: ArcEnv) -> Expression {
        let mut expr = Expression::empty(env);
        expr.num_variables = self.num_variables as usize;
        expr.offset = self.offset;
        expr.linear = Linear::new(self.linear.clone()); // todo(team): might be optimizable with mem copies. See somewhere in code where I do something similar.
        expr.quadratic = self.decode_quadratic();
        expr.higher_order = self.decode_higher_order();
        expr
    }
}
