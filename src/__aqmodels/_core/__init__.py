# from __future__ import annotations
# from enum import Enum
# import functools
# from typing import Any

from . import variable
from .variable import Variable

__all__ = ["Variable"]

# AQM_FUNCTIONS = set()


# add_docstring(
#     _ArrayFunctionDispatcher,
#     """
#     Class to wrap functions with checks for __array_function__ overrides.
#
#     All arguments are required, and can only be passed by position.
#
#     Parameters
#     ----------
#     dispatcher : function or None
#         The dispatcher function that returns a single sequence-like object
#         of all arguments relevant.  It must have the same signature (except
#         the default values) as the actual implementation.
#         If ``None``, this is a ``like=`` dispatcher and the
#         ``_ArrayFunctionDispatcher`` must be called with ``like`` as the
#         first (additional and positional) argument.
#     implementation : function
#         Function that implements the operation on NumPy arrays without
#         overrides.  Arguments passed calling the ``_ArrayFunctionDispatcher``
#         will be forwarded to this (and the ``dispatcher``) as if using
#         ``*args, **kwargs``.
#
#     Attributes
#     ----------
#     _implementation : function
#         The original implementation passed in.
#     """,
# )
#
#
# # exposed for testing purposes; used internally by _ArrayFunctionDispatcher
# add_docstring(
#     _get_implementing_args,
#     """
#     Collect arguments on which to call __array_function__.
#
#     Parameters
#     ----------
#     relevant_args : iterable of array-like
#         Iterable of possibly array-like arguments to check for
#         __array_function__ methods.
#
#     Returns
#     -------
#     Sequence of arguments with __array_function__ methods, in the order in
#     which they should be called.
#     """,
# )
#
#
# ArgSpec = collections.namedtuple("ArgSpec", "args varargs keywords defaults")


# def verify_matching_signatures(implementation, dispatcher):
#     """Verify that a dispatcher function has the right signature."""
#     implementation_spec = ArgSpec(*getargspec(implementation))
#     dispatcher_spec = ArgSpec(*getargspec(dispatcher))
#
#     if (
#         implementation_spec.args != dispatcher_spec.args
#         or implementation_spec.varargs != dispatcher_spec.varargs
#         or implementation_spec.keywords != dispatcher_spec.keywords
#         or (bool(implementation_spec.defaults) != bool(dispatcher_spec.defaults))
#         or (
#             implementation_spec.defaults is not None
#             and len(implementation_spec.defaults) != len(dispatcher_spec.defaults)
#         )
#     ):
#         raise RuntimeError(
#             "implementation and dispatcher for %s have "
#             "different function signatures" % implementation
#         )
#
#     if implementation_spec.defaults is not None:
#         if dispatcher_spec.defaults != (None,) * len(dispatcher_spec.defaults):
#             raise RuntimeError(
#                 "dispatcher functions can only use None for " "default argument values"
#             )


# def aqm_function_dispatch(
#     dispatcher=None, module=None, verify=True, docs_from_dispatcher=False
# ):
#     """Decorator for adding dispatch with the __array_function__ protocol.
#
#     See NEP-18 for example usage.
#
#     Parameters
#     ----------
#     dispatcher : callable or None
#         Function that when called like ``dispatcher(*args, **kwargs)`` with
#         arguments from the NumPy function call returns an iterable of
#         array-like arguments to check for ``__array_function__``.
#
#         If `None`, the first argument is used as the single `like=` argument
#         and not passed on.  A function implementing `like=` must call its
#         dispatcher with `like` as the first non-keyword argument.
#     module : str, optional
#         __module__ attribute to set on new function, e.g., ``module='numpy'``.
#         By default, module is copied from the decorated function.
#     verify : bool, optional
#         If True, verify the that the signature of the dispatcher and decorated
#         function signatures match exactly: all required and optional arguments
#         should appear in order with the same names, but the default values for
#         all optional arguments should be ``None``. Only disable verification
#         if the dispatcher's signature needs to deviate for some particular
#         reason, e.g., because the function has a signature like
#         ``func(*args, **kwargs)``.
#     docs_from_dispatcher : bool, optional
#         If True, copy docs from the dispatcher function onto the dispatched
#         function, rather than from the implementation. This is useful for
#         functions defined in C, which otherwise don't have docstrings.
#
#     Returns
#     -------
#     Function suitable for decorating the implementation of a NumPy function.
#
#     """
#
#     def decorator(implementation):
#         # if verify:
#         #     if dispatcher is not None:
#         #         verify_matching_signatures(implementation, dispatcher)
#         #     else:
#         #         # Using __code__ directly similar to verify_matching_signature
#         #         co = implementation.__code__
#         #         last_arg = co.co_argcount + co.co_kwonlyargcount - 1
#         #         last_arg = co.co_varnames[last_arg]
#         #         if last_arg != "like" or co.co_kwonlyargcount == 0:
#         #             raise RuntimeError(
#         #                 "__array_function__ expects `like=` to be the last "
#         #                 "argument and a keyword-only argument. "
#         #                 f"{implementation} does not seem to comply."
#         #             )
#
#         # if docs_from_dispatcher:
#         #     add_docstring(implementation, dispatcher.__doc__)
#
#         # public_api = _ArrayFunctionDispatcher(dispatcher, implementation)
#         # public_api = functools.wraps(implementation)(public_api)
#
#         # if module is not None:
#         #     public_api.__module__ = module
#
#         # AQM_FUNCTIONS.add(public_api)
#
#         # return public_api
#         return implementation
#
#     return decorator


# def aqm_function_from_dispatcher(
#     implementation, module=None, verify=True, docs_from_dispatcher=True
# ):
#     """Like array_function_dispatcher, but with function arguments flipped."""
#
#     def decorator(dispatcher):
#         return aqm_function_dispatch(
#             dispatcher, module, verify=verify, docs_from_dispatcher=docs_from_dispatcher
#         )(implementation)
#
#     return decorator


# aqm_function = functools.partial(
#     aqm_function_from_dispatcher,
#     module="aqmodels",
#     docs_from_dispatcher=True,
#     verify=False,
# )
#
#
# class Vtype(Enum):
#     """This is the vtype"""
#
#     Real = ...
#     """Real documentation"""
#     Integer = ...
#     """Integer documentation"""
#     Binary = ...
#     """Binary documentation"""
#     Spin = ...
#     """Spin documentation"""
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#
# class Comparator(Enum):
#     """This is the comparator"""
#
#     Eq = ...
#     """Eq documentation"""
#     Leq = ...
#     """Leq documentation"""
#     Geq = ...
#     """Geq documentation"""
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#
# class Bounds:
#     """ """
#
#     def __init__(self, lower, upper) -> None:
#         """
#         Init of bounds...
#         """
#         raise TypeError(
#             f"Invalid input types '{type(lower)} (lower)', '{type(upper)} (upper)' "
#         )
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#
# class Constraint:
#     def __init__(self, lhs, rhs, comparator) -> None:
#         """
#         Constraint init documentation
#         """
#         raise TypeError(
#             f"Invalid input types '{type(lhs)} (lhs)', '{type(rhs)} (rhs)', "
#             f"'{type(comparator)} (comparator)'"
#         )
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#
# class Constraints:
#     """
#     Documentation of Constraints
#     """
#
#     # @aqm_function(_core.Constraints.add_constraint)
#     def add_constraint(self, constraint):
#         """
#         Documentation of add_constraint
#         """
#         return constraint
#
#     def encode(self, compress, level) -> bytes:
#         """
#         Documentation of encode
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def serialize(self, compress, level) -> bytes:
#         """
#         Documentation of serialize
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def __iadd__(self, constraint):
#         """
#         Documentation of __iadd__
#         """
#         raise TypeError(f"Invalid input type '{type(constraint)}' for constraint")
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#     @staticmethod
#     def decode(data) -> Expression:
#         """
#         Documentation of add_constraint
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#     @staticmethod
#     def deserialize(data) -> Expression:
#         """
#         Documentation of deserialize
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#
# class Environment:
#     """
#     Documentation of Environment
#     """
#
#     def __init__(self) -> None:
#         """
#         Documentation of init
#         """
#         raise TypeError()
#
#     def encode(self, compress, level) -> bytes:
#         """
#         Documentation of encode
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def serialize(self, compress, level) -> bytes:
#         """
#         Documentation of serialize
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def __enter__(self) -> Any:
#         """
#         Documentation of __enter__
#         """
#         raise TypeError()
#
#     def __exit__(self, exc_type, exc_value, exc_traceback) -> None:
#         """
#         Documentation of __exit__
#         """
#         raise TypeError(
#             f"Invalid input types '{type(exc_type)} (exc_type)', "
#             f"'{type(exc_value)} (exc_value)', '{type(exc_traceback)} (exc_traceback)'"
#         )
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#     @staticmethod
#     def decode(data) -> Expression:
#         """
#         Documentation of add_constraint
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#     @staticmethod
#     def deserialize(data) -> Expression:
#         """
#         Documentation of deserialize
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#
# class Expression:
#     def __init__(self, env):
#         """
#         Documentation of __init__
#         """
#         raise TypeError(f"Invalid input type '{type(env)} (env)'")
#
#     def get_offset(self) -> float:
#         """
#         Documentation of get_offset
#         """
#         raise TypeError("Invalid input type")
#
#     def get_linear(self, variable) -> float:
#         """
#         Documentation of get_linear
#         """
#         raise TypeError(f"Invalid input type '{type(variable)} (variable)'")
#
#     def get_quadratic(self, u, v) -> float:
#         """
#         Documentation of get_quadratic
#         """
#         raise TypeError(f"Invalid input type '{type(u)} (u)', '{type(v)} (v)'")
#
#     def get_higher_order(self, variables) -> float:
#         """
#         Documentation of get_quadratic
#         """
#         raise TypeError(f"Invalid input type '{type(variables)} (variables)'")
#
#     def num_variables(self) -> int:
#         """
#         Documentation of get num_variables
#         """
#         raise TypeError()
#
#     def encode(self, compress, level) -> bytes:
#         """
#         Documentation of encode
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def serialize(self, compress, level) -> bytes:
#         """
#         Documentation of serialize
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def __add__(self, other) -> Expression:
#         """
#         Description of `__add__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __radd__(self, other) -> Expression:
#         """
#         Description of `__radd__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __iadd__(self, other) -> Expression:
#         """
#         Description of `__iadd__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __mul__(self, other) -> Expression:
#         """
#         Description of `__mul__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __rmul__(self, other) -> Expression:
#         """
#         Description of `__rmul__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __imul__(self, other) -> Expression:
#         """
#         Description of `__imul__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __eq__(self, value) -> Constraint:  # type: ignore
#         """
#         Description of `__eq__`
#         """
#         raise TypeError(f"Invalid input type '{type(value)}' for value")
#
#     def __le__(self, other) -> Constraint:
#         """
#         Description of `__le__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __ge__(self, other) -> Constraint:
#         """
#         Description of `__ge__`
#         """
#         raise TypeError(f"Invalid input type '{type(other)}' for other")
#
#     def __str__(self) -> str:
#         """
#         Description of `__str__`
#         """
#         raise TypeError()
#
#     def __repr__(self) -> str:
#         """
#         Description of `__repr__`
#         """
#         raise TypeError()
#
#     @staticmethod
#     def decode(data) -> Expression:
#         """
#         Documentation of add_constraint
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#     @staticmethod
#     def deserialize(data) -> Expression:
#         """
#         Documentation of deserialize
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#
# class Model:
#     """
#     Documentation of Model
#     """
#
#     def __init__(self, env, name) -> None:
#         """
#         Documentation of __init__
#         """
#         raise TypeError(
#             f"Invalid input type '{type(env)} (env)', '{type(name)} (name)'"
#         )
#
#     @property
#     def name(self) -> str:
#         """
#         Documentation of name
#         """
#         raise TypeError()
#
#     @property
#     def objective(self) -> Expression:
#         """
#         Documentation of objective getter
#         """
#         raise TypeError()
#
#     @objective.setter
#     def objective(self, value):
#         """
#         Documentation of objective setter
#         """
#         raise TypeError(f"Invalid input type '{type(value)}' for data")
#
#     @property
#     def constraints(self) -> Constraints:
#         """
#         Documentation of constraints getter
#         """
#         raise TypeError("Invalid input type")
#
#     @constraints.setter
#     def constraints(self, value: Constraints):
#         """
#         Documentation of constraints setter
#         """
#         raise TypeError(f"Invalid input type '{type(value)}' for data")
#
#     @property
#     def environment(self) -> Environment:
#         """
#         Documentation of environment getter
#         """
#         raise TypeError("Invalid input type")
#
#     def num_constraints(self) -> int:
#         """
#         Documentation of num_constraints getter func
#         """
#         raise TypeError("Invalid input type")
#
#     def encode(self, compress, level) -> bytes:
#         """
#         Documentation of encode
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     def serialize(self, compress, level) -> bytes:
#         """
#         Documentation of serialize
#         """
#         raise TypeError(
#             f"Invalid input type '{type(compress)} (compress)', '{type(level)} (level)'"
#         )
#
#     @staticmethod
#     def decode(data) -> Expression:
#         """
#         Documentation of add_constraint
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
#
#     @staticmethod
#     def deserialize(data) -> Expression:
#         """
#         Documentation of deserialize
#         """
#         raise TypeError(f"Invalid input type '{type(data)}' for data")
