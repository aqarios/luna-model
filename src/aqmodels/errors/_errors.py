from aqmodels._api_utils import export


@export("top", "errors")
class VariableOutOfRangeError(Exception):
    """
    Raised when a variable referenced in an expression is out of bounds for the environment.

    This error typically occurs when querying coefficients (linear, quadratic,
    or higher-order) from an `Expression` using a `Variable` whose index does not
    exist in the environment's internal registry.

    This may happen if:
    - A variable is used from a different environment
    - A variable was removed or never registered properly
    - A raw index or tuple refers to a non-existent variable ID
    """


@export("top", "errors")
class VariableExistsError(Exception):
    """
    Raised when trying to create a variable with a name that already exists.

    Variable names must be unique within an `Environment`. Attempting to redefine
    a variable with the same name will raise this exception.
    """

@export("top", "errors")
class VariableNotExistingError(Exception):
    """
    Raised when trying to get a variable with a name that does not exist.
    """


@export("top", "errors")
class VariablesFromDifferentEnvsError(Exception):
    """
    Raised when multiple variables from different environments are used together.

    All variables in an expression or constraint must belong to the same
    `Environment`. Mixing across environments is disallowed to ensure consistency.
    """


@export("top", "errors")
class DifferentEnvsError(Exception):
    """
    Raised when two incompatible environments are passed to a model or operation.

    Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
    or in structural operations that require consistency across multiple environments.
    """


@export("top", "errors")
class NoActiveEnvironmentFoundError(Exception):
    """
    Raised when a variable or expression is created without an active environment context.

    This typically happens when not using `with Environment(): ...` and no environment
    was explicitly provided.
    """


@export("top", "errors")
class MultipleActiveEnvironmentsError(Exception):
    """
    Raised when multiple environments are active simultaneously.

    This is a logic error, since `aqmodels` only supports one active environment
    at a time. This is enforced to maintain clarity and safety.
    """


@export("top", "errors")
class DecodeError(Exception):
    """
    Raised when decoding or deserialization of binary data fails.

    This can occur if the encoded data is corrupted, incompatible, or not generated
    by `aqmodels.encode()`.
    """


@export("top", "errors")
class ModelNotQuadraticError(Exception):
    """
    Raised when a model is expected to be quadratic, but contains higher-order terms.

    Some solvers or transformations require the model to have at most quadratic
    expressions. This error signals that unsupported terms were detected.
    """


@export("top", "errors")
class ModelNotUnconstrainedError(Exception):
    """
    Raised when an operation requires an unconstrained model, but constraints are present.

    Some solution methods may only work on unconstrained models, such as when
    transforming a symbolic model to a low-level format.
    """


@export("top", "errors")
class ModelVtypeError(Exception):
    """
    Raised when an operation has certain constraints on a model's variable types that
    are violated.

    Some solution methods may only work on models where all variables have the same
    type, or where only certain variable types are permitted.
    """


@export("top", "errors")
class SolutionCreationError(Exception):
    """
    Raised when something goes wrong during the creation of a solution.

    This may happen during the translation to an AqSolution from a different solution
    format, e.g., when the samples have different lengths or the variable types are not
    consistent with the model the solution is created for.
    """
