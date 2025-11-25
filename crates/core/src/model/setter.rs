use crate::{ArcEnv, constraint::ConstraintCollection, expression::Expression};

use super::{Model, Sense};

impl Model {
    pub fn set_sense(&mut self, sense: Sense) -> &mut Self {
        self.sense = sense;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_objective(&mut self, obj: Expression) -> &mut Self {
        self.objective = obj;
        self
    }

    pub fn set_constraints(&mut self, coll: ConstraintCollection) -> &mut Self {
        self.constraints = coll;
        self
    }

    pub fn set_environment(&mut self, env: ArcEnv) -> &mut Self {
        self.environment = env;
        self
    }
}
