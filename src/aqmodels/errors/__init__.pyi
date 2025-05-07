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

class TranslationError(Exception):
    def __str__(self) -> str: ...

class ModelNotQuadraticError(TranslationError):
    def __str__(self) -> str: ...

class ModelNotUnconstrainedError(TranslationError):
    def __str__(self) -> str: ...

class SolutionTranslationError(Exception):
    def __str__(self) -> str: ...

class SampleIncorrectLengthError(SolutionTranslationError):
    def __str__(self) -> str: ...

class SampleIncompatibleVtypeError(SolutionTranslationError):
    def __str__(self) -> str: ...

__all__ = [
    "DecodeError",
    "DifferentEnvsError",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "SampleIncompatibleVtypeError",
    "SampleIncorrectLengthError",
    "SolutionTranslationError",
    "TranslationError",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "errors",
]
