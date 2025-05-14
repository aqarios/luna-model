# This file is auto-generated.
# Do not edit manually.

from ._errors import (
    DecodeError,
    TranslationError,
    ModelNotQuadraticError,
    DifferentEnvsError,
    VariableOutOfRangeError,
    NoActiveEnvironmentFoundError,
    VariablesFromDifferentEnvsError,
    VariableExistsError,
    VariableNotExistingError,
    IllegalConstraintNameError,
    ModelNotUnconstrainedError,
    SolutionCreationError,
    ModelVtypeError,
    MultipleActiveEnvironmentsError,
)
from .._core import errors as __errors

VariableOutOfRangeError = __errors.VariableOutOfRangeError  # noqa: F811
VariableExistsError = __errors.VariableExistsError  # noqa: F811
VariableNotExistingError = __errors.VariableNotExistingError  # noqa: F811
VariablesFromDifferentEnvsError = __errors.VariablesFromDifferentEnvsError  # noqa: F811
DifferentEnvsError = __errors.DifferentEnvsError  # noqa: F811
NoActiveEnvironmentFoundError = __errors.NoActiveEnvironmentFoundError  # noqa: F811
MultipleActiveEnvironmentsError = __errors.MultipleActiveEnvironmentsError  # noqa: F811
DecodeError = __errors.DecodeError  # noqa: F811
ModelVtypeError = __errors.ModelVtypeError  # noqa: F811
SolutionCreationError = __errors.SolutionCreationError  # noqa: F811
IllegalConstraintNameError = __errors.IllegalConstraintNameError  # noqa: F811
TranslationError = __errors.TranslationError  # noqa: F811
ModelNotQuadraticError = __errors.ModelNotQuadraticError  # noqa: F811
ModelNotUnconstrainedError = __errors.ModelNotUnconstrainedError  # noqa: F811

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
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
]
