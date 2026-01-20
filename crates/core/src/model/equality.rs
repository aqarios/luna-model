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
        dbg!(name_eq);
        let env_eq = self.environment.equal_contents(&other.environment);
        dbg!(env_eq);
        let obj_eq = self.objective.equal_contents(&other.objective);
        dbg!(obj_eq);
        let const_eq = self.constraints.equal_contents(&other.constraints);
        dbg!(const_eq);
        let sense_eq = self.sense == other.sense;
        dbg!(sense_eq);
        name_eq && env_eq && obj_eq && const_eq && sense_eq
    }
}
