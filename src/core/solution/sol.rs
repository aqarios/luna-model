use crate::core::expression::{AssignmentConstraints, BiasConstraints};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ConstraintMetadata<Bias>
where
    Bias: BiasConstraints,
{
    lhs_eval: Bias,
    // ... extend with more metadata
}

// #[derive(Debug, Clone)]
// pub struct Res<Assignment, Bias>
// where
//     Assignment: AssignmentConstraints,
//     Bias: BiasConstraints,
// {
//     /// The solution bitstring
//     pub sample: Rc<Vec<Assignment>>,
//     /// The objective value computed from an AqModel. If not present, a raw value from the solver
//     /// may be used. None, if none of these are present.
//     obj_value: Option<Bias>,
//     /// Boolean flag for each single constraint whether it's satisfied
//     constraint_satisfaction: Rc<RefCell<Option<Vec<bool>>>>,
//     /// Whether all constraints are satisfied
//     feasible: Option<bool>,
//     /// Constraint metadata, e.g., to show why a constraint is not feasible.
//     constraint_metadata: Rc<Option<ConstraintMetadata<Bias>>>,
// }

#[derive(Debug, Clone)]
pub struct Res<'a, Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    /// The solution bitstring
    pub sample: &'a Vec<Assignment>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied
    constraint_satisfaction: &'a Option<Vec<bool>>,
    /// Whether all constraints are satisfied
    feasible: Option<bool>,
    /// Constraint metadata, e.g., to show why a constraint is not feasible.
    constraint_metadata: &'a Rc<Option<ConstraintMetadata<Bias>>>,
}

// impl<Assignment, Bias> Res<Assignment, Bias>
// where
//     Assignment: AssignmentConstraints,
//     Bias: BiasConstraints,
// {
//     pub fn new(
//         sample: Rc<Vec<Assignment>>,
//         obj_value: Option<Bias>,
//         constraint_satisfaction: Rc<RefCell<Option<Vec<bool>>>>,
//         feasible: Option<bool>,
//         constraint_metadata: Rc<Option<ConstraintMetadata<Bias>>>,
//     ) -> Self {
//         Self {
//             sample,
//             obj_value,
//             constraint_satisfaction,
//             feasible,
//             constraint_metadata,
//         }
//     }
// }

impl<Assignment, Bias> Res<Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    pub fn new(
        sample: &Rc<Vec<Assignment>>,
        obj_value: Option<Bias>,
        constraint_satisfaction: &Option<Vec<bool>>,
        feasible: Option<bool>,
        constraint_metadata: &Option<ConstraintMetadata<Bias>>,
    ) -> Self {
        Self {
            sample,
            obj_value,
            constraint_satisfaction,
            feasible,
            constraint_metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Runtime {
    total: f64,
    qpu: f64,
}

// #[derive(Debug, Clone, Default)]
// pub struct Solution<Assignment, Bias>
// where
//     Assignment: AssignmentConstraints,
//     Bias: BiasConstraints,
// {
//     pub samples: Vec<Rc<Vec<Assignment>>>,
//     pub num_occurrences: Vec<usize>,
//     pub obj_values: Option<Vec<Bias>>,
//     pub raw_energies: Option<Vec<Bias>>,
//     /// Boolean flag for each single constraint whether it's satisfied
//     constraints: Rc<RefCell<Vec<Vec<bool>>>>,
//     /// Boolean flag for each sample whether it's feasible, i.e., all constraints are satisfied
//     feasible: Option<Vec<bool>>,
//     constraint_metadata: Option<Vec<Rc<Option<ConstraintMetadata<Bias>>>>>,
//     best_sample_idx: Option<usize>,
// }

#[derive(Debug, Clone, Default)]
pub struct Solution<Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    pub samples: Vec<Vec<Assignment>>,
    pub num_occurrences: Vec<usize>,
    pub obj_values: Vec<Bias>,
    pub raw_energies: Vec<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied
    constraints: Vec<Vec<bool>>,
    /// Boolean flag for each sample whether it's feasible, i.e., all constraints are satisfied
    feasible: Vec<bool>,
    constraint_metadata: Vec<Option<ConstraintMetadata<Bias>>>,
    best_sample_idx: Option<usize>,
}

impl<Assignment, Bias> Solution<Assignment, Bias>
where
    Assignment: AssignmentConstraints,
    Bias: BiasConstraints,
{
    // pub fn new() -> Self {
    //     Self {
    //         samples: Vec::new(),
    //         obj_values: Vec::new(),
    //         raw_energies: Vec::new(),
    //         num_occurrences: Vec::new(),
    //         constraints: Vec::new(),
    //         feasible: Vec::new(),
    //         constraint_metadata: Vec::new(),
    //         best_sample_idx: None,
    //     }
    // }

    pub fn position(&self, sample: &Vec<Assignment>) -> Option<usize> {
        // TODO: find out whether this check is efficient enough or there's sth better
        self.samples.iter().position(|x| x == sample)
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// `num_occurrences` means how often this value occurs.
    pub fn extend(&mut self, sample: Vec<Assignment>, num_occurrences: usize) -> &mut Self {
        if let Some(idx) = self.position(&sample) {
            self.num_occurrences[idx] += num_occurrences;
        } else {
            self.samples.push(sample);
            self.num_occurrences.push(num_occurrences);
        }
        self
    }

    /// Extend a solution with a sample, without computing any objective values or similar.
    /// `num_occurrences` means how often this value occurs.
    /// In contrast to `extend`, this method does not check whether the sample is already part of
    /// the solution.
    pub fn extend_no_agg(&mut self, sample: Vec<Assignment>, num_occurrences: usize) -> &mut Self {
        self.samples.push(sample);
        self.num_occurrences.push(num_occurrences);
        self
    }

    // /// Iterate over the single results of the solution
    // pub fn iter(&self) -> impl Iterator<Item = Res<Assignment, Bias>> + use<'_, Assignment, Bias> {
    //     (0..self.samples.len()).map(|i| {
    //         // Rc<Option<Vec<bool>>>
    //
    //         let obj_value = if let Some(objs) = &self.obj_values {
    //             Some(objs[i])
    //         } else if let Some(re) = &self.raw_energies {
    //             Some(re[i])
    //         } else {
    //             None
    //         };
    //         let constraint_satisfaction = self.constraints.borrow().get(i);
    //
    //         Res::new(
    //             Rc::clone(&self.samples[i]),
    //             obj_value,
    //             Rc::clone(&self.constraints[i]),
    //             self.feasible[i],
    //             Rc::clone(&self.constraint_metadata[i]),
    //         )
    //     })
    // }

    /// Iterate over the single results of the solution
    pub fn iter(&self) -> impl Iterator<Item = Res<Assignment, Bias>> + use<'_, Assignment, Bias> {
        (0..self.samples.len()).map(|i| {
            // Rc<Option<Vec<bool>>>

            let obj_value = if let Some(&x) = self.obj_values.get(i) {
                Some(x)
            } else {
                self.raw_energies.get(i)
            };
            let constraints = match self.constraints.get(i) {
                None => &None,
                Some(&c) => &Some(c),
            };
            let feasible = match &self.feasible.get(i) {
                None => None,
                Some(&feas) => Some(feas),
            };
            let constraint_meta = match &self.constraint_metadata.get(i) {
                Some(&Some(x)) => &Some(x),
                _ => &None,
            };

            Res::new(
                &self.samples[i],
                obj_value,
                constraints,
                feasible,
                constraint_meta,
            )
        })
    }

    // pub fn samples_vec(&self) -> Vec<&Vec<Assignment>> {
    //     self.samples.iter().map(|sample| sample.as_ref()).collect()
    // }

    /// Get a vec of all results
    pub fn as_results_vec(&self) -> Vec<Res<Assignment, Bias>> {
        self.iter().collect()
    }
}
