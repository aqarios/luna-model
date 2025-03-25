from __future__ import annotations
import functools
from .._lib import Variable as V


def aqm_function_from_dispatcher(
    dispatcher=None, module=None, verify=True, docs_from_dispatcher=False
):
    def decorator(implementation):
        return implementation

    return decorator


aqm_function = functools.partial(
    aqm_function_from_dispatcher,
    module="aqmodels",
    docs_from_dispatcher=True,
    verify=False,
)


class Expression: ...


class Variable:
    """
    This is a variable
    """

    def __init__(self, name, env, vtype, bounds) -> None:
        """
        Create a new variable
        """
        raise TypeError(
            "Invalid input types "
            f"'{type(name)} (name)', '{type(env)} (env)' "
            f"'{type(vtype)} (vtype)', '{type(bounds)} (bounds)'"
        )

    def __new__(cls, name, env, vtype, bounds) -> Variable:
        """
        Create a new variable
        """
        raise TypeError(
            "Invalid input types "
            f"'{type(name)} (name)', '{type(env)} (env)' "
            f"'{type(vtype)} (vtype)', '{type(bounds)} (bounds)'"
        )

    @aqm_function(V.__add__)
    def __add__(self, other) -> Expression:
        """
        Description of __add__
        """
        return other

    def __radd__(self, other) -> Expression:
        """
        Description of `__radd__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __sub__(self, other) -> Expression:
        """
        Description of __sub__
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __rsub__(self, other) -> Expression:
        """
        Description of `__rsub__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __mul__(self, other) -> Expression:
        """
        Description of `__mul__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __rmul__(self, other) -> Expression:
        """
        Description of `__rmul__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __str__(self) -> str:
        """
        Description of `__str__`
        """
        raise TypeError()

    def __repr__(self) -> str:
        """
        Description of `__repr__`
        """
        raise TypeError()


__all__ = ["Variable"]
