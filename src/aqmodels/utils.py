from typing import Iterable
from ._core import Expression, Variable


def quicksum(
    iterable: Iterable[Expression | Variable | int | float],
    /,
    start: Expression | None = None,
) -> Expression:
    items = list(iterable)
    if start is None:
        for item in items:
            if isinstance(item, Expression) or isinstance(item, Variable):
                start = Expression(env=item._environment)
                break

    if start is None:
        raise TypeError("iterable must contain at least one Expression or Variable, or 'start' needs to be set.")
    
    assert start is not None
    assert isinstance(start, Expression)

    for item in items:
        start = start + item

    return start
