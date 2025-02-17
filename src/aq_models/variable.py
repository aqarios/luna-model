from ._core import Variable as _Variable
from ._core import Environment
from ._core import Vtype
from ._core import Bounds
from .env import GLOBAL_ENV


class Variable(_Variable):
    def __new__(
        cls,
        name: str,
        env: Environment = GLOBAL_ENV,
        vtype: Vtype | None = None,
        bounds: Bounds | None = None,
    ) -> _Variable:
        return super().__new__(cls, name, env, vtype, bounds)
