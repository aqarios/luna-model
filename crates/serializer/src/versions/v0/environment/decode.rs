use hashbrown::HashMap;
use lunamodel_core::{
    Environment,
    prelude::{LazyBounds, Variable},
};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bound, VarIdx, Vtype};
use prost::Message;
use std::collections::VecDeque;

use super::SerEnvironment;

use crate::encode::BytesDecodable;

impl BytesDecodable<Environment> for SerEnvironment {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<Environment> {
        Ok(Self::decode(bytes)?.extract())
    }
}

impl SerEnvironment {
    pub fn extract(&self) -> Environment {
        let mut variables = HashMap::with_capacity(self.variables_len as usize);
        let mut lookup = HashMap::with_capacity(self.variables_len as usize);
        self.extract_bin(&mut variables, &mut lookup);
        self.extract_spin(&mut variables, &mut lookup);
        self.extract_int(&mut variables, &mut lookup);
        self.extract_real(&mut variables, &mut lookup);
        let ivs = variables.into_iter().collect();
        Environment::new(ivs, lookup, self.next_idx)
    }

    fn extract_bin(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) {
        let mut inverted = VecDeque::from(self.inverted_binary.clone());
        for (i, vidx) in self.binary.iter().enumerate() {
            let name = self.binary_names[i].clone();
            let mut var = Variable::new(&name, Vtype::Binary, None).unwrap();
            if !self.binary_is_inverted.is_empty() && self.binary_is_inverted[i] {
                let inv = inverted.pop_front().unwrap();
                var.inverted = Some(inv);
                let invname = var.name().inverted();
                let invvar = Variable::new(&invname, Vtype::InvertedBinary, None).unwrap();
                variables.insert(inv, invvar);
                lookup.insert(invname.into(), inv);
            }
            variables.insert(*vidx, var);
            lookup.insert(name, *vidx);
        }
    }

    fn extract_spin(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) {
        for (i, vidx) in self.spin.iter().enumerate() {
            let name = self.spin_names[i].clone();
            let var = Variable::new(&name, Vtype::Spin, None).unwrap();
            variables.insert(*vidx, var);
            lookup.insert(name, *vidx);
        }
    }

    fn extract_int(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) {
        let mut lowers = VecDeque::from(self.integer_bounds_lower.clone());
        let mut uppers = VecDeque::from(self.integer_bounds_upper.clone());

        for (i, vidx) in self.integer.iter().enumerate() {
            let name = self.integer_names[i].clone();
            let lower_bound: Bound = match self.integer_bounds_has_lower[i] {
                true => Bound::Bounded(lowers.pop_front().unwrap()),
                false => Bound::Unbounded,
            };
            let upper_bound: Bound = match self.integer_bounds_has_upper[i] {
                true => Bound::Bounded(uppers.pop_front().unwrap()),
                false => Bound::Unbounded,
            };
            let bounds = Some(LazyBounds::new(Some(lower_bound), Some(upper_bound)));
            let var = Variable::new(&name, Vtype::Integer, bounds).unwrap();
            variables.insert(*vidx, var);
            lookup.insert(name, *vidx);
        }
    }

    fn extract_real(
        &self,
        variables: &mut HashMap<VarIdx, Variable>,
        lookup: &mut HashMap<String, VarIdx>,
    ) {
        let mut lowers = VecDeque::from(self.real_bounds_lower.clone());
        let mut uppers = VecDeque::from(self.real_bounds_upper.clone());

        for (i, vidx) in self.real.iter().enumerate() {
            let name = self.real_names[i].clone();
            let lower_bound: Bound = match self.real_bounds_has_lower[i] {
                true => Bound::Bounded(lowers.pop_front().unwrap()),
                false => Bound::Unbounded,
            };
            let upper_bound: Bound = match self.real_bounds_has_upper[i] {
                true => Bound::Bounded(uppers.pop_front().unwrap()),
                false => Bound::Unbounded,
            };
            let bounds = Some(LazyBounds::new(Some(lower_bound), Some(upper_bound)));
            let var = Variable::new(&name, Vtype::Real, bounds).unwrap();
            variables.insert(*vidx, var);
            lookup.insert(name, *vidx);
        }
    }
}
