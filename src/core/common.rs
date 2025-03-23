use std::{cell::RefCell, rc::Rc};

use super::{
    expression::{BiasConstraints, IndexConstraints},
    Constraint, Constraints, Environment, Expression, Model, VarRef,
};

pub type MutRcExpression<Index, Bias> = Rc<RefCell<Expression<Index, Bias>>>;
pub type MutRcEnvironment<Index> = Rc<RefCell<Environment<Index>>>;
pub type MutRcConstraint<Index, Bias> = Rc<RefCell<Constraint<Index, Bias>>>;
pub type MutRcConstraints<Index, Bias> = Rc<RefCell<Constraints<Index, Bias>>>;
pub type MutRcModel<Index, Bias> = Rc<RefCell<Model<Index, Bias>>>;
pub type RcVarRef<Index> = Rc<VarRef<Index>>;

impl<Index: IndexConstraints, Bias: BiasConstraints> Into<MutRcConstraints<Index, Bias>>
    for Constraints<Index, Bias>
{
    fn into(self) -> MutRcConstraints<Index, Bias> {
        RefCell::new(self).into()
    }
}

impl<Index: IndexConstraints, Bias: BiasConstraints> Into<MutRcConstraint<Index, Bias>>
    for Constraint<Index, Bias>
{
    fn into(self) -> MutRcConstraint<Index, Bias> {
        RefCell::new(self).into()
    }
}

impl<Index: IndexConstraints> Into<MutRcEnvironment<Index>> for Environment<Index> {
    fn into(self) -> MutRcEnvironment<Index> {
        RefCell::new(self).into()
    }
}

impl<Index: IndexConstraints, Bias: BiasConstraints> Into<MutRcExpression<Index, Bias>>
    for Expression<Index, Bias>
{
    fn into(self) -> MutRcExpression<Index, Bias> {
        Rc::new(RefCell::new(self))
    }
}
