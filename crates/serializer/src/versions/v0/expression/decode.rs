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
        for (&u, &len) in self
            .quad_neighborhood_indices
            .iter()
            .zip(&self.quad_neighborhoods_len)
        {
            let end = start + len;
            for i in start..end {
                let v = self.quad_neighborhoods[i as usize];
                let b = self.quad_neighborhoods_values[i as usize];
                quad += (u, v, b);
            }
            start = end;
        }

        Some(quad)
    }

    fn decode_higher_order(&self) -> Option<HigherOrder> {
        if self.ho_size == 0 {
            return None;
        }

        let mut ho = HigherOrder::with_capacity(self.ho_size as usize);
        let mut start: usize = 0;
        for (&len, &value) in self.ho_lens.iter().zip(&self.ho_values) {
            let end = start + (len as usize);
            let contribs = self.ho_indices[start..end].to_vec();
            ho += (contribs.as_slice(), value);
            start = end;
        }

        Some(ho)
    }

    fn decode_linear_old(&self) -> Linear {
        let mut lin = Linear::default();
        for (idx, (&active, &bias)) in self.active.iter().zip(&self.linear).enumerate() {
            if !active {
                continue;
            }
            lin += (idx as u32, bias)
        }
        lin
    }

    fn decode_linear(&self) -> Linear {
        self.decode_linear_old()
    }

    /// Extracts the data from self to and instance of Expression with Index VarId and
    /// Bias f64.
    pub fn extract(self, env: ArcEnv) -> Expression {
        Expression::empty(env).edit(|e| {
            e.offset = self.offset;
            e.linear = self.decode_linear();
            e.quadratic = self.decode_quadratic();
            e.higher_order = self.decode_higher_order();
            // e.num_vars = self.num_variables as usize;
        })
    }
}
