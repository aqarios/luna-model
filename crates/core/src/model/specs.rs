//! Structural spec checks for models.

use enumset::EnumSet;
use lunamodel_types::{Comparator, Ctype, Specs};

use super::Model;

impl Model {
    /// Derives a coarse structural summary of the model.
    ///
    /// The returned [`Specs`] value is used by transformation and compatibility
    /// checks to reason about a model without re-inspecting every expression.
    pub fn specs(&self) -> Specs {
        let mut vtypes = EnumSet::new();
        for vtype in self.vtypes() {
            vtypes.insert(vtype);
        }
        let mut constraints = EnumSet::new();
        let mut max_constraint_degress: usize = 0;
        for (_, constr) in self.constraints.iter() {
            max_constraint_degress = max_constraint_degress.max(constr.lhs.degree());
            match constr.comparator {
                Comparator::Eq => constraints.insert(Ctype::Equality),
                Comparator::Le => constraints.insert(Ctype::LessEqual),
                Comparator::Ge => constraints.insert(Ctype::GreaterEqual),
            };
        }

        Specs::new(
            self.sense,
            vtypes,
            constraints,
            self.objective.degree(),
            max_constraint_degress,
            self.num_variables(),
        )
    }

    /// Returns whether the model satisfies the requested specification constraints.
    pub fn satisfies(&self, specs: &Specs) -> bool {
        self.specs().satisfies(specs)
    }
}
