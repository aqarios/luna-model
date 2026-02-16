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

from typing import TYPE_CHECKING, Any, Literal

from numpy import ndarray

from luna_model._lm import PySolution
from luna_model.model.sense import Sense
from luna_model.solution.src import ValueSource
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from collections.abc import Callable, Sequence

    from dimod import SampleSet
    from numpy.typing import NDArray
    from pyscipopt import Model as ScipModel
    from qiskit.primitives import PrimitiveResult

    from luna_model._lm import PyVariable
    from luna_model._typing import FilterFn, SolutionFromTypes, _Sample
    from luna_model.environment.env import Environment
    from luna_model.model.model import Model
    from luna_model.solution.res import ResultIter, ResultView
    from luna_model.solution.sample import Samples
    from luna_model.timer import Timing
    from luna_model.variable.var import Variable


class Solution:
    """Optimization algorithm results.

    A Solution stores the results from solving a model, including variable
    assignments (samples), objective function values, feasibility status,
    constraint satisfaction, and runtime information.

    Solutions can be created directly or converted from various solver formats
    (e.g., D-Wave, Qiskit, SCIP).

    Parameters
    ----------
    samples : Sequence[_Sample]
        List of variable assignment dictionaries.
    counts : list[int], optional
        Number of times each sample was observed.
    raw_energies : list[float], optional
        Raw energy values from the solver.
    obj_values : list[float], optional
        Evaluated objective function values.
    feasible : list[bool], optional
        Feasibility status for each sample.
    constraints : list[dict[str, bool]], optional
        Constraint satisfaction by constraint name for each sample.
    variables_bounds : dict[str, list[bool]], optional
        Variable bound satisfaction by variable name.
    timing : Timing, optional
        Runtime and timing information.
    sense : Sense, optional
        Optimization sense (MIN or MAX).
    env : Environment, optional
        The environment for variables.
    vtypes : list[Vtype], optional
        Variable types.

    Attributes
    ----------
    obj_values : NDArray or None
        Objective function values for each sample.
    raw_energies : NDArray or None
        Raw energy values from solver.
    counts : NDArray
        Observation counts for each sample.
    runtime : Timing or None
        Timing information.
    sense : Sense
        Optimization sense.
    results : ResultIter
        Iterator over result views.
    samples : Samples
        Collection of variable assignment samples.
    variable_names : list[str]
        Names of variables in the solution.

    Examples
    --------
    Create solution from samples:

    >>> from luna_model import Solution, Environment, Variable
    >>> env = Environment()
    >>> x = Variable("x", env=env)
    >>> y = Variable("y", env=env)
    >>> samples = [{"x": 0, "y": 1}, {"x": 1, "y": 0}]
    >>> solution = Solution(samples, obj_values=[5.0, 3.0], env=env)

    Get best results:

    >>> best_results = solution.best()
    >>> for result in best_results:
    ...     print(f"Value: {result.obj_value}, Sample: {result.sample.to_dict()}")

    Filter feasible solutions:

    >>> feasible = solution.filter_feasible()

    Compute statistics:

    >>> mean_value = solution.expectation_value()
    >>> feas_ratio = solution.feasibility_ratio()
    """

    _s: PySolution

    def __init__(  # noqa: PLR0913
        self,
        samples: Sequence[_Sample],
        counts: list[int] | None = None,
        raw_energies: list[float] | None = None,
        obj_values: list[float] | None = None,
        feasible: list[bool] | None = None,
        constraints: Sequence[dict[str, bool]] | None = None,
        variables_bounds: dict[str, list[bool]] | None = None,
        timing: Timing | None = None,
        sense: Sense | None = None,
        env: Environment | None = None,
        vtypes: list[Vtype] | None = None,
    ) -> None:
        """Initialize a solution with samples and metadata."""
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
        """Get objective function values for each sample.

        Returns
        -------
        NDArray or None
            Array of objective values, one per sample.
        """
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
        return Sense._from_pysense(self._s.sense)

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
        """Get the best results according to the optimization sense.

        Returns
        -------
        list[ResultView] or None
            List of best results (lowest for MIN, highest for MAX).
        """
        return self._s.best()

    def cvar(self, alpha: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute Conditional Value at Risk (CVaR).

        CVaR is the expected value of the best (lowest for MIN, highest for MAX)
        alpha fraction of samples, weighted by their counts.

        Parameters
        ----------
        alpha : float
            Risk level (0 < alpha <= 1). The fraction of best samples to consider.
        value_toggle : ValueSource, optional
            Whether to use objective values or raw energies. Default is OBJ.

        Returns
        -------
        float
            The CVaR value (expected value of the alpha-tail).

        Raises
        ------
        ValueError
            If alpha is not in the range (0, 1].
        """
        return self._s.cvar(alpha, value_toggle._val)

    def temperature_weighted(self, beta: float, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute temperature-weighted expectation value.

        Parameters
        ----------
        beta : float
            Inverse temperature parameter (beta = 1/T). Higher values emphasize lower-energy states.
        value_toggle : ValueSource, optional
            Whether to use objective values or raw energies. Default is OBJ.

        Returns
        -------
        float
            The temperature-weighted expectation value.
        """
        return self._s.temperature_weighted(beta, value_toggle._val)

    def expectation_value(self, value_toggle: ValueSource = ValueSource.OBJ) -> float:
        """Compute the expectation value weighted by counts.

        Parameters
        ----------
        value_toggle : ValueSource, optional
            Whether to use objective values or raw energies.

        Returns
        -------
        float
            The weighted mean value.
        """
        return self._s.expectation_value(value_toggle._val)

    def feasibility_ratio(self) -> float:
        """Compute the ratio of feasible samples.

        Returns
        -------
        float
            Ratio of feasible samples (0.0 to 1.0).
        """
        return self._s.feasibility_ratio()

    def filter(self, f: FilterFn) -> Solution:
        """Filter results by a custom predicate function.

        Parameters
        ----------
        f : FilterFn
            Function that takes a ResultView and returns bool.

        Returns
        -------
        Solution
            New solution containing only filtered results.
        """
        return self._from_pys(self._s.filter(f))

    def filter_feasible(self) -> Solution:
        """Filter to keep only feasible results.

        Returns
        -------
        Solution
            New solution containing only feasible results.
        """
        return self._from_pys(self._s.filter_feasible())

    def highest_constraint_violation(self) -> str | None:
        """Get the constraint with the highest violation rate.

        Returns
        -------
        str or None
            Name of the constraint with the highest violation rate across all samples,
            or None if no constraints are violated or no constraint information exists.
        """
        return self._s.highest_constraint_violation()

    def print(  # noqa: PLR0913
        self,
        layout: Literal["row", "column"] = "column",
        max_line_len: int = 80,
        max_col_len: int = 5,
        max_lines: int = 10,
        max_var_name_len: int = 10,
        show_metadata: Literal["before", "after", "hide"] = "after",
    ) -> str:
        """Get formatted string representation of the solution.

        Parameters
        ----------
        layout : {"row", "column"}, optional
            Layout orientation for displaying samples. Default is "column".
        max_line_len : int, optional
            Maximum line length in characters. Default is 80.
        max_col_len : int, optional
            Maximum number of samples to display. Default is 5.
        max_lines : int, optional
            Maximum number of variable rows to display. Default is 10.
        max_var_name_len : int, optional
            Maximum variable name length. Default is 10.
        show_metadata : {"before", "after", "hide"}, optional
            Where to show metadata (objective, feasibility, etc.). Default is "after".

        Returns
        -------
        str
            Formatted string representation.
        """
        return self._s.print(
            layout=layout,
            max_line_len=max_line_len,
            max_col_len=max_col_len,
            max_lines=max_lines,
            max_var_name_len=max_var_name_len,
            show_metadata=show_metadata,
        )

    def add_var(self, var: str | Variable, data: Sequence[int | float], vtype: Vtype = Vtype.BINARY) -> None:
        """Add a variable to all samples in the solution.

        Parameters
        ----------
        var : str or Variable
            The variable name or Variable object to add.
        data : Sequence[int or float]
            Values for this variable across all samples. Length must match number of samples.
        vtype : Vtype, optional
            Variable type. Default is BINARY.
        """
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.add_var(
            var._v if isinstance(var, Variable) else var,
            data,
            vtype._val,
        )

    def add_vars(
        self,
        variables: Sequence[Variable | str],
        data: Sequence[Sequence[int | float]],
        vtypes: Sequence[Vtype | None] | None = None,
    ) -> None:
        """Add multiple variables to all samples in the solution.

        Parameters
        ----------
        variables : Sequence[Variable or str]
            List of variable names or Variable objects to add.
        data : Sequence[Sequence[int or float]]
            Values for each variable across all samples. Outer sequence length must
            match number of variables, inner sequence length must match number of samples.
        vtypes : Sequence[Vtype or None], optional
            Variable types for each variable. If None, types are inferred.
        """
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.add_vars(
            [var._v if isinstance(var, Variable) else var for var in variables],
            data,
            [v._val if v else None for v in vtypes] if vtypes else None,
        )

    def remove_var(self, var: str | Variable) -> None:
        """Remove a variable from all samples in the solution.

        Parameters
        ----------
        var : str or Variable
            The variable name or Variable object to remove.
        """
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.remove_var(var._v if isinstance(var, Variable) else var)  # type: ignore[attribute]

    def remove_vars(self, variables: Sequence[str | Variable]) -> None:
        """Remove multiple variables from all samples in the solution.

        Parameters
        ----------
        variables : Sequence[str or Variable]
            List of variable names or Variable objects to remove.
        """
        from luna_model.variable import Variable  # noqa: PLC0415

        self._s.remove_vars(
            [var._v if isinstance(var, Variable) else var for var in variables]  # type: ignore[attribute]
        )

    def __len__(self) -> int:
        """Get the number of samples in the solution.

        Returns
        -------
        int
            Number of samples.
        """
        return self._s.__len__()

    def __iter__(self, /) -> ResultIter:
        """Iterate over all results in the solution.

        Returns
        -------
        ResultIter
            Iterator over ResultView objects.
        """
        return self._s.__iter__()

    def __getitem__(self, item: int) -> ResultView:
        """Get a result by index.

        Parameters
        ----------
        item : int
            Index of the result to retrieve.

        Returns
        -------
        ResultView
            The result at the given index.
        """
        return self._s.__getitem__(item)

    def __eq__(self, other: Solution) -> bool:  # type: ignore[override]
        """Check if two solutions are equal.

        Parameters
        ----------
        other : Solution
            Solution to compare with.

        Returns
        -------
        bool
            True if solutions have identical samples, values, and metadata.
        """
        return self._s.__eq__(other._s)

    def __reduce__(self) -> tuple[Callable[[bytes], Solution], tuple[bytes]]:
        """Support for pickle serialization.

        Returns
        -------
        tuple[Callable[[bytes], Solution], tuple[bytes]]
            Tuple of (decoder function, (encoded data,)) for pickle.

        Notes
        -----
        This method is called automatically by Python's pickle module.
        It uses the solution's encode/decode methods internally.
        """
        data = self.encode()
        return Solution.decode, (data,)

    def encode(self) -> bytes:
        """Encode solution to bytes for serialization.

        Returns
        -------
        bytes
            Encoded solution data.
        """
        return self._s.encode()

    def serialize(self) -> bytes:
        """Serialize solution to bytes. Alias for encode().

        Returns
        -------
        bytes
            Serialized solution data.
        """
        return self.encode()

    @classmethod
    def decode(cls, data: bytes) -> Solution:
        """Decode solution from bytes.

        Parameters
        ----------
        data : bytes
            Encoded solution data.

        Returns
        -------
        Solution
            Decoded solution object.
        """
        return cls._from_pys(PySolution.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Solution:
        """Deserialize solution from bytes. Alias for decode().

        Parameters
        ----------
        data : bytes
            Serialized solution data.

        Returns
        -------
        Solution
            Deserialized solution object.
        """
        return cls.decode(data)

    @classmethod
    def from_(
        cls,
        other: SolutionFromTypes,
        timing: Timing | None = None,
        env: Environment | None = None,
        **kwargs: Any,
    ) -> Solution:
        """Create solution from various solver result formats.

        Automatically detects the format and converts to a Solution object.
        Supports D-Wave SampleSet, Qiskit PrimitiveResult, SCIP Model, numpy arrays,
        dictionaries, and more.

        Parameters
        ----------
        other : SolutionFromTypes
            Result object from a solver (SampleSet, PrimitiveResult, etc.) or
            data structure (dict, list, ndarray).
        timing : Timing, optional
            Runtime information to attach to the solution.
        env : Environment, optional
            Environment for variables. Required for some formats.
        **kwargs : Any
            Additional keyword arguments specific to the source format.

        Returns
        -------
        Solution
            Converted solution object.

        Raises
        ------
        ValueError
            If the format is not recognized or supported.
        RuntimeError
            If required dependencies are not installed.
        """
        translator = _find_translator(other)
        return translator(other, env=env, timing=timing, **kwargs)  # type: ignore[argument]

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
        """Create solution from a single sample dictionary.

        Parameters
        ----------
        data : dict
            Single sample as a dictionary mapping variable names/objects to values.
        env : Environment, optional
            Environment containing the variables.
        model : Model, optional
            Model to evaluate the sample against.
        timing : Timing, optional
            Runtime information.
        counts : int, optional
            Number of times this sample was observed. Default is 1.
        sense : Sense, optional
            Optimization sense. Inferred from model if provided.
        energy : float, optional
            Raw energy value for this sample.

        Returns
        -------
        Solution
            Solution containing one sample.
        """
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
        """Create solution from multiple sample dictionaries.

        Parameters
        ----------
        data : Sequence[_Sample]
            List of samples, each as a dictionary mapping variable names/objects to values.
        env : Environment, optional
            Environment containing the variables.
        model : Model, optional
            Model to evaluate the samples against.
        timing : Timing, optional
            Runtime information.
        counts : list[int], optional
            Number of times each sample was observed. Default is 1 for each.
        sense : Sense, optional
            Optimization sense. Inferred from model if provided.
        energies : list[float], optional
            Raw energy values for each sample.

        Returns
        -------
        Solution
            Solution containing multiple samples.
        """
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
        variables: Sequence[Variable | str] | None = None,
        env: Environment | None = None,
        model: Model | None = None,
        timing: Timing | None = None,
        counts: list[int] | None = None,
        sense: Sense | None = None,
        energies: list[float] | None = None,
    ) -> Solution:
        """Create solution from numpy arrays.

        Parameters
        ----------
        data : NDArray
            2D array where rows are samples and columns are variables.
        variables : Sequence[Variable or str], optional
            Variable names/objects corresponding to columns. Inferred from env or model if not provided.
        env : Environment, optional
            Environment containing the variables.
        model : Model, optional
            Model to evaluate the samples against.
        timing : Timing, optional
            Runtime information.
        counts : list[int], optional
            Number of times each sample was observed. Default is 1 for each.
        sense : Sense, optional
            Optimization sense. Inferred from model if provided.
        energies : list[float], optional
            Raw energy values for each sample.

        Returns
        -------
        Solution
            Solution created from array data.
        """
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
        """Create solution from a counts dictionary.

        Parameters
        ----------
        data : dict[str, int]
            Dictionary mapping bitstrings (e.g., "0101") to observation counts.
        env : Environment, optional
            Environment containing the variables.
        model : Model, optional
            Model to evaluate the samples against.
        timing : Timing, optional
            Runtime information.
        sense : Sense, optional
            Optimization sense. Inferred from model if provided.
        bit_order : {"LTR", "RTL"}, optional
            Bit order interpretation: "RTL" (right-to-left, default) or "LTR" (left-to-right).
        energies : list[float], optional
            Raw energy values for each unique bitstring.
        var_order : list[str], optional
            Order of variable names corresponding to bit positions. Inferred from env or model if not provided.

        Returns
        -------
        Solution
            Solution created from counts data.
        """
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
            A model to evaluate the samples with.
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

    def __str__(self) -> str:
        """Get string representation of the solution.

        Returns
        -------
        str
            Formatted string showing samples and metadata.
        """
        return self._s.__str__()

    def __repr__(self) -> str:
        """Get debug string representation of the solution.

        Returns
        -------
        str
            Debug representation including type and memory address.
        """
        return self._s.__repr__()


def _map_sample(sample: _Sample) -> dict[str | PyVariable, int | float]:
    return {s if isinstance(s, str) else s._v: v for s, v in sample.items()}


def _map_samples(
    samples: Sequence[_Sample],
) -> Sequence[dict[str | PyVariable, int | float]]:
    return [_map_sample(s) for s in samples]


def _find_translator(other: SolutionFromTypes) -> Callable:  # noqa: PLR0911, C901
    scip_model_type = _maybe_scip_model_type()
    sampleset_type = _maybe_sampleset_type()
    primitive_type = _maybe_primitive_type()

    if scip_model_type is not None and isinstance(other, scip_model_type):
        from luna_model.translator.solution.zib import ZibTranslator  # noqa: PLC0415

        return ZibTranslator.to_lm

    if sampleset_type is not None and isinstance(other, sampleset_type):
        from luna_model.translator.solution.dwave import DwaveTranslator  # noqa: PLC0415

        return DwaveTranslator.to_lm

    if primitive_type is not None and isinstance(other, primitive_type):
        from luna_model.translator.solution.ibm import IbmTranslator  # noqa: PLC0415

        return IbmTranslator.to_lm

    if isinstance(other, ndarray):
        from luna_model.translator.solution.numpy import NumpyTranslator  # noqa: PLC0415

        return NumpyTranslator.to_lm
    if isinstance(other, dict) and "solution_bitstring" in other:
        from luna_model.translator.solution.qctrl import QctrlTranslator  # noqa: PLC0415

        return QctrlTranslator.to_lm
    if isinstance(other, dict) and "samples" in other:
        from luna_model.translator.solution.aws import AwsTranslator  # noqa: PLC0415

        return AwsTranslator.to_lm

    if isinstance(other, dict):
        key_types = list(other.keys())
        if len(key_types) > 0 and isinstance(key_types[0], str):
            return Solution.from_counts

        return Solution.from_dict
    if isinstance(other, list):
        return Solution.from_dicts

    type_str = str(type(other))
    if "scip" in type_str:
        msg = "scip is required for translating from a ScipModel. You can install it using the 'scip' extra."
        raise RuntimeError(msg)
    if "qiskit" in type_str:
        msg = (
            "qiskit and qiskit_optimization are required for translating from a PrimitiveResult. "
            "You can install it using the 'qiskit' extra."
        )
        raise RuntimeError(msg)
    if "dimod" in type_str:
        msg = "dimod is required for translating from a SampleSet. You can install it using the 'dimod' extra."
        raise RuntimeError(msg)

    msg = f"unsupported type '{type(other)}'. "
    raise ValueError(msg)


def _maybe_scip_model_type() -> type[ScipModel] | None:
    try:
        from pyscipopt import Model as ScipModel  # noqa: PLC0415

    except ImportError:
        return None
    else:
        return ScipModel


def _maybe_sampleset_type() -> type[SampleSet] | None:
    try:
        from dimod import SampleSet  # noqa: PLC0415

    except ImportError:
        return None
    else:
        return SampleSet


def _maybe_primitive_type() -> type[PrimitiveResult] | None:
    try:
        from qiskit.primitives import PrimitiveResult  # noqa: PLC0415

    except ImportError:
        return None
    else:
        return PrimitiveResult
