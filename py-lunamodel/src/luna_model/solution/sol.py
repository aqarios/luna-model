from __future__ import annotations
from typing import TYPE_CHECKING, Any, Callable, Literal, TypeAlias

from numpy.typing import NDArray

from luna_model.model.model import Model
from luna_model.model.sense import Sense
from luna_model.solution.res import ResultIter, ResultView
from luna_model.solution.src import ValueSource

from luna_model._lm import PySolution
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from luna_model.variable.var import Variable
    from luna_model.solution.sample import Samples
    from luna_model.solution.timer import Timing
    from luna_model.environment.environment import Environment

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
    _SampleList: TypeAlias = list[_Sample]
    from qiskit.primitives import PrimitiveResult, PubResult  # type: ignore[import]
    from pyscipopt import Model as ScipModel  # type: ignore[import]
    from dimod import SampleSet  # type: ignore[import]

SoutionFromTypes: TypeAlias = (
    dict[str, Any]
    | SampleSet
    | PrimitiveResult[PubResult]
    | ScipModel
    | _Sample
    | _SampleList
)


class Solution:
    _s: PySolution

    @classmethod
    def _from_pys(cls, py_s: PySolution) -> Solution:
        s = cls.__new__(cls)
        s._s = py_s
        return s

    @property
    def obj_values(self) -> NDArray | None:
        return self._s.obj_values

    @obj_values.setter
    def obj_values(self, other: NDArray | None) -> None:
        self._s.obj_values = other

    @property
    def raw_energies(self) -> NDArray | None:
        return self._s.raw_energies

    @raw_energies.setter
    def raw_energies(self, other: NDArray | None) -> None:
        self._s.raw_energies = other

    @property
    def counts(self) -> NDArray:
        return self._s.counts

    @property
    def runtime(self) -> Timing | None:
        return self._s.runtime

    @runtime.setter
    def runtime(self, timing: Timing) -> None:
        self._s.runtime = timing

    @property
    def sense(self) -> Sense:
        return self._s.sense

    @property
    def results(self) -> ResultIter:
        return self._s.results

    @property
    def samples(self) -> Samples:
        return self._s._samples

    @property
    def variable_names(self) -> list[str]:
        return self._s.variable_names

    @property
    def best_sample_idx(self) -> list[int]:
        return self._s.best_sample_idx

    def best(self) -> list[ResultView]:
        return self._s.best

    def cvar(self, alpha: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        return self._s.cvar(alpha, value_toggle.value)

    def temperature_weighted(
        self, beta: float, value_toggle: ValueSource = ValueSource.OBJ
    ) -> float:
        return self._s.temperature_weighted(beta, value_toggle.value)

    def expectation_value(self, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        return self._s.expectation_value(value_toggle.value)

    def feasibility_ratio(self) -> float:
        return self._s.feasibility_ratio()

    def filter(self, f: Callable[[ResultView], bool]) -> Solution:
        return self._from_pys(self._s.filter(f))

    def filter_feasible(self) -> Solution:
        return self._from_pys(self._s.filter_feasible())

    def highest_constraint_violation(self) -> str | None:
        return self._s.highest_constraint_violation()

    def repr_html(self) -> str:
        return self._s.repr_html()

    def print(
        self,
        layout: Literal["row", "column"] = "column",
        max_line_length: int = 80,
        max_column_length: int = 5,
        max_lines: int = 10,
        max_var_name_length: int = 10,
        show_metadata: Literal["before", "after", "hide"] = "after",
    ) -> None:
        self._s.print(
            layout=layout,
            max_line_length=max_line_length,
            max_column_length=max_column_length,
            max_lines=max_lines,
            max_var_name_length=max_var_name_length,
            show_metadata=show_metadata,
        )

    def add_var(
        self, var: str | Variable, data: list[int | float], vtype: Vtype = Vtype.BINARY
    ) -> None:
        self._s.add_var(var._v if isinstance(var, Variable) else var, data, vtype.value)

    def add_vars(
        self,
        variables: list[Variable | str],
        data: list[list[int | float]],
        vtypes: list[Vtype | None] | None = None,
    ) -> None:
        self._s.add_vars(
            [var._v if isinstance(var, Variable) else var for var in variables],
            data,
            [v.value if v else None for v in vtypes] if vtypes else None,
        )

    def remove_var(self, var: str | Variable) -> None:
        self._s.remove_var(var._v if isinstance(var, Variable) else var)

    def remove_vars(self, variables: list[str | Variable]) -> None:
        self._s.remove_vars(
            [var._v if isinstance(var, Variable) else var for var in variables]
        )

    def __len__(self) -> int:
        return self._s.__len__()

    def __iter__(self, /) -> ResultIter:
        return self._s.__iter__()

    def __getitem__(self, item: int) -> ResultView:
        return self._s.__getitem__(item)

    def __eq__(self, other: Solution) -> bool:  # type: ignore[override]
        return self._s.__eq__(other._s)

    def encode(self, compress: bool = True, level: int = 3) -> bytes:
        return self._s.encode(compress, level)

    def serialize(self, compress: bool = True, level: int = 3) -> bytes:
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Solution:
        return cls._from_pys(PySolution.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Solution:
        return cls.decode(data)

    @classmethod
    def from_(
        cls,
        other: SoutionFromTypes,
        timing: Timing | None = None,
        env: Environment | None = None,
        **kwargs,
    ) -> Solution:
        return cls._from_pys(PySolution.from_(other, timing=timing, env=env, **kwargs))

    @classmethod
    def from_dict(
        cls,
        data: dict[Variable | str, int | float],
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: int | None = None,
        sense: Sense | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_dict(
                data=data,
                env=env._env if env else None,
                model=model._m if model else None,
                timing=timing,
                counts=counts,
                sense=sense.value if sense else None,
            )
        )

    @classmethod
    def from_dicts(
        cls,
        data: list[dict[Variable | str, int | float]],
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: list[int] | None = None,
        sense: Sense | None = None,
        energies: list[float] | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_dicts(
                data=data,
                env=env._env if env else None,
                model=model._m if model else None,
                timing=timing,
                counts=counts,
                sense=sense.value if sense else None,
                energies=energies,
            )
        )

    @classmethod
    def from_counts(
        cls,
        data: dict[str, int],
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        sense: Sense | None = None,
        bit_order: Literal["LTR", "RTL"] = "RTL",
        energies: list[float] | None = None,
        var_order: list[str] | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_counts(
                data=data,
                env=env._env if env else None,
                model=model._m if model else None,
                timing=timing,
                sense=sense.value if sense else None,
                bit_order=bit_order,
                energies=energies,
                var_order=var_order,
            )
        )
