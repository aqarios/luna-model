from .._core import errors

VariableOutOfRangeError = errors.VariableOutOfRangeError
VariableExistsError = errors.VariableExistsError
VariableNotExistingError = errors.VariableNotExistingError
VariableCreationError = errors.VariableCreationError
VariablesFromDifferentEnvsError = errors.VariablesFromDifferentEnvsError
DifferentEnvsError = errors.DifferentEnvsError
NoActiveEnvironmentFoundError = errors.NoActiveEnvironmentFoundError
MultipleActiveEnvironmentsError = errors.MultipleActiveEnvironmentsError
DecodeError = errors.DecodeError
VariableNamesError = errors.VariableNamesError
IllegalConstraintNameError = errors.IllegalConstraintNameError
TranslationError = errors.TranslationError
ModelNotQuadraticError = errors.ModelNotQuadraticError
ModelNotUnconstrainedError = errors.ModelNotUnconstrainedError
ModelSenseNotMinimizeError = errors.ModelSenseNotMinimizeError
ModelVtypeError = errors.ModelVtypeError
SolutionTranslationError = errors.SolutionTranslationError
SampleIncorrectLengthError = errors.SampleIncorrectLengthError
SampleUnexpectedVariableError = errors.SampleUnexpectedVariableError
SampleIncompatibleVtypeError = errors.SampleIncompatibleVtypeError

__all__ = [
    "VariableOutOfRangeError",
    "VariableExistsError",
    "VariableNotExistingError",
    "VariableCreationError",
    "VariablesFromDifferentEnvsError",
    "DifferentEnvsError",
    "NoActiveEnvironmentFoundError",
    "MultipleActiveEnvironmentsError",
    "DecodeError",
    "VariableNamesError",
    "IllegalConstraintNameError",
    "TranslationError",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelSenseNotMinimizeError",
    "ModelVtypeError",
    "SolutionTranslationError",
    "SampleIncorrectLengthError",
    "SampleUnexpectedVariableError",
    "SampleIncompatibleVtypeError",
]
