from __future__ import annotations
from typing import TYPE_CHECKING

from luna_model._utils import wrap_var
from luna_model._lm import PyEnvironment

if TYPE_CHECKING:
    from luna_model.variable import Variable


class Environment:
    _env: PyEnvironment

    def __init__(self) -> None:
        self._env = PyEnvironment()

    @classmethod
    def _from_pyenv(cls, py_env: PyEnvironment) -> Environment:
        env = cls.__new__(cls)
        env._env = py_env
        return env

    @classmethod
    def _from_ctx(cls) -> Environment | None:
        return Environment._from_pyenv(PyEnvironment._from_ctx())

    def __enter__(self) -> Environment:
        return Environment._from_pyenv(self._env.__enter__())

    def __exit__(self, exc_type, exc_value, exc_traceback) -> None:
        return self._env.__exit__(exc_type, exc_value, exc_traceback)

    @property
    def num_variables(self) -> int:
        return self._env.num_variables

    @property
    def id(self) -> int:
        return self._env.id

    def get_variable(self, name: str) -> Variable:
        return wrap_var(self._env.get_variable(name))

    def variables(self) -> list[Variable]:
        return [wrap_var(v) for v in self._env.variables()]

    def equal_contents(self, other: Environment) -> bool:
        return self._env.equal_contents(other._env)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        return self._env.encode(compress, level)

    def serialize(
        self, /, compress: bool | None = True, level: int | None = 3
    ) -> bytes:
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Environment:
        return cls._from_pyenv(PyEnvironment.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Environment:
        return cls.decode(data)

    def __reduce__(self):
        return (Environment.decode, (self.encode(),))

    def __eq__(self, other: Environment) -> bool:  # type: ignore[override]
        return self._env.__eq__(other._env)

    def __contains__(self, var: str) -> bool:
        return self._env.__contains__(var)

    def __str__(self) -> str:
        return self._env.__str__()

    def __repr__(self) -> str:
        return self._env.__repr__()
