from __future__ import annotations
from lm._core import PyEnvironment


class Environment:
    _env: PyEnvironment

    def __init__(self, data: int | None = None) -> None:
        if data is None:
            self._env = PyEnvironment()
        elif isinstance(data, int):
            self._env = PyEnvironment._from_raw_ptr(data)

    @classmethod
    def _from_pyenv(cls, py_env: PyEnvironment) -> Environment:
        env = cls.__new__(cls)
        env._env = py_env
        return env

    def __enter__(self) -> Environment:
        return self._env.__enter__()

    def __exit__(self, exc_type, exc_value, exc_traceback) -> None:
        return self._env.__exit__(exc_type, exc_value, exc_traceback)
