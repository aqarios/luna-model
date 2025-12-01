from __future__ import annotations
from lm._core import PyVariable
from .environment import Environment


class Variable:
    _v: PyVariable

    def __init__(self, name: str, env: Environment | None = None) -> None:
        self._v = PyVariable(name, env._env if env else None)

    @classmethod
    def _from_pyvar(cls, py_var: PyVariable) -> Variable:
        var = cls.__new__(cls)
        var._v = py_var
        return var
