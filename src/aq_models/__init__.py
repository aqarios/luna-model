from ._core import Environment
from ._core import Vtype
from ._core import Bounds

# from ._core import Expression
from ._core import VariableExistsException
from ._core import Variable as Var


GLOBAL_ENV: Environment = Environment()


class Variable(Var):
    def __new__(
        cls,
        name: str,
        environment: Environment = GLOBAL_ENV,
        vtype: Vtype | None = None,
        bounds: Bounds | None = None,
    ) -> Var:
        return super().__new__(cls, name, environment, vtype, bounds)


__all__ = [
    "Variable",
    "Vtype",
    "Bounds",
    "Environment",
    # "Expression",
    "VariableExistsException",
    "GLOBAL_ENV",
]
