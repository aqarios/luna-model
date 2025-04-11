from typing import overload

from aqmodels import Solution, Sample, Result
from aqmodels._constraints import Constraints
from aqmodels._environment import Environment
from aqmodels._expression import Expression


class Model:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, name: str) -> None: ...

    @overload
    def __init__(self, env: Environment) -> None: ...

    @overload
    def __init__(
            self,
            name: str | None = ...,
            env: Environment | None = ...,
    ) -> None: ...

    @property
    def name(self) -> str: ...

    @property
    def objective(self) -> Expression: ...

    @objective.setter
    def objective(self, value: Expression): ...

    @property
    def constraints(self) -> Constraints: ...

    @constraints.setter
    def constraints(self, value: Constraints): ...

    @property
    def environment(self) -> Environment: ...

    def num_constraints(self) -> int: ...

    def evaluate(self, solution: Solution) -> Solution: ...

    def evaluate_sample(self, sample: Sample) -> Result: ...

    @overload
    def serialize(self) -> bytes: ...

    @overload
    def serialize(self, compress: bool | None = ...) -> bytes: ...

    @overload
    def serialize(self, level: int | None = ...) -> bytes: ...

    @overload
    def serialize(
            self, compress: bool | None = ..., level: int | None = ...
    ) -> bytes: ...

    @overload
    def encode(self) -> bytes: ...

    @overload
    def encode(self, compress: bool | None = ...) -> bytes: ...

    @overload
    def encode(self, level: int | None = ...) -> bytes: ...

    @overload
    def encode(self, compress: bool | None = ..., level: int | None = ...) -> bytes: ...

    @staticmethod
    def deserialize(data: bytes) -> Model: ...

    @staticmethod
    def decode(data: bytes) -> Model: ...
