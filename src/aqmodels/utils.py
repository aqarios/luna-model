"""Utility module containing convenience functions."""

from collections.abc import Iterable

from ._core import Expression, Variable


class InvalidStartTypeError(TypeError):
    """To be raised when an invalid start type is specified in quicksum."""

    def __init__(self, actual: type) -> None:
        super().__init__(f"start must be of type `Expression`, is '{actual}'")


class StartCannotBeInferredError(TypeError):
    """To be raised when the start value in the quicksum cannot be inferred."""

    def __init__(self) -> None:
        super().__init__(
            "iterable must contain at least one Expression or Variable, "
            "or 'start' needs to be set."
        )


def quicksum(
    iterable: Iterable[Expression | Variable | int | float],
    /,
    start: Expression | None = None,
) -> Expression:
    """Sum over an Iterable of Expression, Variable, int or float values.

    Create an Expression based on an iterable of Expression, Variable, int or float
    elements. Note that either the `iterable` must contain at least one `Expression`
    or `Variable` or the start parameter is set.

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
    TypeError (InvalidStartTypeError | StartCannotBeInferredError)
        If the `iterable` does not contain any Expression or Variable.
        If the `start` is not of type Expression.
    """
    items = list(iterable)
    if start is None:
        for item in items:
            if isinstance(item, Expression | Variable):
                start = Expression(env=item._environment)  # noqa: SLF001
                break

    if start is None:
        raise StartCannotBeInferredError

    if not isinstance(start, Expression):
        raise InvalidStartTypeError(type(start))

    _start: Expression = start

    for item in items:
        _start += item

    return _start
