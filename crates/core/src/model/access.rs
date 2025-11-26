use lunamodel_types::Vtype;
use lunamodel_utils::{unique, unique_by};

use super::Model;

impl Model {
    /// Access the **unique** [Vtype]s in the [Model]'s objective ([Expression](crate::expression::Expression))
    /// and the constraints ([ConstraintCollection](crate::constraint::ConstraintCollection)).
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.objective.vtypes().chain(self.constraints.vtypes()))
    }

    /// Access the total number of variables in the [Model].
    /// This value might be different to the number of variables registered in the
    /// [Environment](crate::environment::ArcEnv) as only the variables conributing to the
    /// objective or in the constraints is respected.
    pub fn num_variables(&self) -> usize {
        unique_by(self.objective.vars().chain(self.constraints.vars()), |e| {
            e.id
        })
        .count()
    }
}
