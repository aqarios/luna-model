# This file is auto-generated.
# Do not edit manually.

from . import errors

class VariableOutOfRangeError(Exception):
    def __str__(self) -> str: ...

class VariableExistsError(Exception):
    def __str__(self) -> str: ...

class VariablesFromDifferentEnvsError(Exception):
    def __str__(self) -> str: ...


__all__ = [
    "VariableExistsError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "errors",
]