use std::rc::Rc;

use prost::Message;

use crate::serialization::{Decodable, Encodable};
use crate::{
    core::{
        solution::sol::SampleCol, ConcreteSolution, RcSolution, Solution, VarAssignment, Vtype,
    },
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable, DecodeError},
        utils::force_i8,
    },
};

fn assignment_type_to_u8(vtype: Vtype) -> u8 {
    match vtype {
        Vtype::Binary => 0,
        Vtype::Spin => 1,
        Vtype::Integer => 2,
        Vtype::Real => 3,
    }
}

fn u8_to_assignment_type(u: u8) -> Vtype {
    match u {
        0 => Vtype::Binary,
        1 => Vtype::Spin,
        2 => Vtype::Integer,
        3 => Vtype::Real,
        _ => panic!("issue"),
    }
}

/// Representation of encodable solution based on protocol buffers.
#[derive(Clone, PartialEq, Message)]
pub struct SerSolution {
    /// The number of samples in the solution.
    #[prost(uint64, tag = 1)]
    num_samples: u64,

    /// The length of each sample
    #[prost(uint32, tag = 2)]
    sample_len: u32,
    /// The type for each element in a sample
    #[prost(bytes, tag = 3)]
    sample_types: Vec<u8>,

    #[prost(bytes, tag = 4)]
    bins: Vec<u8>,

    #[prost(int32, repeated, tag = 5)]
    spins: Vec<i32>,

    #[prost(int64, repeated, tag = 6)]
    ints: Vec<i64>,

    #[prost(double, repeated, tag = 7)]
    reals: Vec<f64>,

    /// The number of occurences for each sample in the solution.
    #[prost(uint64, repeated, tag = 8)]
    counts: Vec<u64>,

    /// The objective value for each sample in the solution
    #[prost(double, repeated, tag = 9)]
    obj_values: Vec<f64>,

    /// If a sample has an objective value stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 10)]
    has_obj_value: Vec<bool>,

    /// The raw energies for each sample in the solution
    #[prost(double, repeated, tag = 11)]
    raw_energies: Vec<f64>,

    /// If a sample has a raw energy stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 12)]
    has_raw_energy: Vec<bool>,

    /// The index of the best sample
    #[prost(uint64, optional, tag = 13)]
    best_sample_idx: Option<u64>,

    /// Runtime metrics of the solution.
    #[prost(bytes, optional, tag = 14)]
    timing: Option<Vec<u8>>,

    /// The variable names
    #[prost(string, repeated, tag = 15)]
    variable_names: Vec<String>,
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerSolution {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<ConcreteSolution> for SerSolution {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<ConcreteSolution, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl Creatable<ConcreteSolution> for SerSolution {
    fn new(value: &ConcreteSolution) -> Self {
        Self::default().fill(&value)
    }
}

impl SerSolution {
    /// Fills the serializable solution based on an instance of Solution.
    fn fill(mut self, solution: &ConcreteSolution) -> Self {
        let samples = solution.samples();
        for ((i, sample), &occ) in solution.samples().iter().enumerate().zip(&solution.counts) {
            // for (pos, a) in sample.iter().enumerate() {
            for a in sample.iter() {
                match a {
                    VarAssignment::Binary(v) => {
                        self.bins.push(v);
                        // self.bins_pos.push(pos as u64);
                        // self.bin_sample_association.push(i as u64);
                    }
                    VarAssignment::Spin(v) => {
                        self.spins.push(v as i32);
                        // self.spins_pos.push(pos as u64);
                        // self.spin_sample_association.push(i as u64);
                    }
                    VarAssignment::Integer(v) => {
                        self.ints.push(v);
                        // self.ints_pos.push(pos as u64);
                        // self.int_sample_association.push(i as u64);
                    }
                    VarAssignment::Real(v) => {
                        self.reals.push(v);
                        // self.reals_pos.push(pos as u64);
                        // self.real_sample_association.push(i as u64);
                    }
                };
            }
            self.sample_types = if solution.len() > 0 {
                let s = solution.samples().get_sample(0).unwrap();
                s.iter()
                    .map(|a| match a {
                        VarAssignment::Binary(_) => assignment_type_to_u8(Vtype::Binary),
                        VarAssignment::Spin(_) => assignment_type_to_u8(Vtype::Spin),
                        VarAssignment::Integer(_) => assignment_type_to_u8(Vtype::Integer),
                        VarAssignment::Real(_) => assignment_type_to_u8(Vtype::Real),
                    })
                    .collect()
            } else {
                Vec::new()
            };
            self.sample_len = solution.samples.len() as u32;
            self.counts.push(occ as u64);

            if let Some(res) = solution.get_result_view(i) {
                if let Some(ov) = res.obj_value() {
                    self.obj_values.push(ov);
                    self.has_obj_value.push(true);
                } else {
                    self.has_obj_value.push(false);
                }

                if let Some(en) = res.raw_energy() {
                    self.has_raw_energy.push(true);
                    self.raw_energies.push(en);
                } else {
                    self.has_raw_energy.push(false);
                }
            } else {
                self.has_obj_value.push(false);
                self.has_raw_energy.push(false);
            }
        }

        self.num_samples = samples.len() as u64;
        self.best_sample_idx = solution.best_sample_idx.and_then(|v| Some(v as u64));
        self.timing = solution.timing.map(|t| t.encode());
        self
    }

    pub fn extract(&self) -> Result<ConcreteSolution, DecodeError> {
        let mut sol = Solution::default();
        let num_samples = self.num_samples as usize;
        let mut type_per_pos: Vec<Vtype> = Vec::new();
        for &st in self.sample_types.iter() {
            let vt = u8_to_assignment_type(st);
            match vt {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(num_samples))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(num_samples))),
                Vtype::Integer => {
                    sol.add_column(SampleCol::Integer(Vec::with_capacity(num_samples)))
                }
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(num_samples))),
            }
            type_per_pos.push(vt);
        }

        sol.counts = self.counts.iter().map(|&v| v as usize).collect();
        sol.n_samples = self.num_samples as usize;

        let (mut lb, mut ls, mut li, mut lr) = (0, 0, 0, 0);
        let sample_len = self.sample_len as usize;
        for _ in 0..num_samples {
            for j in 0..sample_len {
                // let pos = i * num_samples + j;
                // println!("{pos}");
                match &type_per_pos[j] {
                    Vtype::Binary => {
                        sol.samples[j]
                            .push(self.bins[lb])
                            .expect("something went wrong");
                        lb += 1;
                    }
                    Vtype::Spin => {
                        sol.samples[j]
                            .push(force_i8(self.spins[ls]))
                            .expect("something went wrong");
                        ls += 1;
                    }
                    Vtype::Integer => {
                        sol.samples[j]
                            .push(self.ints[li])
                            .expect("something went wrong");
                        li += 1;
                    }
                    Vtype::Real => {
                        sol.samples[j]
                            .push(self.reals[lr])
                            .expect("something went wrong");
                        lr += 1;
                    }
                };
            }
        }
        sol.obj_values = vec![None; num_samples];
        if !self.obj_values.is_empty() {
            let mut idx = 0;
            for (i, &has_val) in self.has_obj_value.iter().enumerate() {
                if has_val {
                    sol.obj_values[i] = Some(self.obj_values[idx]);
                    idx += 1;
                }
            }
        }
        sol.raw_energies = vec![None; num_samples];
        if !self.raw_energies.is_empty() {
            let mut idx = 0;
            for (i, &has_val) in self.has_raw_energy.iter().enumerate() {
                if has_val {
                    sol.raw_energies[i] = Some(self.raw_energies[idx]);
                    idx += 1;
                }
            }
        }

        sol.best_sample_idx = self.best_sample_idx.map(|idx| idx as usize);

        if let Some(t) = &self.timing {
            sol.timing = Some(t.decode(())?);
        }

        Ok(RcSolution(Rc::new(sol)))
    }
}
