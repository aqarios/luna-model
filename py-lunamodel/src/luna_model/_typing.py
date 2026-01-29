from __future__ import annotations

from collections.abc import Callable, Sequence
from typing import Any, Protocol, TypeAlias

from dimod import SampleSet  # type: ignore[import]
from pyscipopt import Model as ScipModel  # type: ignore[import]
from qiskit.primitives import PrimitiveResult, PubResult  # type: ignore[import]

from luna_model._lm import PyBounds
from luna_model.solution import ResultView
from luna_model.variable import Unbounded, Variable


class VBounds(Protocol):
    @property
    def _b(self) -> PyBounds: ...
    @property
    def upper(self) -> float | type[Unbounded]: ...
    @property
    def lower(self) -> float | type[Unbounded]: ...


_Sample: TypeAlias = (
    dict[str | Variable, float | int]
    | dict[str | Variable, float]
    | dict[str | Variable, int]
    | dict[str, float]
    | dict[str, int]
    | dict[str, float | int]
    | dict[Variable, float]
    | dict[Variable, int]
    | dict[Variable, float | int]
)
_SampleList: TypeAlias = Sequence[_Sample]

SolutionFromTypes: TypeAlias = (
    dict[str, Any] | SampleSet | PrimitiveResult[PubResult] | ScipModel | _Sample | _SampleList
)

FilterFn: TypeAlias = Callable[[ResultView], bool]
