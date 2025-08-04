from collections.abc import Generator, Iterable
from typing import overload

from . import Expression, Variable

@overload
def quicksum(iterable: Generator[Expression], /) -> Expression: ...
@overload
def quicksum(iterable: Generator[Variable], /) -> Expression: ...
@overload
def quicksum(iterable: Generator[int], /) -> Expression: ...
@overload
def quicksum(iterable: Generator[float], /) -> Expression: ...
@overload
def quicksum(
    iterable: Generator[Expression], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Generator[Variable], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Generator[int], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Generator[float], /, start: Expression | None = None
) -> Expression: ...
@overload
def quicksum(
    iterable: Generator[Expression | Variable | float | int],
    /,
    start: Expression | None = None,
) -> Expression: ...
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

__all__ = ["quicksum"]
