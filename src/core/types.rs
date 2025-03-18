use std::{cell::RefCell, rc::Rc};

use super::{Constraint, Constraints, Environment, Expression};

pub type MutRcExpression<Index, Bias> = Rc<RefCell<Expression<Index, Bias>>>;
pub type MutRcEnvironment<Index> = Rc<RefCell<Environment<Index>>>;
pub type MutRcConstraint<Index, Bias> = Rc<RefCell<Constraint<Index, Bias>>>;
pub type MutRcConstraints<Index, Bias> = Rc<RefCell<Constraints<Index, Bias>>>;
