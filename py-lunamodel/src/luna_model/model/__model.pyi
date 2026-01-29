from pathlib import Path
from typing import Literal, overload

from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel  # type: ignore[import]
from numpy.typing import NDArray  # type: ignore[import]

from luna_model.translator import Qubo, TranslationTarget
from luna_model.variable import Vtype

class Model:
    @overload
    @classmethod
    def from_(
        cls,
        cqm: ConstrainedQuadraticModel,
        name: str | None = None,
    ) -> Model: ...
    @overload
    @classmethod
    def from_(
        cls,
        bqm: BinaryQuadraticModel,
        name: str | None = None,
    ) -> Model: ...
    @overload
    @classmethod
    def from_(
        cls,
        filepath: Path,
        name: str | None = None,
    ) -> Model: ...
    @overload
    @classmethod
    def from_(
        cls,
        lp: str,
        name: str | None = None,
    ) -> Model: ...
    @overload
    @classmethod
    def from_(
        cls,
        qubo: NDArray,
        name: str | None = None,
        offset: float | None = None,
        variable_names: list[str] | None = None,
        vtype: Vtype | None = None,
    ) -> Model: ...
    @classmethod
    def from_(
        cls,
        other: ConstrainedQuadraticModel | BinaryQuadraticModel | str | Path | NDArray,
        name: str | None = None,
        **kwargs,
    ) -> Model: ...
    @overload
    def to(self, target: Literal[TranslationTarget.QUBO]) -> Qubo: ...
    @overload
    def to(
        self,
        target: Literal[TranslationTarget.LP],
        filepath: Path,
    ) -> None: ...
    @overload
    def to(self, target: Literal[TranslationTarget.LP]) -> str: ...
    @overload
    def to(self, target: Literal[TranslationTarget.BQM]) -> BinaryQuadraticModel: ...
    @overload
    def to(self, target: Literal[TranslationTarget.CQM]) -> ConstrainedQuadraticModel: ...
    def to(self, target: TranslationTarget) -> Qubo | str | BinaryQuadraticModel | ConstrainedQuadraticModel | None: ...
