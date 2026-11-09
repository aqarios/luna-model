//! Version 0 decoding for environments.

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
    /// Decodes version-0 bytes into an environment.
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<Environment> {
        Self::decode(bytes)?.extract()
    }
}

impl SerEnvironment {
    /// Extracts the runtime environment from the protobuf structure.
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
        // v0 does not store the variable order, index order is the best approximation.
        let mut ivs: IndexMap<u32, Variable> = variables.into_iter().collect();
        ivs.sort_keys();
        let nxt_idx = first_unused(ivs.keys())?;
        Ok(Environment::new(ivs, lookup, nxt_idx))
    }

    /// Restores binary and inverted-binary variables.
    fn extract_bin(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) -> LunaModelResult<()> {
        for (i, &vidx) in self.binary.iter().enumerate() {
            let name = at(&self.binary_names, i, "binary variable name")?;
            insert(
                variables,
                lookup,
                vidx,
                variable(&name, Vtype::Binary, None)?,
                name,
            )?;
        }

        for (i, &vidx) in self.inverted_binary.iter().enumerate() {
            let name = at(
                &self.inverted_binary_names,
                i,
                "inverted binary variable name",
            )?;
            let mut var = variable(&name, Vtype::InvertedBinary, None)?;
            if let Some(&binidx) = lookup.get(&var.name().inverted().0) {
                let binvar = variables
                    .get_mut(&binidx)
                    .ok_or_else(|| corrupt("binary variable"))?;
                binvar.inverted = Some(vidx);
                var.inverted = Some(binidx);
            }
            insert(variables, lookup, vidx, var, name)?;
        }
        Ok(())
    }

    /// Restores spin variables.
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

    /// Restores integer variables and their bounds.
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

    /// Restores real variables and their bounds.
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
