//! Version 1 decoding for environments.

use indexmap::IndexMap;
use lunamodel_core::{
    Environment,
    prelude::{LazyBounds, Variable},
};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bound, VarIdx, Vtype};
use prost::Message;
use std::collections::HashMap;

use super::SerEnvironment;

use crate::check::{at, corrupt, first_unused, insert, next, variable};
use crate::encode::BytesDecodable;

impl BytesDecodable<Environment> for SerEnvironment {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<Environment> {
        Self::decode(bytes)?.extract()
    }
}

impl SerEnvironment {
    pub fn extract(&self) -> LunaModelResult<Environment> {
        // Sized from the data present, not from a stored count.
        let capacity = self.binary.len()
            + self.inverted_binary.len()
            + self.spin.len()
            + self.integer.len()
            + self.real.len();
        let mut variables = HashMap::with_capacity(capacity);
        let mut lookup = HashMap::with_capacity(capacity);
        self.extract_bin(&mut variables, &mut lookup)?;
        self.extract_spin(&mut variables, &mut lookup)?;
        self.extract_int(&mut variables, &mut lookup)?;
        self.extract_real(&mut variables, &mut lookup)?;
        let ivs = self.ordered(variables);
        let lowest_free = first_unused(ivs.keys())?;
        let nxt_idx = match self.next_idx {
            Some(idx) if idx < lowest_free => return Err(corrupt("next variable index in use")),
            Some(idx) => idx,
            None => lowest_free,
        };
        Ok(Environment::new(ivs, lookup, nxt_idx))
    }

    /// Arranges the decoded variables in their serialized environment order.
    ///
    /// Variables not covered by the stored order (e.g. data written before the
    /// order was serialized) are appended in index order, which matches the
    /// insertion order of environments that were never reordered.
    fn ordered(&self, mut variables: HashMap<VarIdx, Variable>) -> IndexMap<VarIdx, Variable> {
        let mut ivs: IndexMap<VarIdx, Variable> = self
            .order
            .iter()
            .filter_map(|idx| variables.remove_entry(idx))
            .collect();
        let mut rest: IndexMap<VarIdx, Variable> = variables.into_iter().collect();
        rest.sort_keys();
        ivs.extend(rest);
        ivs
    }

    fn extract_bin(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) -> LunaModelResult<()> {
        let mut inverted = self.inverted_binary.iter();
        for (i, &vidx) in self.binary.iter().enumerate() {
            let name = at(&self.binary_names, i, "binary variable name")?;
            let mut var = variable(&name, Vtype::Binary, None)?;
            if !self.binary_is_inverted.is_empty()
                && at(&self.binary_is_inverted, i, "binary inversion flag")?
            {
                let inv = *next(&mut inverted, "inverted binary variable")?;
                var.inverted = Some(inv);
                let invname = var.name().inverted();
                let invvar = variable(&invname.0, Vtype::InvertedBinary, None)?;
                insert(variables, lookup, inv, invvar, invname.0)?;
            }
            insert(variables, lookup, vidx, var, name)?;
        }
        Ok(())
    }

    fn extract_spin(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) -> LunaModelResult<()> {
        for (i, &vidx) in self.spin.iter().enumerate() {
            let name = at(&self.spin_names, i, "spin variable name")?;
            insert(
                variables,
                lookup,
                vidx,
                variable(&name, Vtype::Spin, None)?,
                name,
            )?;
        }
        Ok(())
    }

    fn extract_int(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) -> LunaModelResult<()> {
        let mut lowers = self.integer_bounds_lower.iter();
        let mut uppers = self.integer_bounds_upper.iter();

        for (i, &vidx) in self.integer.iter().enumerate() {
            let name = at(&self.integer_names, i, "integer variable name")?;
            let lower_bound: Bound =
                match at(&self.integer_bounds_has_lower, i, "lower bound flag")? {
                    true => Bound::Bounded(*next(&mut lowers, "integer lower bound")?),
                    false => Bound::Unbounded,
                };
            let upper_bound: Bound =
                match at(&self.integer_bounds_has_upper, i, "upper bound flag")? {
                    true => Bound::Bounded(*next(&mut uppers, "integer upper bound")?),
                    false => Bound::Unbounded,
                };
            let bounds = Some(LazyBounds::new(Some(lower_bound), Some(upper_bound)));
            insert(
                variables,
                lookup,
                vidx,
                variable(&name, Vtype::Integer, bounds)?,
                name,
            )?;
        }
        Ok(())
    }

    fn extract_real(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) -> LunaModelResult<()> {
        let mut lowers = self.real_bounds_lower.iter();
        let mut uppers = self.real_bounds_upper.iter();

        for (i, &vidx) in self.real.iter().enumerate() {
            let name = at(&self.real_names, i, "real variable name")?;
            let lower_bound: Bound = match at(&self.real_bounds_has_lower, i, "lower bound flag")? {
                true => Bound::Bounded(*next(&mut lowers, "real lower bound")?),
                false => Bound::Unbounded,
            };
            let upper_bound: Bound = match at(&self.real_bounds_has_upper, i, "upper bound flag")? {
                true => Bound::Bounded(*next(&mut uppers, "real upper bound")?),
                false => Bound::Unbounded,
            };
            let bounds = Some(LazyBounds::new(Some(lower_bound), Some(upper_bound)));
            insert(
                variables,
                lookup,
                vidx,
                variable(&name, Vtype::Real, bounds)?,
                name,
            )?;
        }
        Ok(())
    }
}
