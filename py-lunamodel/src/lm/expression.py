from lm._core import PyExpression
from .environment import Environment


class Expression:
    """ """

    _expr: PyExpression


    def __init__(self, env: Environment | None = None) -> None:
        self._expr = PyExpression(env._env)
