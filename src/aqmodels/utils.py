from typing import Iterable
from ._core import Expression, Variable

a = sum([1, 2, 3])


def quicksum(
    iterable: Iterable[Expression | Variable | float | int],
    /,
    start: Expression = Expression(),
) -> Expression:
    for item in iterable:
        start += item

    return start
