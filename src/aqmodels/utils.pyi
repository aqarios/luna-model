from collections.abc import Iterable
from typing import overload

from aqmodels import Expression, Variable

@overload
def quicksum(iterable: Iterable[Expression], /) -> Expression: ...
@overload
def quicksum(iterable: Iterable[Variable], /) -> Expression: ...
@overload
def quicksum(iterable: Iterable[int], /) -> Expression: ...
@overload
def quicksum(iterable: Iterable[float], /) -> Expression: ...
@overload
def quicksum(
    iterable: Iterable[Expression], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Iterable[Variable], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Iterable[int], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Iterable[float], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Iterable[Expression | Variable | float | int],
    /,
    start: Expression | None = None,
) -> Expression: ...
