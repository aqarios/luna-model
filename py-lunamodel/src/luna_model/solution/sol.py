from __future__ import annotations

from typing import TYPE_CHECKING, Any, Literal

from dimod import SampleSet
from numpy import ndarray
from pyscipopt import Model as ScipModel
from qiskit.primitives import PrimitiveResult

from luna_model._lm import PySolution
from luna_model.solution.src import ValueSource
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from collections.abc import Callable, Sequence

    from numpy.typing import NDArray

    from luna_model._lm import PyVariable
    from luna_model._typing import FilterFn, SolutionFromTypes, _Sample
    from luna_model.environment.env import Environment
    from luna_model.model.model import Model
    from luna_model.model.sense import Sense
    from luna_model.solution.res import ResultIter, ResultView
    from luna_model.solution.sample import Samples
    from luna_model.solution.timer import Timing
    from luna_model.variable.var import Variable


class Solution:
    """Solution."""

    _s: PySolution

    def __init__(  # noqa: PLR0913
        self,
        samples: Sequence[_Sample],
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
        """Create solution."""
        self._s = PySolution(
            samples=_map_samples(samples),
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
        """Get objective values."""
        return self._s.obj_values

    @obj_values.setter
    def obj_values(self, other: NDArray | None) -> None:
        """Set objective values."""
        self._s.obj_values = other

    @property
    def raw_energies(self) -> NDArray | None:
        """Get raw energies."""
        return self._s.raw_energies

    @raw_energies.setter
    def raw_energies(self, other: NDArray | None) -> None:
        """Set raw energies."""
        self._s.raw_energies = other

    @property
    def counts(self) -> NDArray:
        """Get counts."""
        return self._s.counts

    @property
    def runtime(self) -> Timing | None:
        """Get runtime."""
        return self._s.runtime

    @runtime.setter
    def runtime(self, timing: Timing) -> None:
        """Set runtime."""
        self._s.runtime = timing

    @property
    def sense(self) -> Sense:
        """Get sense."""
        return self._s.sense

    @property
    def results(self) -> ResultIter:
        """Get results."""
        return self._s.results

    @property
    def samples(self) -> Samples:
        """Get samples."""
        return self._s.samples

    @property
    def variable_names(self) -> list[str]:
        """Get variable names."""
        return self._s.variable_names

    def best(self) -> list[ResultView] | None:
        """Get the best results."""
        return self._s.best()

    def cvar(self, alpha: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute the cvar."""
        return self._s.cvar(alpha, value_toggle._val)

    def temperature_weighted(self, beta: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute the temperature weighted."""
        return self._s.temperature_weighted(beta, value_toggle._val)

    def expectation_value(self, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute the expectation value."""
        return self._s.expectation_value(value_toggle._val)

    def feasibility_ratio(self) -> float:
        """Compute the feasibility ratio."""
        return self._s.feasibility_ratio()

    def filter(self, f: FilterFn) -> Solution:
        """Filter the solution creating a new one."""
        return self._from_pys(self._s.filter(f))

    def filter_feasible(self) -> Solution:
        """Filter the solution by its feasible."""
        return self._from_pys(self._s.filter_feasible())

    def highest_constraint_violation(self) -> str | None:
        """Compute highest constraint violation."""
        return self._s.highest_constraint_violation()

    def repr_html(self) -> str:
        """Represent for html view."""
        return self._s.repr_html()

    def print(  # noqa: PLR0913
        self,
        layout: Literal["row", "column"] = "column",
        max_line_len: int = 80,
        max_col_len: int = 5,
        max_lines: int = 10,
        max_var_name_len: int = 10,
        show_metadata: Literal["before", "after", "hide"] = "after",
    ) -> str:
        """Get formatted solution string."""
        return self._s.print(
            layout=layout,
            max_line_len=max_line_len,
            max_col_len=max_col_len,
            max_lines=max_lines,
            max_var_name_len=max_var_name_len,
            show_metadata=show_metadata,
        )

    def add_var(self, var: str | Variable, data: list[int | float], vtype: Vtype = Vtype.BINARY) -> None:
        """Add a variable entry."""
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.add_var(
            var._v if isinstance(var, Variable) else var,
            data,
            vtype._val,
        )

    def add_vars(
        self,
        variables: list[Variable | str],
        data: list[list[int | float]],
        vtypes: list[Vtype | None] | None = None,
    ) -> None:
        """Add multiple a variable entries."""
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.add_vars(
            [var._v if isinstance(var, Variable) else var for var in variables],
            data,
            [v._val if v else None for v in vtypes] if vtypes else None,
        )

    def remove_var(self, var: str | Variable) -> None:
        """Remove variable entry."""
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.remove_var(var._v if isinstance(var, Variable) else var)  # type: ignore[attribute]

    def remove_vars(self, variables: list[str | Variable]) -> None:
        """Remove variable entries."""
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.remove_vars(
            [var._v if isinstance(var, Variable) else var for var in variables]  # type: ignore[attribute]
        )

    def __len__(self) -> int:
        """Get solution length."""
        return self._s.__len__()

    def __iter__(self, /) -> ResultIter:
        """Iterate results in solution."""
        return self._s.__iter__()

    def __getitem__(self, item: int) -> ResultView:
        """Get solution item."""
        return self._s.__getitem__(item)

    def __eq__(self, other: Solution) -> bool:  # type: ignore[override]
        """Check solution equality."""
        return self._s.__eq__(other._s)

    def __reduce__(self) -> tuple[Callable[[bytes], Solution], tuple[bytes]]:
        """Reduce solution. Used by pickle."""
        data = self.encode()
        return Solution.decode, (data,)

    def encode(self, compress: bool = True, level: int = 3) -> bytes:
        """Encode solution."""
        return self._s.encode(compress, level)

    def serialize(self, compress: bool = True, level: int = 3) -> bytes:
        """Serialize solution."""
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Solution:
        """Decode solution."""
        return cls._from_pys(PySolution.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Solution:
        """Deserialize solution."""
        return cls.decode(data)

    @classmethod
    def from_(  # noqa: PLR0911
        cls,
        other: SolutionFromTypes,
        timing: Timing | None = None,
        env: Environment | None = None,
        **kwargs: Any,
    ) -> Solution:
        """Create solution form other."""
        if isinstance(other, ScipModel):
            from luna_model.translator.solution.zib import ZibTranslator  # noqa: PLC0415

            return ZibTranslator.to_lm(other, env=env, timing=timing, **kwargs)
        if isinstance(other, SampleSet):
            from luna_model.translator.solution.dwave import DwaveTranslator  # noqa: PLC0415

            return DwaveTranslator.to_lm(other, env=env, timing=timing, **kwargs)
        if isinstance(other, PrimitiveResult):
            from luna_model.translator.solution.ibm import IbmTranslator  # noqa: PLC0415

            return IbmTranslator.to_lm(other, env=env, timing=timing, **kwargs)
        if isinstance(other, ndarray):
            from luna_model.translator.solution.numpy import NumpyTranslator  # noqa: PLC0415

            return NumpyTranslator.to_lm(other, env=env, timing=timing, **kwargs)
        if isinstance(other, dict) and "solution_bitstring" in other:
            from luna_model.translator.solution.qctrl import QctrlTranslator  # noqa: PLC0415

            return QctrlTranslator.to_lm(other, env=env, timing=timing, **kwargs)  # type: ignore[reportArgumentType]
        if isinstance(other, dict) and "samples" in other:
            from luna_model.translator.solution.aws import AwsTranslator  # noqa: PLC0415

            return AwsTranslator.to_lm(other, env=env, timing=timing, **kwargs)  # type: ignore[reportArgumentType]

        if isinstance(other, dict):
            key_types = list(other.keys())
            if len(key_types) > 0 and isinstance(key_types[0], str):
                return Solution.from_counts(other, env=env, timing=timing, **kwargs)  # type: ignore[reportArgumentType]

            return Solution.from_dict(other, env=env, timing=timing, **kwargs)
        if isinstance(other, list):
            return Solution.from_dicts(other, env=env, timing=timing, **kwargs)
        msg = f"unsupported type '{type(other)}'"
        raise ValueError(msg)

    @classmethod
    def from_dict(  # noqa:PLR0913
        cls,
        data: _Sample,
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: int | None = None,
        sense: Sense | None = None,
        energy: float | None = None,
    ) -> Solution:
        """Create solution from dict."""
        return cls._from_pys(
            PySolution.from_dict(
                data=_map_sample(data),
                env=env._env if env else None,
                model=model._m if model else None,  # type: ignore[attribute]
                timing=timing,
                counts=counts,
                sense=sense._val if sense else None,
                energy=energy,
            )
        )

    @classmethod
    def from_dicts(  # noqa: PLR0913
        cls,
        data: Sequence[_Sample],
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: list[int] | None = None,
        sense: Sense | None = None,
        energies: list[float] | None = None,
    ) -> Solution:
        """Create solution from dicts."""
        return cls._from_pys(
            PySolution.from_dicts(
                data=_map_samples(data),
                env=env._env if env else None,
                model=model._m if model else None,  # type: ignore[attribute]
                timing=timing,
                counts=counts,
                sense=sense._val if sense else None,
                energies=energies,
            )
        )

    @classmethod
    def from_arrays(  # noqa: PLR0913
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
        """Create solution from arrays."""
        return cls._from_pys(
            PySolution.from_arrays(
                data=data,
                variables=[v if isinstance(v, str) else v._v for v in variables] if variables is not None else None,
                env=env._env if env is not None else None,
                model=model._m if model is not None else None,
                timing=timing,
                counts=counts,
                sense=sense._val if sense is not None else None,
                energies=energies,
            )
        )

    @classmethod
    def from_counts(  # noqa: PLR0913
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
        """Create solution from counts."""
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

    @classmethod
    def from_random(
        cls,
        n_samples: int,
        seed: int | None = None,
        env: Environment | None = None,
        model: Model | None = None,
        sense: Sense | None = None,
    ) -> Solution:
        """Create a `Solution` from random sampling.

        If a Model is passed, the solution will be evaluated immediately. Otherwise,
        there has to be an environment present to determine the correct variable types.

        Parameters
        ----------
        n_samples : int
            The number of samples drawn randomly.
        seed : int, optional
            The random seed
        env : Environment, optional
            The environment the variable types shall be determined from.
        model : Model, optional
            A model to evaluate the sample with.
        sense : Senes, optional
            The sense if no model is specified

        Returns
        -------
        Solution
            The solution object created from random sampling.
        """
        return cls._from_pys(
            PySolution.from_random(
                n_samples=n_samples,
                seed=seed,
                env=env._env if env else None,
                model=model._m if model else None,
                sense=sense._val if sense else None,
            )
        )

    def aggregate(self) -> Solution:
        """Aggregate a `Solution`.

        Condense solution entries into one with more counts if a solution multiple
        duplicate entries.
        """
        return self._from_pys(self._s.aggregate())

    def __str__(self) -> str:
        """Get solution as string."""
        return self._s.__str__()

    def __repr__(self) -> str:
        """Get solution as debug string."""
        return self._s.__repr__()


def _map_sample(sample: _Sample) -> dict[str | PyVariable, int | float]:
    return {s if isinstance(s, str) else s._v: v for s, v in sample.items()}


def _map_samples(
    samples: Sequence[_Sample],
) -> list[dict[str | PyVariable, int | float]]:
    return [_map_sample(s) for s in samples]
