//! Version 1 decoding for expressions.

use lunamodel_core::prelude::*;
use lunamodel_error::LunaModelResult;
use prost::Message;

use super::SerExpression;
use crate::encode::BytesDecodable;

/// Makes the SerExpression conform with the requirements for it to be a Decodable.
impl BytesDecodable<Expression, ArcEnv> for SerExpression {
    /// Decodes version-1 bytes into an expression tied to `payload`.
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<Expression> {
        Ok(Self::decode(bytes)?.extract(payload))
    }
}

impl SerExpression {
    /// Restores the quadratic term from the flattened protobuf fields.
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

    /// Restores the higher-order term from the flattened protobuf fields.
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

    /// Legacy linear-term decoding path based on the active mask.
    fn decode_linear_old(&self) -> Linear {
        let mut lin = Linear::default();
        for (idx, (&active, &bias)) in self.active.iter().zip(&self.linear_values).enumerate() {
            if !active {
                continue;
            }
            lin += (idx as u32, bias)
        }
        lin
    }

    /// Sparse linear-term decoding path using explicit indices.
    fn decode_linear_new(&self) -> Linear {
        let mut lin = Linear::default();
        for (&idx, &bias) in self.linear_indices.iter().zip(&self.linear_values) {
            lin += (idx, bias)
        }
        lin
    }

    /// Dispatches between legacy and sparse linear decoding formats.
    fn decode_linear(&self) -> Linear {
        match self.is_new {
            true => self.decode_linear_new(),
            false => self.decode_linear_old(),
        }
    }

    /// Extracts the runtime expression from the protobuf structure.
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
