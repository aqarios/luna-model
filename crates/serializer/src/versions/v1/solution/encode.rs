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

        let mut flat: Vec<bool> = Vec::default();
        for (name, vals) in &sol.constraints {
            self.n_constraints = vals.len() as u64;
            flat.extend(vals);
            if self.constraint_names.len() != vals.len() {
                self.constraint_names.push(name.clone());
            }
        }
        self.constraints = flat.into_iter().collect::<BitVec<u8, Lsb0>>().into_vec();

        let mut flat: Vec<bool> = Vec::default();
        for (name, vals) in &sol.variable_bounds {
            self.n_variable_bounds = vals.len() as u64;
            flat.extend(vals);
            if self.variable_bound_names.len() != vals.len() {
                self.variable_bound_names.push(name.clone());
            }
        }
        self.variable_bounds = flat.into_iter().collect::<BitVec<u8, Lsb0>>().into_vec();

        // done
        self
    }
}
