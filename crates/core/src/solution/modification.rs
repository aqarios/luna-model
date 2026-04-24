//! Mutation operations for column-oriented solutions.

use indexmap::IndexMap;
use itertools::Itertools;
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, Vtype};
use std::collections::HashMap;

use crate::solution::{Column, col::Assignment};

use super::Solution;

impl Solution {
    /// Appends a sample row to the solution.
    ///
    /// The incoming row is keyed by variable name to match the solution's
    /// index-independent representation. Existing columns are appended in place;
    /// missing columns are created on demand using the assignment type of the
    /// provided value.
    pub fn push(
        &mut self,
        sample: IndexMap<String, Assignment>,
        counts: usize,
        energy: Option<Bias>,
    ) -> LunaModelResult<()> {
        // First, let's check that the keys of the sample and the samples are equal.
        self.counts.push(counts);
        energy.and_then(|e| {
            self.raw_energies.as_mut().map(|r| {
                let _: () = r.push(e);
            })
        });
        for (key, a) in sample {
            if let Some(col) = self.samples.get_mut(&key) {
                col.push(a)?;
            } else {
                self.samples.insert(key, Column::with(a));
            }
        }

        Ok(())
    }

    /// Adds an empty binary column for a variable name.
    pub fn add_empty_binary(&mut self, var: String) {
        self.samples.insert(var, Column::empty_binary());
    }

    /// Adds an empty spin column for a variable name.
    pub fn add_empty_spin(&mut self, var: String) {
        self.samples.insert(var, Column::empty_spin());
    }

    /// Adds an empty integer column for a variable name.
    pub fn add_empty_integer(&mut self, var: String) {
        self.samples.insert(var, Column::empty_integer());
    }

    /// Adds an empty real column for a variable name.
    pub fn add_empty_real(&mut self, var: String) {
        self.samples.insert(var, Column::empty_real());
    }

    /// Replaces or inserts a binary column from raw numeric data.
    pub fn add_binary(
        &mut self,
        var: String,
        data: Vec<f64>,
        tol: Option<f64>,
    ) -> LunaModelResult<()> {
        let mut col = Column::empty_binary();
        data.iter().try_for_each(|e| col.try_push(*e, tol))?;
        self.samples.insert(var, col);
        Ok(())
    }

    /// Replaces or inserts a spin column from raw numeric data.
    pub fn add_spin(
        &mut self,
        var: String,
        data: Vec<f64>,
        tol: Option<f64>,
    ) -> LunaModelResult<()> {
        let mut col = Column::empty_spin();
        data.iter().try_for_each(|e| col.try_push(*e, tol))?;
        self.samples.insert(var, col);
        Ok(())
    }

    /// Replaces or inserts an integer column from raw numeric data.
    pub fn add_integer(
        &mut self,
        var: String,
        data: Vec<f64>,
        tol: Option<f64>,
    ) -> LunaModelResult<()> {
        let mut col = Column::empty_integer();
        data.iter().try_for_each(|e| col.try_push(*e, tol))?;
        self.samples.insert(var, col);
        Ok(())
    }

    /// Replaces or inserts a real-valued column.
    pub fn add_real(&mut self, var: String, data: Vec<f64>) {
        self.samples.insert(var, Column::real(data));
    }

    /// Dispatches to the appropriate typed column insertion helper.
    pub fn add_col(
        &mut self,
        vtype: Vtype,
        var: String,
        data: Vec<f64>,
        tol: Option<f64>,
    ) -> LunaModelResult<()> {
        match vtype {
            Vtype::Binary => self.add_binary(var, data, tol),
            Vtype::Spin => self.add_spin(var, data, tol),
            Vtype::Integer => self.add_integer(var, data, tol),
            Vtype::Real => {
                self.add_real(var, data);
                Ok(())
            }
            Vtype::InvertedBinary => Ok(()),
        }
    }

    /// Removes a single variable column by name.
    pub fn remove_col(&mut self, var: &str) -> Option<Column> {
        self.samples.shift_remove(var)
    }

    /// Removes many variable columns by name and returns the removed columns.
    pub fn remove_cols(&mut self, vars: &[String]) -> Vec<Option<Column>> {
        vars.iter()
            .map(|var| self.samples.shift_remove(var))
            .collect()
    }

    /// Lazily removes many variable columns by name.
    pub fn iter_remove_cols(&mut self, vars: &[String]) -> impl Iterator<Item = Option<Column>> {
        vars.iter().map(|var| self.samples.shift_remove(var))
    }

    /// Combines duplicate sample rows into a single row and sums their counts.
    ///
    /// Duplicate detection is currently based on the stringified sample values in
    /// column order. That keeps the implementation simple, but it also means the
    /// exact behavior for floating-point heavy solutions depends on how those
    /// values stringify.
    pub fn aggregate(&mut self) -> LunaModelResult<()> {
        let mut dups: HashMap<String, usize> = HashMap::new();
        let mut to_rm: IndexMap<usize, usize> = IndexMap::new();
        let mut indices: Vec<usize> = Vec::new();

        for sample in self.samples() {
            // TODO(team): round f64 (v) up to predefined decimal place followed by stringify.
            // Added this again, since Rust does not have a builtin to round this way. Will have to
            // do some more research.
            let samplekey = sample.iter().map(|v| v.to_string()).join(",");
            if let Some(&first) = dups.get(&samplekey) {
                to_rm.insert(sample.idx, first);
                indices.push(sample.idx);
            } else {
                dups.insert(samplekey, sample.idx);
            }
        }

        indices.sort();
        for &rmi in indices.iter().rev() {
            let base = *to_rm.get(&rmi).unwrap();
            self.counts[base] += self.counts[rmi];
            self.counts.remove(rmi);
            for (_, c) in self.samples.iter_mut() {
                c.remove(rmi);
            }
            for (_, c) in self.constraints.iter_mut() {
                c.remove(rmi);
            }
            for (_, c) in self.variable_bounds.iter_mut() {
                c.remove(rmi);
            }
            if let Some(v) = self.raw_energies.as_mut() {
                v.remove(rmi);
            }
            if let Some(v) = self.obj_values.as_mut() {
                v.remove(rmi);
            }
            if let Some(v) = self.feasible.as_mut() {
                v.remove(rmi);
            }
        }

        Ok(())
    }
}
