from collections.abc import Callable
from functools import wraps as _wraps
from typing import Any, Concatenate, ParamSpec, TypeVar, cast

P = ParamSpec("P")
R = TypeVar("R")
S = TypeVar("S")  # self type


def wraps(
    map_in: Callable[P, tuple[tuple[Any, ...], dict[str, Any]]] | None = None,
    map_out: Callable[[Any], R] | None = None,
) -> Callable[[Callable[Concatenate[S, P], R]], Callable[Concatenate[S, P], R]]:
    """Decorate a method to delegate its implementation to ``super()``.

    The decorated method keeps its own signature and return type, but its body is
    replaced by a call to the method with the same name on ``super(type(self), self)``.

    Optional hooks let you adapt arguments before the super call and adapt the
    result after the super call.

    Parameters
    ----------
    map_in : Callable[P, tuple[tuple[Any, ...], dict[str, Any]]] | None, optional
        Optional argument mapper. Receives the decorated method's ``*args`` and
        ``**kwargs`` (excluding ``self``) and must return ``(args, kwargs)`` to be
        passed to the super method. If omitted, arguments are forwarded unchanged.
    map_out : Callable[[Any], R] | None, optional
        Optional result mapper. Receives the raw result of the super call and
        returns the final value. If omitted, the super result is returned unchanged.

    Returns
    -------
    Callable[[Callable[Concatenate[S, P], R]], Callable[Concatenate[S, P], R]]
        A decorator that preserves the method's type signature while delegating
        execution to ``super()``.

    Notes
    -----
    This decorator assumes a method with the same name exists on the super object.
    If not, ``AttributeError`` is raised at runtime.
    """

    def deco(func: Callable[Concatenate[S, P], R]) -> Callable[Concatenate[S, P], R]:
        @_wraps(func)
        def wrapper(self: S, *args: P.args, **kwargs: P.kwargs) -> R:
            sup = getattr(super(type(self), self), func.__name__)
            call_args, call_kwargs = (args, kwargs) if map_in is None else map_in(*args, **kwargs)
            result = sup(*call_args, **call_kwargs)
            return cast("R", result if map_out is None else map_out(result))

        return wrapper

    return deco
