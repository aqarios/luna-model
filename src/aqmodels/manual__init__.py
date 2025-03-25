from ._core import Environment, Variable
from ._variable import Variable as __Variable
from ._environment import Environment as __Environment

Variable = __Variable  # type: ignore[misc,assignment]
Environment = __Environment  # type: ignore[misc,assignment]

__all__ = [
    "Variable",
    "Environment",
]
