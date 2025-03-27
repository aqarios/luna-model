from aqmodels._api_utils import export


@export("top", "errors")
class VariableOutOfRangeError(Exception):
    """ """


@export("top", "errors")
class VariableExistsError(Exception):
    """ """


@export("top", "errors")
class VariablesFromDifferentEnvsError(Exception):
    """ """


@export("top", "errors")
class DifferentEnvsError(Exception):
    """ """


@export("top", "errors")
class NoActiveEnvironmentFoundError(Exception):
    """ """


@export("top", "errors")
class MultipleActiveEnvironmentsError(Exception):
    """ """


@export("top", "errors")
class DecodeError(Exception):
    """ """


@export("top", "errors")
class ModelNotQuadraticError(Exception):
    """ """


@export("top", "errors")
class ModelNotUnconstrainedError(Exception):
    """ """
