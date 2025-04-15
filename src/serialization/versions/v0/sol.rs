use std::rc::Rc;

use prost::Message;

use crate::{
    core::{
        solution::sol::SampleCol, ConcreteSolution, RcSolution, Solution, VarAssignment, Vtype,
    },
    serialization::encodable::{BytesDecodable, BytesEncodable, Creatable, DecodeError},
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

    #[prost(bytes, tag = 110)]
    bins: Vec<u8>,
    #[prost(uint64, repeated, tag = 111)]
    bins_pos: Vec<u64>,
    #[prost(uint64, repeated, tag = 112)]
    bin_sample_association: Vec<u64>,

    #[prost(int32, repeated, tag = 120)]
    spins: Vec<i32>,
    #[prost(uint64, repeated, tag = 121)]
    spins_pos: Vec<u64>,
    #[prost(uint64, repeated, tag = 122)]
    spin_sample_association: Vec<u64>,

    #[prost(int64, repeated, tag = 130)]
    ints: Vec<i64>,
    #[prost(uint64, repeated, tag = 131)]
    ints_pos: Vec<u64>,
    #[prost(uint64, repeated, tag = 132)]
    int_sample_association: Vec<u64>,

    #[prost(double, repeated, tag = 140)]
    reals: Vec<f64>,
    #[prost(uint64, repeated, tag = 141)]
    reals_pos: Vec<u64>,
    #[prost(uint64, repeated, tag = 142)]
    real_sample_association: Vec<u64>,

    /// The number of occurences for each sample in the solution.
    #[prost(uint64, repeated, tag = 30)]
    num_occurrences: Vec<u64>,

    /// The objective value for each sample in the solution
    #[prost(double, repeated, tag = 40)]
    obj_values: Vec<f64>,
    /// If a sample has an objective value stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 41)]
    has_obj_value: Vec<bool>,

    /// The raw energies for each sample in the solution
    #[prost(double, repeated, tag = 50)]
    raw_energies: Vec<f64>,
    /// If a sample has a raw energy stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 51)]
    has_raw_energy: Vec<bool>,

    /// The index of the best sample
    #[prost(uint64, optional, tag = 60)]
    best_sample_idx: Option<u64>,
    // /// Runtime metrics of the solution.
    // pub timing: Option<Timing>,
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
        Ok(Self::decode(bytes)?.extract())
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
        for ((i, sample), &occ) in solution
            .samples()
            .iter()
            .enumerate()
            .zip(&solution.num_occurrences)
        {
            let mut sample_len: u32 = 0;
            for (pos, a) in sample.iter().enumerate() {
                sample_len += 1;
                match a {
                    VarAssignment::Binary(v) => {
                        self.bins.push(v);
                        self.bins_pos.push(pos as u64);
                        self.bin_sample_association.push(i as u64);
                    }
                    VarAssignment::Spin(v) => {
                        self.spins.push(v as i32);
                        self.spins_pos.push(pos as u64);
                        self.spin_sample_association.push(i as u64);
                    }
                    VarAssignment::Integer(v) => {
                        self.ints.push(v);
                        self.ints_pos.push(pos as u64);
                        self.int_sample_association.push(i as u64);
                    }
                    VarAssignment::Real(v) => {
                        self.reals.push(v);
                        self.reals_pos.push(pos as u64);
                        self.real_sample_association.push(i as u64);
                    }
                };
            }
            self.sample_len = sample_len as u32;
            self.num_occurrences.push(occ as u64);

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
        self
    }

    pub fn extract(&self) -> ConcreteSolution {
        let mut sol = Solution::default();

        let num_samples = self.num_samples as usize;
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
        }

        // todo!();

        RcSolution(Rc::new(sol))
    }
}
