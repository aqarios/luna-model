from __future__ import annotations
from luna_model._lm import PyEnvironment


class Environment:
    _env: PyEnvironment

    def __init__(self) -> None:
        self._env = PyEnvironment()

    @classmethod
    def _from_pyenv(cls, py_env: PyEnvironment) -> Environment:
        env = cls.__new__(cls)
        env._env = py_env
        return env

    def __enter__(self) -> Environment:
        return self._env.__enter__()

    def __exit__(self, exc_type, exc_value, exc_traceback) -> None:
        return self._env.__exit__(exc_type, exc_value, exc_traceback)
