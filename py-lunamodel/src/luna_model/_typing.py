"""Type definitions and protocols for LunaModel.

This module defines type aliases and protocols used throughout LunaModel
for type hints and type checking. It includes protocols for bounds,
samples, solutions, and filter functions.
"""

from __future__ import annotations

from collections.abc import Callable, Sequence
from typing import TYPE_CHECKING, Any, Protocol, TypeAlias

try:
    from dimod import SampleSet
    from pyscipopt import Model as ScipModel
    from qiskit.primitives import PrimitiveResult, PubResult
except ImportError:
    pass

from luna_model.solution import ResultView
from luna_model.variable import Unbounded, Variable

if TYPE_CHECKING:
    from luna_model._lm import PyBounds


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
