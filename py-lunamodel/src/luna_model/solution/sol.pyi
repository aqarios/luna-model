from __future__ import annotations
from typing import TYPE_CHECKING, Any, overload

from numpy.typing import NDArray

from luna_model.model.sense import Sense
from luna_model.model.model import Model
from luna_model.variable.var import Variable
from luna_model.solution.timer import Timing
from luna_model.environment.environment import Environment

from qiskit.primitives import PrimitiveResult, PubResult  # type: ignore[import]
from qiskit_optimization import QuadraticProgram  # type: ignore[import]
from pyscipopt import Model as ScipModel  # type: ignore[import]
from dimod import SampleSet  # type: ignore[import]

if TYPE_CHECKING:
    from luna_model._typing import _SampleList, _Sample

class Solution:
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        samples: _SampleList,
        /,
        *,
        counts: list[int] | None = ...,
        sense: Sense | None = ...,
        energies: list[float] | None = ...,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        sample: _Sample,
        /,
        *,
        counts: int | None = ...,
        sense: Sense | None = ...,
        energy: float | None = ...,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        result: NDArray,
        /,
        *,
        energies: NDArray,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        result: PrimitiveResult[PubResult],
        /,
        *,
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        sample_set: SampleSet,
        /,
        *,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        result: dict[str, Any],
        /,
        *,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_(  # noqa: D418
        cls,
        result: ScipModel,
        /,
        *,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
    @classmethod
    def from_(
        cls,
        other: dict[str, Any]
        | SampleSet
        | PrimitiveResult[PubResult]
        | ScipModel
        | _Sample
        | _SampleList,
        /,
        timing: Timing | None = ...,
        env: Environment | None = ...,
        **kwargs,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable, int]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        sense: Sense = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable, float]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        sense: Sense = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[str, int]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        energies: list[float] | None = ...,
        sense: Sense = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[str, float]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        sense: Sense = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable | str, int]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        sense: Sense = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
    @overload
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable | str, float]],
        *,
        env: Environment = ...,
        model: Model = ...,
        timing: Timing = ...,
        counts: list[int] = ...,
        sense: Sense = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable | str, int | float]],
        *,
        env: Environment | None = ...,
        model: Model | None = ...,
        timing: Timing | None = ...,
        counts: list[int] | None = ...,
        sense: Sense | None = ...,
        energies: list[float] | None = ...,
    ) -> Solution: ...
