from luna_model._lm import quicksum as q
from luna_model.expression.expr import Expression


def quicksum(iterable, start: Expression | None = None) -> Expression:
    return Expression._from_pyexpr(q(iterable, start._expr if start else None))
