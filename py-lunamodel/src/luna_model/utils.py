from luna_model._lm import quicksum as q
from luna_model.expression.expr import Expression
from luna_model.variable.var import Variable


def quicksum(iterable, start: Expression | Variable | None = None) -> Expression:
    return Expression._from_pyexpr(q(iterable, start))
