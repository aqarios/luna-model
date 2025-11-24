use lunamodel_types::Vtype;

use crate::variable::VarRef;

use super::Expression;

impl Expression {
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unimplemented!()
    }

    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        unimplemented!()
    }

    pub fn degree(&self) -> usize {
        unimplemented!()
    }
}
