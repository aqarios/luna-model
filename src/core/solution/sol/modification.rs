use super::{column::Column, Solution};
use crate::{
    core::{traits::PushOrCreate, SharedEnvironment, Vtype},
    errors::{SampleIncorrectLengthErr, SolutionCreationErr},
    types::{
        Bias, BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
        VarIndex,
    },
};
use num::NumCast;

impl Solution {
    /// Extend a solution with a sample, without computing any objective values or similar.
    /// This method does not check whether the sample is already part of the solution as for now the
    /// solution translator is expected to do the aggregation.
    pub fn extend<S: Copy + NumCast>(
        &mut self,
        sample: &Vec<S>,
        counts: usize,
        energy: Bias,
    ) -> Result<&mut Self, SolutionCreationErr> {
        self.add_sample(sample)?;
        self.counts.push(counts);
        self.raw_energies.push_or_create(energy);
        self.n_samples += 1;
        Ok(self)
    }

    fn add_sample<T: Copy + NumCast>(
        &mut self,
        sample: &Vec<T>,
    ) -> Result<(), SolutionCreationErr> {
        if sample.len() != self.samples.len() {
            Err(SampleIncorrectLengthErr)?
        } else {
            for (i, &a) in sample.iter().enumerate() {
                self.samples[i].push(a)?;
            }
            Ok(())
        }
    }

    pub fn create_columns(&mut self, env: &SharedEnvironment, capacity: usize) {
        for (idx, v) in env.borrow().all_variables().enumerate() {
            self.add_new_col_for(idx.into(), v.vtype, capacity);
        }
    }

    pub fn add_eval_data(
        &mut self,
        objective_value: Vec<Bias>,
        constraints: Vec<Vec<bool>>,
        variable_bounds: Vec<Vec<bool>>,
    ) {
        let f = constraints
            .iter()
            .zip(&variable_bounds)
            .map(|(c, v)| c.iter().all(|&b| b) && v.iter().all(|&b| b))
            .collect();

        self.feasible = Some(f);
        self.obj_values = Some(objective_value);
        self.constraints = Some(constraints);
        self.variable_bounds = Some(variable_bounds);
        // todo: add best sample index comp here or during getter dynamically?
        // I'd suggest when field is accessed.
    }
}

impl Solution {
    pub fn add_column(&mut self, column: Column) {
        self.samples.push(column);
    }

    pub fn add_new_col_for(&mut self, varid: VarIndex, vtype: Vtype, capacity: usize) {
        match vtype {
            Vtype::Binary => self.add_binary_col(varid, Vec::with_capacity(capacity)),
            Vtype::Spin => self.add_spin_col(varid, Vec::with_capacity(capacity)),
            Vtype::Integer => self.add_integer_col(varid, Vec::with_capacity(capacity)),
            Vtype::Real => self.add_real_col(varid, Vec::with_capacity(capacity)),
            Vtype::__Ghost => (),
        }
    }

    pub fn add_binary_col(&mut self, varid: VarIndex, data: Vec<BinaryAssignmentType>) {
        self.add_column(Column::new_binary(varid, data))
    }

    pub fn add_spin_col(&mut self, varid: VarIndex, data: Vec<SpinAssignmentType>) {
        self.add_column(Column::new_spin(varid, data))
    }

    pub fn add_integer_col(&mut self, varid: VarIndex, data: Vec<IntegerAssignmentType>) {
        self.add_column(Column::new_integer(varid, data))
    }

    pub fn add_real_col(&mut self, varid: VarIndex, data: Vec<RealAssignmentType>) {
        self.add_column(Column::new_real(varid, data))
    }
}
