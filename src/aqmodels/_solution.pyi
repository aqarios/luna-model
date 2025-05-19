from typing import overload

from numpy.typing import NDArray

from aqmodels import ResultIterator
from aqmodels import ResultView
from aqmodels import Samples
from aqmodels import Timing, Variable, Environment, Model
from aqmodels import Vtype

class Solution:
    """
    The solution object that is obtained by running an algorihtm.

    The `Solution` class represents a summary of all data obtained from solving a
    model. It contains samples, i.e., assignments of values to each model variable as
    returned by the algorithm, metadata about the solution quality, e.g., the objective
    value, and the runtime of the algorithm.

    A `Solution` can be constructed explicitly using `from_dict` or by obtaining a solution
    from an algorithm or by converting a different solution format with one of the available
    translators. Note that the latter requires the environment the model was created in.

    Examples
    --------
    Basic usage, assuming that the algorithm already returns a `Solution`:

    >>> from luna_quantum import Model, Solution
    >>> model: Model = ...
    >>> algorithm = ...
    >>> solution: Solution = algorithm.run(model)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    When you have a `dimod.Sampleset` as the raw solution format:

    >>> from luna_quantum.translator import BqmTranslator
    >>> from luna_quantum import Model, Solution, DwaveTranslator
    >>> from dimod import SimulatedAnnealingSampler
    >>> model: Model = ...
    >>> bqm = BqmTranslator.from_aq(model)
    >>> sampleset = SimulatedAnnealingSampler().sample(bqm)
    >>> solution = DwaveTranslator.from_dimod_sample_set(sampleset)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    Serialization:

    >>> blob = solution.encode()
    >>> restored = Solution.decode(blob)
    >>> restored.samples
    [[1, 0, 1], [0, 0, 1]]

    Notes
    -----
    - To ensure metadata like objective values or feasibility, use `model.evaluate(solution)`.
    - Use `encode()` and `decode()` to serialize and recover solutions.
    """

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __iter__(self, /) -> ResultIterator:
        """
        Extract a result view from the `Solution` object.

        Returns
        -------
        ResultView

        Raises
        ------
        TypeError
            If `item` has the wrong type.
        IndexError
            If the row index is out of bounds for the variable environment.
        """
        ...

    def __getitem__(self, item: int, /) -> ResultView:
        """
        Extract a result view from the `Solution` object.

        Returns
        -------
        ResultView

        Raises
        ------
        TypeError
            If `item` has the wrong type.
        IndexError
            If the row index is out of bounds for the variable environment.
        """
        ...

    def __eq__(self, other: Solution, /) -> bool: # type: ignore
        """
        Check whether this solution is equal to `other`.

        Parameters
        ----------
        other : Model

        Returns
        -------
        bool
        """
        ... 

    @property
    def results(self, /) -> ResultIterator:
        """Get an iterator over the single results of the solution."""
        ...

    @property
    def samples(self, /) -> Samples:
        """Get a view into the samples of the solution."""
        ...

    @property
    def obj_values(self, /) -> NDArray:
        """
        Get the objective values of the single samples as a ndarray. A value will be
        None if the sample hasn't yet been evaluated.
        """
        ...

    @property
    def raw_energies(self, /) -> NDArray:
        """
        Get the raw energy values of the single samples as returned by the solver /
        algorithm. Will be None if the solver / algorithm did not provide a value.
        """
        ...

    @property
    def counts(self, /) -> NDArray:
        """Return how often each sample occurred in the solution."""
        ...

    @property
    def runtime(self, /) -> Timing | None:
        """Get the solver / algorithm runtime."""
        ...

    @property
    def best_sample_idx(self, /) -> int | None:
        """Get the index of the sample with the best objective value."""
        ...

    @overload
    def encode(self, /) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool, level: int) -> bytes:
        """
        Serialize the solution into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is True.
        level : int, optional
            Compression level (0–9). Default is 3.

        Returns
        -------
        bytes
            Encoded model representation.

        Raises
        ------
        IOError
            If serialization fails.
        """
        ...

    @overload
    def serialize(self, /) -> bytes: ...
    @overload
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for details.
        """
        ...

    @staticmethod
    def decode(data: bytes) -> Solution:
        """
        Reconstruct a solution object from binary data.

        Parameters
        ----------
        data : bytes
            Serialized model blob created by `encode()`.

        Returns
        -------
        Solution
            The reconstructed solution.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @staticmethod
    def deserialize(data: bytes) -> Solution:
        """Alias for `decode()`."""
        ...

    @staticmethod
    def build(
        component_types: list[Vtype],
        *,
        binary_cols: list[list[int]] | None,
        spin_cols: list[list[int]] | None,
        int_cols: list[list[int]] | None,
        real_cols: list[list[float]] | None,
        raw_energies: list[float | None] | None,
        timing: Timing | None,
        counts: list[int] | None,
    ) -> Solution:
        """
        Build a `Solution` based on the provided input data. The solution is constructed
        based on a column layout of the solution. Let's take the following sample-set with three
        samples as an example:
       
        [ 0  1  -1  3  2.2  1 ]
        [ 1  0  -1  6  3.8  0 ]
        [ 1  1  +1  2  2.4  0 ]
       
        Each row encodes a single sample. However, the variable types vary, the first, second, and
        last columns all represent a Binary variable (index 0, 1, 5). The third column represents a
        variable of type Spin (index 2). The fourth column (index 3), a variable of type Integer and
        the fifth column (index 4), a real-valued variable.
       
        Thus, the `component_types` list is:
       
        >>> component_types = [Vtype.Binary, Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary]
       
        Now we can extract all columns for a binary-valued variable and append them to a new list:
       
        >>> binary_cols = [[0, 1, 1], [1, 0, 1], [1, 0, 0]]
       
        where the first element in the list represents the first column, the second element the\
        second column and the third element the fifth column.
        We do the same for the remaining variable types:
       
        >>> spin_cols = [[-1, -1, +1]]
        >>> int_cols = [[3, 6, 2]]
        >>> real_cols = [[2.2, 3.8, 2.4]]
       
        If we know the raw energies, we can construct them as well:
       
        >>> raw_energies = [-200, -100, +300]
       
        And finally call the `build` function:
       
        >>> sol = Solution.build(
        ...     component_types,
        ...     binary_cols,
        ...     spin_cols,
        ...     int_cols,
        ...     real_cols,
        ...     raw_energies,
        ...     timing,
        ...     counts=[1, 1, 1]
        ... )
        >>> sol
       
        In this example, we could also neglect the `counts` as it defaults to `1`
        for all samples if not set:
       
        >>> sol = Solution.build(
        ...     component_types,
        ...     binary_cols,
        ...     spin_cols,
        ...     int_cols,
        ...     real_cols,
        ...     raw_energies,
        ...     timing
        ... )
        >>> sol
       
       
        Parameters
        ----------
        component_types : list[Vtype]
            The variable type each element in a sample encodes.
        binary_cols : list[list[int]], optional
            The data of all binary valued columns. Each inner list encodes a single binary-valued
            column. Required if any element in the `component_types` is `Vtype.Binary`.
        spin_cols : list[list[int]], optional
            The data of all spin-valued columns. Each inner list encodes a single spin-valued
            column. Required if any element in the `component_types` is `Vtype.Spin`.
        int_cols : list[list[int]], optional
            The data of all integer-valued columns. Each inner list encodes a single integer valued
            column. Required if any element in the `component_types` is `Vtype.Integer`.
        real_cols : list[list[float]], optional
            The data of all real-valued columns. Each inner list encodes a single real-valued
            column. Required if any element in the `component_types` is `Vtype.Real`.
        raw_energies : list[float, optional], optional
            The data of all real valued columns. Each inner list encodes a single real-valued
            column.
        timing : Timing, optional
            The timing data.
        counts : list[int], optional
            The number how often each sample in the solution has occurred. By default, 1 for all
            samples.
       
        Returns
        -------
        Solution
            The constructed solution
       
        Raises
        ------
        RuntimeError
            If a sample column has an incorrect number of samples or if `counts` has
            a length different from the number of samples given.
        """
        ...

    @overload
    @staticmethod
    def from_dict(data: dict[Variable | str, int | float]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, env: Environment
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable | str, int | float], *, model: Model) -> Solution: ...
    @staticmethod
    def from_dict(data: dict[Variable | str, int | float], *, env: Environment | None = ..., model: Model | None = ...) -> Solution:
        """
        Create a `Solution` from a dict that maps variables or variable names to their
        assigned values.

        If a Model is passed, the solution will be evaluated immediately. Otherwise,
        there has to be an environment present to determine the correct variable types.

        Parameters
        ----------
        data : dict[Variable | str, int | float]
            The sample that shall be part of the solution.
        env : Environment, optional
            The environment the variable types shall be determined from.
        model : Model, optional
            A model to evaluate the sample with.

        Returns
        -------
        Solution
            The solution object created from the sample dict.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment or model is passed to the method or available from the
            context.
        ValueError
            If `env` and `model` are both present. When this is the case, the user's
            intention is unclear as the model itself already contains an environment.
        SolutionTranslationError
            Generally if the sample translation fails. Might be specified by one of the
            three following errors.
        SampleIncorrectLengthErr
            If a sample has a different number of variables than the environment.
        SampleUnexpectedVariableError
            If a sample has a variable that is not present in the environment.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...
