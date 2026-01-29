from collections.abc import Iterable

from luna_model._lm import quicksum as q
from luna_model.expression.expr import Expression
from luna_model.variable.var import Variable


def quicksum(iterable: Iterable, start: Expression | Variable | None = None) -> Expression:
    """Quicksum an iterable of Expression, Variable and floats."""
    return Expression._from_pyexpr(q(iterable, start))
