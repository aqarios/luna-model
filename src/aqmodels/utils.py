from typing import Iterable
from ._core import Expression, Variable


def quicksum(
    iterable: Iterable[Expression | Variable | int | float],
    /,
    start: Expression | None = None,
) -> Expression:
    """
    Create an Expression based on an iterable of Expression, Variable, int or float elements.
    Note that either the `iterable` must contain at least one `Expression` or `Variable` or
    the start parameter is set.

    Parameters
    ----------
    iterable : Iterable[Expression | Variable | int | float]
        The iterable of elements to sum up.
    start : Expression | None, optional
        The starting value for the summation.

    Returns
    -------
    Expression
        The expression created based on the sum of the iterable elements.

    Raises
    ------
    TypeError
        If the `iterable` does not contain any Expression or Variable.
    """
    items = list(iterable)
    if start is None:
        for item in items:
            if isinstance(item, Expression) or isinstance(item, Variable):
                start = Expression(env=item._environment)
                break

    if start is None:
        raise TypeError(
            "iterable must contain at least one Expression or Variable, or 'start' needs to be set."
        )

    assert start is not None
    assert isinstance(start, Expression)

    for item in items:
        start = start + item

    return start
