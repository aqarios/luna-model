from lm._core import PyVariable
from .environment import Environment


class Variable:
    _var: PyVariable

    def __init__(self, name: str, env: Environment | None = None) -> None:
        self._var = PyVariable(name, env._env if env else None)
