//! Version 1 decoding for expressions.

use lunamodel_core::prelude::*;
use lunamodel_error::LunaModelResult;
use lunamodel_types::VarIdx;
use prost::Message;

use super::SerExpression;
use crate::check::{span, var};
use crate::encode::BytesDecodable;

/// Makes the SerExpression conform with the requirements for it to be a Decodable.
impl BytesDecodable<Expression, ArcEnv> for SerExpression {
    /// Decodes version-1 bytes into an expression tied to `payload`.
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<Expression> {
        Self::decode(bytes)?.extract(payload)
    }
}

impl SerExpression {
    /// Restores the quadratic term from the flattened protobuf fields.
    fn decode_quadratic(&self, next_idx: VarIdx) -> LunaModelResult<Option<Quadratic>> {
        if self.quad_size == 0 {
            return Ok(None);
        }
        let mut quad = Quadratic::default();
        let mut start = 0;
        for (&u, &len) in self
            .quad_neighborhood_indices
            .iter()
            .zip(&self.quad_neighborhoods_len)
        {
            let u = var(u, next_idx)?;
            let len = len as usize;
            let vs = span(&self.quad_neighborhoods, start, len, "quadratic terms")?;
            let bs = span(
                &self.quad_neighborhoods_values,
                start,
                len,
                "quadratic terms",
            )?;
            for (&v, &b) in vs.iter().zip(bs) {
                quad += (u, var(v, next_idx)?, b);
            }
            start += len;
        }

        Ok(Some(quad))
    }

    /// Restores the higher-order term from the flattened protobuf fields.
    fn decode_higher_order(&self, next_idx: VarIdx) -> LunaModelResult<Option<HigherOrder>> {
        if self.ho_size == 0 {
            return Ok(None);
        }

        let mut ho = HigherOrder::with_capacity(self.ho_lens.len());
        let mut start: usize = 0;
        for (&len, &value) in self.ho_lens.iter().zip(&self.ho_values) {
            let len = len as usize;
            let vars = span(&self.ho_indices, start, len, "higher-order terms")?;
            for &v in vars {
                var(v, next_idx)?;
            }
            ho += (vars, value);
            start += len;
        }

        Ok(Some(ho))
    }

    /// Legacy linear-term decoding path based on the active mask.
    fn decode_linear_old(&self, next_idx: VarIdx) -> LunaModelResult<Linear> {
        let mut lin = Linear::default();
        for (idx, (&active, &bias)) in self.active.iter().zip(&self.linear_values).enumerate() {
            if !active {
                continue;
            }
            lin += (var(idx as u32, next_idx)?, bias)
        }
        Ok(lin)
    }

    /// Sparse linear-term decoding path using explicit indices.
    fn decode_linear_new(&self, next_idx: VarIdx) -> LunaModelResult<Linear> {
        let mut lin = Linear::default();
        for (&idx, &bias) in self.linear_indices.iter().zip(&self.linear_values) {
            lin += (var(idx, next_idx)?, bias)
        }
        Ok(lin)
    }

    /// Dispatches between legacy and sparse linear decoding formats.
    fn decode_linear(&self, next_idx: VarIdx) -> LunaModelResult<Linear> {
        match self.is_new {
            true => self.decode_linear_new(next_idx),
            false => self.decode_linear_old(next_idx),
        }
    }

    /// Extracts the runtime expression from the protobuf structure.
    pub fn extract(self, env: ArcEnv) -> LunaModelResult<Expression> {
        let next_idx = env.read().next_idx();
        let linear = self.decode_linear(next_idx)?;
        let quadratic = self.decode_quadratic(next_idx)?;
        let higher_order = self.decode_higher_order(next_idx)?;
        Ok(Expression::empty(env).edit(|e| {
            e.offset = self.offset;
            e.linear = linear;
            e.quadratic = quadratic;
            e.higher_order = higher_order;
        }))
    }
}
