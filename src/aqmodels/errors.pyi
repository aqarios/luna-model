class VariableOutOfRangeError(Exception):
    """Referenced variable is out of bounds for the environment.

    Raised when a variable referenced in an expression is out of bounds for the
    environment.

    This error typically occurs when querying coefficients (linear, quadratic,
    or higher-order) from an `Expression` using a `Variable` whose index does not
    exist in the environment's internal registry.

    This may happen if:
        - A variable is used from a different environment
        - A variable was removed or never registered properly
        - A raw index or tuple refers to a non-existent variable ID
    """

    def __str__(self, /) -> str: ...

class VariableExistsError(Exception):
    """
    Raised when trying to create a variable with a name that already exists.

    Variable names must be unique within an `Environment`. Attempting to redefine
    a variable with the same name will raise this exception.
    """

    def __str__(self, /) -> str: ...

class VariableNotExistingError(Exception):
    """Raised when trying to get a variable with a name that does not exist."""

    def __str__(self, /) -> str: ...

class VariableCreationError(Exception):
    """
    Raised when an error occurs during the creation of a variable.

    For example, binary and spin variables cannot be created with bounds.
    """

    def __str__(self, /) -> str: ...

class VariablesFromDifferentEnvsError(Exception):
    """
    Raised when multiple variables from different environments are used together.

    All variables in an expression or constraint must belong to the same
    `Environment`. Mixing across environments is disallowed to ensure consistency.
    """

    def __str__(self, /) -> str: ...

class DifferentEnvsError(Exception):
    """
    Raised when two incompatible environments are passed to a model or operation.

    Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
    or in structural operations that require consistency across multiple environments.
    """

    def __str__(self, /) -> str: ...

class NoActiveEnvironmentFoundError(Exception):
    """Variable or Expression created without an environment (or context).

    Raised when a variable or expression is created without an active environment
    context.

    This typically happens when not using `with Environment(): ...` and no environment
    was explicitly provided.
    """

    def __str__(self, /) -> str: ...

class MultipleActiveEnvironmentsError(Exception):
    """
    Raised when multiple environments are active simultaneously.

    This is a logic error, since `aqmodels` only supports one active environment
    at a time. This is enforced to maintain clarity and safety.
    """

    def __str__(self, /) -> str: ...

class DecodeError(Exception):
    """
    Raised when decoding or deserialization of binary data fails.

    This can occur if the encoded data is corrupted, incompatible, or not generated
    by `aqmodels.encode()`.
    """

    def __str__(self, /) -> str: ...

class VariableNamesError(Exception):
    """The provided variable names are invalid.

    Raised when the QuboTranslator tries to create a model from a QUBO matrix, but
    the provided variable names are invalid.

    If variable names are provided to the QuboTranslator, they have to be unique, and
    the number of names has to match the number of variables in the QUBO matrix.
    """

    def __str__(self, /) -> str: ...

class IllegalConstraintNameError(Exception):
    """Raised when a constraint is tried to be created with an illegal name."""

    def __str__(self, /) -> str: ...

class TranslationError(Exception):
    """Raised when an error occurred during translation."""

    def __str__(self, /) -> str: ...

class ModelNotQuadraticError(TranslationError):
    """
    Raised when a model is expected to be quadratic but contains higher-order terms.

    Some solvers or transformations require the model to have at most quadratic
    expressions. This error signals that unsupported terms were detected.
    """

    def __str__(self, /) -> str: ...

class ModelNotUnconstrainedError(TranslationError):
    """Operation requires an unconstrained model.

    Raised when an operation requires an unconstrained model, but constraints are
    present.

    Some solution methods may only work on unconstrained models, such as when
    transforming a symbolic model to a low-level format.
    """

    def __str__(self, /) -> str: ...

class ModelSenseNotMinimizeError(TranslationError):
    """Operation requires a model for minimization.

    Raised when an operation requires a model with minimization sense, but has
    maximization sense.

    Some model formats only work with minimization sense. In this case, consider
    setting the sense to `minimize` before the transformation, and multiplying the
    objective by `-1` if necessary.
    """

    def __str__(self, /) -> str: ...

class ModelVtypeError(TranslationError):
    """Operation has constraints on model's variable types.

    Raised when an operation has certain constraints on a model's variable types that
    are violated.

    Some solution methods may only work on models where all variables have the same
    type, or where only certain variable types are permitted.
    """

    def __str__(self, /) -> str: ...

class SolutionTranslationError(Exception):
    """
    Raised when something goes wrong during the translation of a solution.

    This may happen during the translation to an AqSolution from a different solution
    format, e.g., when the samples have different lengths or the variable types are not
    consistent with the model the solution is created for.
    """

    def __str__(self, /) -> str: ...

class SampleIncorrectLengthError(SolutionTranslationError):
    """
    Raised when a sample length is different from the number of model variables.

    When an external solution format is translated to an AqSolution, the number of
    variable assignments in the solution's sample has to exactly match the number of
    variables in the model environment that is passed to the translator.
    """

    def __str__(self, /) -> str: ...

class SampleUnexpectedVariableError(SolutionTranslationError):
    """Variable not present in environment.

    Raised when a sample contains a variable with a name that is not present in the
    environment.

    When a sample is translated to an AqResult, the currently active environment has to
    contain the same variables as the sample.
    """

    def __str__(self, /) -> str: ...

class SampleIncompatibleVtypeError(SolutionTranslationError):
    """A sample's assignments have incompatible vtypes.

    Raised when a sample's assignments have variable types incompatible with the
    model's variable types.

    When an external solution format is translated to an AqSolution, the variable
    assignments are tried to be converted into the model's corresponding variable type.
    This may fail when the assignment types are incompatible.

    Note that conversions with precision loss or truncation are admitted, but
    conversions of variables outside the permitted range will fail.
    """

    def __str__(self, /) -> str: ...

class ComputationError(Exception):
    """Raised when an error occured in an internal computation."""

    def __str__(self, /) -> str: ...

class EvaluationError(Exception):
    """Raised when an error occured during evaluation of a model."""

    def __str__(self, /) -> str: ...

class DuplicateConstraintNameError(Exception):
    """Raised when a duplicate constraint name is used."""

    def __str__(self, /) -> str: ...

class CompilationError(RuntimeError):
    """Raised when an error occured during compilation of a model in the PassManager."""

    def __str__(self, /) -> str: ...

class StartCannotBeInferredError(TypeError):
    """To be raised when the start value in the quicksum cannot be inferred."""

    def __str__(self, /) -> str: ...

class NoConstraintForKeyError(IndexError):
    """Raised getting a constraint from the constraints that does not exist."""

    def __str__(self, /) -> str: ...

class InternalPanicError(RuntimeError):
    """Raised when an internal and unrecoverable error occurred."""

    def __str__(self, /) -> str: ...

__all__ = [
    "ComputationError",
    "ComputationError",
    "DecodeError",
    "DifferentEnvsError",
    "DuplicateConstraintNameError",
    "EvaluationError",
    "IllegalConstraintNameError",
    "InternalPanicError",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelSenseNotMinimizeError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "NoConstraintForKeyError",
    "SampleIncompatibleVtypeError",
    "SampleIncorrectLengthError",
    "SampleUnexpectedVariableError",
    "SolutionTranslationError",
    "TranslationError",
    "VariableCreationError",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
]
