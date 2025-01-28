from ._core import Environment
from ._core import Expression
from ._core import Variable as Var


GLOBAL_ENV: Environment = Environment()


class Variable(Var):
    def __new__(cls, name: str, environment: Environment = GLOBAL_ENV) -> Var:
        return super().__new__(cls, name, environment)


__all__ = [
    "Variable",
    "Environment",
    "Expression",
    "GLOBAL_ENV",
]
