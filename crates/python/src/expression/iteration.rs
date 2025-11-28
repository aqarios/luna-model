use lunamodel_core::ArcEnv;
use lunamodel_types::{Bias, VarIdx};
use pyo3::prelude::*;

use crate::{PyExpression, PyVariable};

/// Convenience class to indicate the empty set of variables of an expression's
/// constant term when iterating over the expression's components.
///
/// Note that the bias corresponding to the constant part is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[pyclass]
pub struct PyConstant();

/// Convenience class to indicate the variable of an expression's linear term when
/// iterating over the expression's components.
///
/// Note that the bias corresponding to this variable is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[pyclass]
pub struct PyLinear(pub PyVariable);

/// Convenience class to indicate the variables of an expression's quadratic term when
/// iterating over the expression's components.
///
/// Note that the bias corresponding to these two variables is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[pyclass]
pub struct PyQuadratic(pub (PyVariable, PyVariable));

/// Convenience class to indicate the set of variables of an expression's higher-order
/// term when iterating over the expression's components.
///
/// Note that the bias corresponding to these variables is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[pyclass]
pub struct PyHigherOrder(pub Vec<PyVariable>);

/// Iterate over the single components of an expression.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[pyclass]
pub struct PyExpressionIterator {
    items: Vec<(Vec<VarIdx>, Bias)>,
    env: ArcEnv,
    current_idx: usize,
}

// impl PyExpressionIterator {
//     fn new(expr: &PyExpression) -> Self {
//         use super::ExprContent::*;
//         Self {
//             items: match &expr.expr {
//                 Expr(expr) => expr.items(),
//                 Model(p) => p.read_arc().objective.items(),
//             },
//             env: match &expr.expr {
//                 Expr(expr) => expr.env.clone(),
//                 Model(p) => p.read_arc().environment.clone(),
//             },
//             current_idx: 0,
//         }
//     }
// }
