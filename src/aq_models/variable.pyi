from typing import overload
from ._core import Vtype
from ._core import Bounds
from ._core import Environment
from ._core import Variable as _Variable

class Variable:
    @overload
    def __new__(
        cls,
        name: str,
        vtype: Vtype | None = None,
        bounds: Bounds | None = None,
    ) -> _Variable: ...
    @overload
    def __new__(
        cls,
        name: str,
        env: Environment,
        vtype: Vtype | None = None,
        bounds: Bounds | None = None,
    ) -> _Variable: ...
