# This file is auto-generated.
# Do not edit manually.

from . import errors

class VariableOutOfRangeError(Exception):
    def __str__(self) -> str: ...

class VariableExistsError(Exception):
    def __str__(self) -> str: ...

class VariableNotExistingError(Exception):
    def __str__(self) -> str: ...

class VariablesFromDifferentEnvsError(Exception):
    def __str__(self) -> str: ...

class DifferentEnvsError(Exception):
    def __str__(self) -> str: ...

class NoActiveEnvironmentFoundError(Exception):
    def __str__(self) -> str: ...

class MultipleActiveEnvironmentsError(Exception):
    def __str__(self) -> str: ...

class DecodeError(Exception):
    def __str__(self) -> str: ...

class ModelVtypeError(Exception):
    def __str__(self) -> str: ...

class VariableNamesError(Exception):
    def __str__(self) -> str: ...

class SolutionCreationError(Exception):
    def __str__(self) -> str: ...

class IllegalConstraintNameError(Exception):
    def __str__(self) -> str: ...

class TranslationError(Exception):
    def __str__(self) -> str: ...

class ModelNotQuadraticError(TranslationError):
    def __str__(self) -> str: ...

class ModelNotUnconstrainedError(TranslationError):
    def __str__(self) -> str: ...

__all__ = [
    "DecodeError",
    "DifferentEnvsError",
    "IllegalConstraintNameError",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "SolutionCreationError",
    "TranslationError",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "errors",
]
