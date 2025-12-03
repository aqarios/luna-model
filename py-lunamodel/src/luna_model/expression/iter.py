from __future__ import annotations
from typing import Protocol

from luna_model.variable import Variable


class Constant(Protocol):
    """A constant expression.

    Convenience class to indicate the empty set of variables of an expression's
    constant term when iterating over the expression's components.

    Note that the bias corresponding to the constant part is not part of this class.

    Examples
    --------
    >>> from luna_model import Constant, Expression, HigherOrder, Linear, Quadratic
    >>> expr: Expression = ...
    >>> vars: Constant | Linear | Quadratic | HigherOrder
    >>> bias: float
    >>> for vars, bias in expr.items():
    >>> match vars:
    >>>     case Constant(): do_something_with_constant(bias)
    >>>     case Linear(x): do_something_with_linear_var(x, bias)
    >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
    >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
    """


class Linear(Protocol):
    """A linear expression.

    Convenience class to indicate the variable of an expression's linear term when
    iterating over the expression's components.

    Note that the bias corresponding to this variable is not part of this class.

    Examples
    --------
    >>> from luna_model import Constant, Expression, HigherOrder, Linear, Quadratic
    >>> expr: Expression = ...
    >>> vars: Constant | Linear | Quadratic | HigherOrder
    >>> bias: float
    >>> for vars, bias in expr.items():
    >>> match vars:
    >>>     case Constant(): do_something_with_constant(bias)
    >>>     case Linear(x): do_something_with_linear_var(x, bias)
    >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
    >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
    """

    __match_args__ = ("var",)

    @property
    def var(self) -> Variable: ...


class Quadratic(Protocol):
    """A quadratic expression.

    Convenience class to indicate the variables of an expression's quadratic term when
    iterating over the expression's components.

    Note that the bias corresponding to these two variables is not part of this class.

    Examples
    --------
    >>> from luna_model import Constant, Expression, HigherOrder, Linear, Quadratic
    >>> expr: Expression = ...
    >>> vars: Constant | Linear | Quadratic | HigherOrder
    >>> bias: float
    >>> for vars, bias in expr.items():
    >>> match vars:
    >>>     case Constant(): do_something_with_constant(bias)
    >>>     case Linear(x): do_something_with_linear_var(x, bias)
    >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
    >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
    """

    __match_args__ = ("var_a", "var_b")

    @property
    def var_a(self) -> Variable: ...
    @property
    def var_b(self) -> Variable: ...


class HigherOrder(Protocol):
    """A higher-order expression.

    Convenience class to indicate the set of variables of an expression's higher-order
    term when iterating over the expression's components.

    Note that the bias corresponding to these variables is not part of this class.

    Examples
    --------
    >>> from luna_model import Constant, Expression, HigherOrder, Linear, Quadratic
    >>> expr: Expression = ...
    >>> vars: Constant | Linear | Quadratic | HigherOrder
    >>> bias: float
    >>> for vars, bias in expr.items():
    >>> match vars:
    >>>     case Constant(): do_something_with_constant(bias)
    >>>     case Linear(x): do_something_with_linear_var(x, bias)
    >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
    >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
    """

    __match_args__ = ("vars",)

    @property
    def vars(self) -> list[Variable]: ...


class ExprIter(Protocol):
    """
    Iterate over the single components of an expression.

    Examples
    --------
    >>> from luna_model import Constant, Expression, HigherOrder, Linear, Quadratic
    >>> expr: Expression = ...
    >>> vars: Constant | Linear | Quadratic | HigherOrder
    >>> bias: float
    >>> for vars, bias in expr.items():
    >>> match vars:
    >>>     case Constant(): do_something_with_constant(bias)
    >>>     case Linear(x): do_something_with_linear_var(x, bias)
    >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
    >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
    """

    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]: ...
    def __iter__(self) -> ExprIter: ...
