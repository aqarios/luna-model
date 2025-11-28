from lm._core import PyEnvironment


class Environment:
    _env: PyEnvironment

    def __init__(self) -> None:
        self._env = PyEnvironment()
