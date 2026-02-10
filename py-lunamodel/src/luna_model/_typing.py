# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
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
