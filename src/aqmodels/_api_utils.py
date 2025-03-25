import functools
from types import FunctionType
from typing import TypeVar

T = TypeVar("T")


def attach_doc(from_obj: object, to_obj: T) -> T:
    """Attach docstring and module from one object to another."""
    to_obj.__doc__ = getattr(from_obj, "__doc__", None)
    to_obj.__module__ = getattr(from_obj, "__module__", to_obj.__module__)
    return to_obj


def bind(real: T, doc_source: object) -> T:
    """Alias `real` and attach docstring from `doc_source`."""
    return attach_doc(doc_source, real)


def doc_forward(func: FunctionType):
    """
    Decorator that forwards a method call to its super() implementation,
    but allows attaching a docstring and signature.
    """

    @functools.wraps(func)
    def wrapper(self, *args, **kwargs):
        return getattr(super(self.__class__, self), func.__name__)(*args, **kwargs)

    return wrapper
