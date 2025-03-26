from aqmodels._api_utils import export


@export("top", "errors")
class VariableOutOfRangeError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


@export("top", "errors")
class VariableExistsError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


@export("errors")
class VariablesFromDifferentEnvsError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class DifferentEnvsError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class NoActiveEnvironmentFoundError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class MultipleActiveEnvironmentsError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class DecodeError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class ModelNotQuadraticError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...


class ModelNotUnconstrainedError(Exception):
    """
    VariableOutOfRangeError doc
    """

    ...
