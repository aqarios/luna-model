use super::{column::Column, Solution, VarKey};
use crate::{
    core::{SharedEnvironment, VarRef, Vtype},
    errors::{ColumnCreationErr, SampleIncorrectLengthErr, SolutionCreationErr},
    types::{
        Bias, BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
        VarIndex,
    },
};
use itertools::Itertools;
use num::NumCast;

impl Solution {
    /// Extend a solution with a sample, without computing any objective values or similar.
    /// This method does not check whether the sample is already part of the solution as for now the
    /// solution translator is expected to do the aggregation.
    pub fn extend<S: Copy + NumCast>(
        &mut self,
        sample: &Vec<S>,
        counts: usize,
        energy: Option<Bias>,
    ) -> Result<&mut Self, SolutionCreationErr> {
        self.add_sample(sample)?;
        self.counts.push(counts);
        self.raw_energies.push(energy);
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

impl Solution {
    pub fn add_samplecol<N: NumCast + Copy>(
        &mut self,
        var: VarKey,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), ColumnCreationErr> {
        match var {
            VarKey::Name(varname) => self.add_samplecol_for_varname(varname, data, vtype),
            VarKey::Var(var) => self.add_samplecol_for_var(&var, data, vtype),
        }
    }

    pub fn add_samplecol_for_var<N: NumCast + Copy>(
        &mut self,
        var: &VarRef,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), ColumnCreationErr> {
        let varname = var
            .env
            .borrow()
            .get_for_index(var.id)
            .map_err(|e| ColumnCreationErr::new(&e.to_string()))?
            .name
            .clone();
        self.variable_names.push(varname);
        self.add_column(Column::try_new(data, var.id, vtype)?);
        // todo: adjust other values and fix logic after restructuring the solution
        // internally.
        Ok(())
    }
    pub fn add_samplecol_for_varname<N: NumCast + Copy>(
        &mut self,
        varname: String,
        data: &[N],
        vtype: Vtype,
    ) -> Result<(), ColumnCreationErr> {
        let varid = self.variable_names.len();
        self.add_column(Column::try_new(
            data,
            varid.into(),
            vtype,
        )?);
        self.variable_names.push(varname);
        // todo: adjust other values and fix logic after restructuring the solution
        // internally.
        Ok(())
    }

    pub fn remove_samplecol(&mut self, var: VarKey) {
        match var {
            VarKey::Var(var) => self.remove_samplecol_for_var(var),
            VarKey::Name(varname) => self.remove_samplecol_for_varname(varname),
        }
    }

    pub fn remove_samplecol_for_var(&mut self, var: &VarRef) {
        let env = var.env.borrow();
        let variable = env.get_for_index(var.id);
        match variable {
            Ok(variable) => {
                let id: usize = var.id.into();
                self.remove_samplecol_for_varname(variable.name.clone())
            },
            Err(_) => (),
        }
    }

    pub fn remove_samplecol_for_varname(&mut self, varname: String) {
        let index = self.variable_names.iter().find_position(|&n| *n == varname);
        match index {
            Some((idx, _)) => {
                let _ = self.variable_names.remove(idx);
                let _ = self.samples.remove(idx);
            },
            None => (),
        };
    }
}
