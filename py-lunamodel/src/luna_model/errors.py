"""Exception types for LunaModel.

This module defines all exception types that can be raised by LunaModel
operations. All exceptions inherit from the base LunaModelError.

Exception hierarchy:
    - LunaModelError: Base exception for all LunaModel errors
    - ComputationError: Errors during mathematical computations
    - DecodeError: Errors during decoding/deserialization
    - DifferentEnvsError: Operations on objects from different environments
    - EvaluationError: Errors during expression evaluation
    - TranslationError: Errors during model translation
    - VariableCreationError: Errors during variable creation
    - And many more specific error types...
"""

from luna_model._lm import (
    PyComputationError as ComputationError,
)
from luna_model._lm import (
    PyDecodeError as DecodeError,
)
from luna_model._lm import (
    PyDifferentEnvsError as DifferentEnvsError,
)
from luna_model._lm import (
    PyDuplicateConstraintNameError as DuplicateConstraintNameError,
)
from luna_model._lm import (
    PyEvaluationError as EvaluationError,
)
from luna_model._lm import (
    PyIllegalConstraintNameError as IllegalConstraintNameError,
)
from luna_model._lm import (
    PyInternalPanicError as InternalPanicError,
)
from luna_model._lm import (
    PyLunaModelError as LunaModelError,
)
from luna_model._lm import (
    PyModelNotQuadraticError as ModelNotQuadraticError,
)
from luna_model._lm import (
    PyModelNotUnconstrainedError as ModelNotUnconstrainedError,
)
from luna_model._lm import (
    PyModelSenseNotMinimizeError as ModelSenseNotMinimizeError,
)
from luna_model._lm import (
    PyModelVtypeError as ModelVtypeError,
)
from luna_model._lm import (
    PyMultipleActiveEnvironmentsError as MultipleActiveEnvironmentsError,
)
from luna_model._lm import (
    PyNoActiveEnvironmentFoundError as NoActiveEnvironmentFoundError,
)
from luna_model._lm import (
    PyNoConstraintForKeyError as NoConstraintForKeyError,
)
from luna_model._lm import (
    PyRandomSamplingError as RandomSamplingError,
)
from luna_model._lm import (
    PySampleColCreationError as SampleColCreationError,
)
from luna_model._lm import (
    PySampleIncompatibleVtypeError as SampleIncompatibleVtypeError,
)
from luna_model._lm import (
    PySampleIncorrectLengthError as SampleIncorrectLengthError,
)
from luna_model._lm import (
    PySampleUnexpectedVariableError as SampleUnexpectedVariableError,
)
from luna_model._lm import (
    PySolutionTranslationError as SolutionTranslationError,
)
from luna_model._lm import (
    PyStartCannotBeInferredError as StartCannotBeInferredError,
)
from luna_model._lm import (
    PyTranslationError as TranslationError,
)
from luna_model._lm import (
    PyUnsupportedOperationError as UnsupportedOperationError,
)
from luna_model._lm import (
    PyVariableCreationError as VariableCreationError,
)
from luna_model._lm import (
    PyVariableExistsError as VariableExistsError,
)
from luna_model._lm import (
    PyVariableNamesError as VariableNamesError,
)
from luna_model._lm import (
    PyVariableNotExistingError as VariableNotExistingError,
)
from luna_model._lm import (
    PyVariableOutOfRangeError as VariableOutOfRangeError,
)
from luna_model._lm import (
    PyVariablesFromDifferentEnvsError as VariablesFromDifferentEnvsError,
)

__all__ = [
    "ComputationError",
    "ComputationError",
    "DecodeError",
    "DifferentEnvsError",
    "DuplicateConstraintNameError",
    "EvaluationError",
    "IllegalConstraintNameError",
    "InternalPanicError",
    "LunaModelError",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelSenseNotMinimizeError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "NoConstraintForKeyError",
    "RandomSamplingError",
    "SampleColCreationError",
    "SampleIncompatibleVtypeError",
    "SampleIncorrectLengthError",
    "SampleUnexpectedVariableError",
    "SolutionTranslationError",
    "StartCannotBeInferredError",
    "TranslationError",
    "UnsupportedOperationError",
    "VariableCreationError",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
]
