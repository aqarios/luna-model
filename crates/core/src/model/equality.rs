use super::Model;
use crate::traits::ContentEquality;

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.environment.id() == other.environment.id()
            && self.objective == other.objective
            && self.constraints == other.constraints
    }
}

impl ContentEquality for Model {
    fn equal_contents(&self, other: &Self) -> bool {
        let name_eq = self.name == other.name;
        let env_eq = self.environment.equal_contents(&other.environment);
        let obj_eq = self.objective.equal_contents(&other.objective);
        let const_eq = self.constraints.equal_contents(&other.constraints);
        let sense_eq = self.sense == other.sense;
        name_eq && env_eq && obj_eq && const_eq && sense_eq
    }
}
