# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from luna_model._deprecated import deprecated
from luna_model._lm import (
    PyAnalysisPassError as AnalysisPassError,
)
from luna_model._lm import (
    PyCompressionError as CompressionError,
)
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
    PyIfElsePassError as IfElsePassError,
)
from luna_model._lm import (
    PyIllegalConstraintNameError as IllegalConstraintNameError,
)
from luna_model._lm import (
    PyInternalPanicError as InternalPanicError,
)
from luna_model._lm import (
    PyInvalidToleranceError as InvalidToleranceError,
)
from luna_model._lm import (
    PyLunaModelError as LunaModelError,
)
from luna_model._lm import (
    PyMetaAnalysisPassError as MetaAnalysisPassError,
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
    PyTransformationError as TransformationError,
)
from luna_model._lm import (
    PyTransformationPassError as TransformationPassError,
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
from luna_model.transformation.record import TransformationRecord

class TransformError(TransformationError):
    record: TransformationRecord | None
    """Transformation record recovered up to the point of failure, if available."""

class ModelInfeasibleError(TransformError): ...

@deprecated("CompilationError is deprecated. Use TransformError instead.")  # noqa: PYI053
class CompilationError(TransformError): ...

__all__ = [
    "AnalysisPassError",
    "CompilationError",
    "CompressionError",
    "ComputationError",
    "DecodeError",
    "DifferentEnvsError",
    "DuplicateConstraintNameError",
    "EvaluationError",
    "IfElsePassError",
    "IllegalConstraintNameError",
    "InternalPanicError",
    "InvalidToleranceError",
    "LunaModelError",
    "MetaAnalysisPassError",
    "ModelInfeasibleError",
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
    "TransformError",
    "TransformationError",
    "TransformationPassError",
    "TranslationError",
    "UnsupportedOperationError",
    "VariableCreationError",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
]
