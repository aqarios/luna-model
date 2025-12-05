from __future__ import annotations
from typing import TYPE_CHECKING, Protocol

from luna_model.solution.sample import Sample

if TYPE_CHECKING:
    from numpy.typing import NDArray


class Result(Protocol):
    @property
    def sample(self) -> Sample: ...
    @property
    def obj_value(self) -> float | None: ...
    @property
    def constraints(self) -> NDArray | None: ...
    @property
    def variable_bounds(self) -> NDArray | None: ...
    @property
    def feasible(self) -> bool | None: ...


class ResultView(Result, Protocol):
    @property
    def index(self) -> int: ...
    @property
    def counts(self) -> int: ...
    @property
    def raw_energy(self) -> float | None: ...
    def __eq__(self, other: ResultView) -> bool: ...  # type: ignore[override]


class ResultIter(Protocol):
    def __iter__(self) -> ResultIter: ...
    def __next__(self) -> ResultView: ...
