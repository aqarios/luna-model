use crate::core::expression::BiasConstraints;
use crate::core::solution::AssignmentBaseTypes;
use crate::core::writer::line_length_restrictor::LineLengthRestrictor;
use crate::core::{RcSolution, Sample, Samples};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub struct SolutionWriter<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    writer: LineLengthRestrictor,
    phantom_bias: PhantomData<Bias>,
    phantom_assignment_types: PhantomData<AssignmentTypes>,
}

impl<Bias, AssignmentTypes> SolutionWriter<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new() -> Self {
        Self {
            writer: LineLengthRestrictor::new(0),
            phantom_bias: PhantomData,
            phantom_assignment_types: PhantomData,
        }
    }

    pub fn write_solution(&mut self, sol: RcSolution<Bias, AssignmentTypes>) -> &mut Self {
        self.write_samples(sol.samples(), &sol.counts)
    }

    pub fn write_samples(
        &mut self,
        samples: Samples<Bias, AssignmentTypes>,
        counts: &Vec<usize>,
    ) -> &mut Self {
        self.writer.write("{").increase_indent().new_line();
        for (idx, (sample, n_occ)) in samples.iter().zip(counts).enumerate() {
            if idx > 0 {
                self.writer.new_line();
            }
            self.writer.increase_indent();
            self.write_sample(sample);
            self.writer.write(":").space().write(&format!("{n_occ},"));
            self.writer.decrease_indent();
        }
        self.writer.decrease_indent().with_new_line("}");
        self
    }

    // Might be good once solutions have an env attached
    // pub fn write_samples_with_env(
    //     &mut self,
    //     samples: Samples<Bias, AssignmentTypes>,
    //     counts: &Vec<usize>,
    //     env: MutRcEnvironment<ConcreteIndex>,
    // ) -> &mut Self {
    //     self.writer.write("{").increase_indent().new_line();
    //     for (idx,(sample, n_occ)) in samples.iter().zip(counts).enumerate() {
    //         if idx > 0 {
    //             self.writer.new_line();
    //         }
    //         self.writer.increase_indent();
    //         self.write_sample_with_env(sample, Rc::clone(&env));
    //         self.writer
    //             .write(":")
    //             .space()
    //             .write(&format!("{n_occ},"));
    //         self.writer.decrease_indent();
    //     }
    //     self.writer.decrease_indent().with_new_line("}");
    //     self
    // }

    pub fn write_sample(&mut self, sample: Sample<Bias, AssignmentTypes>) -> &mut Self {
        self.writer.write("[");
        for (idx, assignment) in sample.iter().enumerate() {
            if idx > 0 {
                self.writer.write(",").space();
            }
            self.writer.write(&assignment.to_string());
        }
        self.writer.write("]");
        self
    }

    // Might be a good choice once solutions have an env attached
    // pub fn write_sample_with_env(
    //     &mut self,
    //     sample: Sample<Bias, AssignmentTypes>,
    //     env: MutRcEnvironment<ConcreteIndex>,
    // ) -> &mut Self {
    //     self.writer.write("{");
    //     for (idx, assignment) in sample.iter().enumerate() {
    //         if idx > 0 {
    //             self.writer.write(",").space();
    //         }
    //         self.writer
    //             .write(&env.borrow().variables[idx].name)
    //             .write(":")
    //             .space()
    //             .write(&format!("{assignment}"));
    //     }
    //     self.writer.write("}");
    //     self
    // }
}

impl<Bias, AssignmentTypes> Display for SolutionWriter<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.writer.to_string())
    }
}
