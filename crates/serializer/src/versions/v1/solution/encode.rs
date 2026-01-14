use bitvec::{order::Lsb0, vec::BitVec};
use lunamodel_core::{Solution, solution::Column};
use lunamodel_types::Vtype;
use prost::Message;

use crate::encode::{BytesEncodable, Encodable};
use crate::utils::vtype_to_u8;

use super::SerSolution;

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerSolution {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerSolution {
    pub fn fill(mut self, sol: &Solution) -> SerSolution {
        self.num_samples = sol.n_samples() as u64;
        self.variable_names = sol.variable_names();
        self.sense = sol.sense.to_string();
        self.timing = sol.timing.map(|t| t.serialize());
        self.counts = sol.counts.iter().map(|&c| c as u64).collect();
        self.obj_values = sol.obj_values.clone().unwrap_or_else(Vec::default);
        self.raw_energies = sol.raw_energies.clone().unwrap_or_else(Vec::default);

        let mut binvec: BitVec<u8, Lsb0> = BitVec::new();
        let mut spinvec: BitVec<u8, Lsb0> = BitVec::new();

        for (_, col) in sol.samples.iter() {
            match col {
                Column::Binary(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Binary));
                    self.n_bins += 1;
                    binvec.extend(inner.iter().map(|b| b == 1).into_iter());
                }
                Column::Spin(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Spin));
                    self.n_spins += 1;
                    spinvec.extend(inner.iter().map(|s| s == -1).into_iter());
                }
                Column::Integer(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Integer));
                    self.ints.extend(inner.iter());
                }
                Column::Real(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Real));
                    self.reals.extend(inner.iter());
                }
            }
        }

        self.bins = binvec.into_vec();
        self.spins = spinvec.into_vec();

        self.n_constraints = sol.constraints.len() as u64;
        let mut constrs: Vec<Vec<bool>> =
            vec![vec![true; sol.constraints.len()]; self.num_samples as usize];
        self.constraint_names = Vec::new();
        for (idx, (name, vals)) in sol.constraints.iter().enumerate() {
            self.constraint_names.push(name.clone());
            for (s_idx, value) in vals.iter().enumerate() {
                constrs[s_idx][idx] = *value;
            }
        }
        self.constraints = constrs.into_iter().flatten().collect::<BitVec<u8, Lsb0>>().into_vec();

        self.n_variable_bounds = sol.variable_bounds.len() as u64;
        let mut vbounds: Vec<Vec<bool>> =
            vec![vec![true; sol.variable_bounds.len()]; self.num_samples as usize];
        self.variable_bound_names = Vec::new();
        for (idx, (name, vals)) in sol.variable_bounds.iter().enumerate() {
            self.variable_bound_names.push(name.clone());
            for (s_idx, value) in vals.iter().enumerate() {
                vbounds[s_idx][idx] = *value;
            }
        }
        self.variable_bounds = vbounds.into_iter().flatten().collect::<BitVec<u8, Lsb0>>().into_vec();

        self
    }
}