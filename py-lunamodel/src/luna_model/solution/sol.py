from __future__ import annotations
from typing import TYPE_CHECKING, Callable, Literal, Sequence

from luna_model._lm import PySolution
from luna_model.variable.vtype import Vtype
from luna_model.solution.src import ValueSource

if TYPE_CHECKING:
    from luna_model._lm import PyVariable
    from luna_model.variable.var import Variable
    from luna_model.solution.sample import Samples
    from luna_model.solution.timer import Timing
    from luna_model.environment.env import Environment
    from luna_model.model.model import Model
    from luna_model.model.sense import Sense
    from luna_model.solution.res import ResultIter, ResultView
    from luna_model._typing import FilterFn, SoutionFromTypes

    from numpy.typing import NDArray

    SampleT = (
        dict[Variable | str, int | float]
        | dict[Variable, int | float]
        | dict[Variable, int]
        | dict[Variable, float]
        | dict[str, int | float]
        | dict[str, int]
        | dict[str, float]
    )


class Solution:
    _s: PySolution

    def __init__(
        self,
        samples: list[SampleT],
        counts: list[int] | None = None,
        raw_energies: list[float] | None = None,
        obj_values: list[float] | None = None,
        feasible: list[bool] | None = None,
        constraints: list[dict[str, bool]] | None = None,
        variables_bounds: dict[str, list[bool]] | None = None,
        timing: Timing | None = None,
        sense: Sense | None = None,
        env: Environment | None = None,
        vtypes: list[Vtype] | None = None,
    ) -> None:
        self._s = PySolution(
            samples=map_samples(samples),
            counts=counts,
            raw_energies=raw_energies,
            obj_values=obj_values,
            feasible=feasible,
            constraints=constraints,
            variables_bounds=variables_bounds,
            timing=timing,
            sense=sense._val if sense else None,
            env=env._env if env else None,
            vtypes=[vtype._val for vtype in vtypes] if vtypes else None,
        )

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
        return self._s.samples

    @property
    def variable_names(self) -> list[str]:
        return self._s.variable_names

    # @property
    # def best_sample_idx(self) -> list[int]:
    #     return self._s.best_sample_idx

    def best(self) -> list[ResultView] | None:
        return self._s.best

    def cvar(self, alpha: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        return self._s.cvar(alpha, value_toggle._val)

    def temperature_weighted(
        self, beta: float, value_toggle: ValueSource = ValueSource.OBJ
    ) -> float:
        return self._s.temperature_weighted(beta, value_toggle._val)

    def expectation_value(self, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        return self._s.expectation_value(value_toggle._val)

    def feasibility_ratio(self) -> float:
        return self._s.feasibility_ratio()

    def filter(self, f: FilterFn) -> Solution:
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
        max_line_len: int = 80,
        max_col_len: int = 5,
        max_lines: int = 10,
        max_var_name_len: int = 10,
        show_metadata: Literal["before", "after", "hide"] = "after",
    ) -> str:
        return self._s.print(
            layout=layout,
            max_line_len=max_line_len,
            max_col_len=max_col_len,
            max_lines=max_lines,
            max_var_name_len=max_var_name_len,
            show_metadata=show_metadata,
        )

    def add_var(
        self, var: str | Variable, data: list[int | float], vtype: Vtype = Vtype.BINARY
    ) -> None:
        from luna_model.variable import Variable

        self._s.add_var(
            var._v if isinstance(var, Variable) else var,  # type: ignore[attribute]
            data,
            vtype._val,  # type: ignore[attribute]
        )

    def add_vars(
        self,
        variables: list[Variable | str],
        data: list[list[int | float]],
        vtypes: list[Vtype | None] | None = None,
    ) -> None:
        from luna_model.variable import Variable

        self._s.add_vars(
            [var._v if isinstance(var, Variable) else var for var in variables],  # type: ignore[attribute]
            data,
            [v._val if v else None for v in vtypes] if vtypes else None,
        )

    def remove_var(self, var: str | Variable) -> None:
        from luna_model.variable import Variable

        self._s.remove_var(var._v if isinstance(var, Variable) else var)  # type: ignore[attribute]

    def remove_vars(self, variables: list[str | Variable]) -> None:
        from luna_model.variable import Variable

        self._s.remove_vars(
            [var._v if isinstance(var, Variable) else var for var in variables]  # type: ignore[attribute]
        )

    def __len__(self) -> int:
        return self._s.__len__()

    def __iter__(self, /) -> ResultIter:
        return self._s.__iter__()

    def __getitem__(self, item: int) -> ResultView:
        return self._s.__getitem__(item)

    def __eq__(self, other: Solution) -> bool:  # type: ignore[override]
        return self._s.__eq__(other._s)

    def __reduce__(self) -> tuple[Callable, tuple[bytes, ...]]:
        data = self.encode()
        return Solution.decode, (data,)

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
        data: SampleT,
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: int | None = None,
        sense: Sense | None = None,
        energy: float | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_dict(
                data=map_sample(data),
                env=env._env if env else None,
                model=model._m if model else None,  # type: ignore[attribute]
                timing=timing,
                counts=counts,
                sense=sense._val if sense else None,
                energy=energy,
            )
        )

    @classmethod
    def from_dicts(
        cls,
        data: Sequence[SampleT],
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: list[int] | None = None,
        sense: Sense | None = None,
        energies: list[float] | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_dicts(
                data=map_samples(data),
                env=env._env if env else None,
                model=model._m if model else None,  # type: ignore[attribute]
                timing=timing,
                counts=counts,
                sense=sense._val if sense else None,
                energies=energies,
            )
        )

    @classmethod
    def from_arrays(
        cls,
        data: NDArray,
        variables: list[Variable | str] | None = None,
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: list[int] | None = None,
        sense: Sense | None = None,
        energies: list[float] | None = None,
    ) -> Solution:
        return cls._from_pys(
            PySolution.from_arrays(
                data=data,
                variables=[v if isinstance(v, str) else v._v for v in variables],
                env=env._env if env is not None else None,
                model=model._m if model is not None else None,
                timing=timing,
                counts=counts,
                sense=sense._val if sense is not None else None,
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
                model=model._m if model else None,  # type: ignore[attribute]
                timing=timing,
                sense=sense._val if sense else None,
                bit_order=bit_order,
                energies=energies,
                var_order=var_order,
            )
        )

    def __str__(self) -> str:
        return self._s.__str__()

    def __repr__(self) -> str:
        return self._s.__str__()


def map_sample(sample: SampleT) -> dict[str | PyVariable, int | float]:
    return {s if isinstance(s, str) else s._v: v for s, v in sample.items()}


def map_samples(
    samples: Sequence[SampleT],
) -> list[dict[str | PyVariable, int | float]]:
    return [map_sample(s) for s in samples]