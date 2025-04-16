from aqmodels._api_utils import export, dispatched


@export
class Solution:
    """
    The solution object that is obtained by running an algorihtm.

    The ``Solution`` class represents a summary of all data obtained from solving a
    model. It contains samples, i.e., assignments of values to each model variable as
    returned by the algorithm, metadata about the solution quality, e.g., the objective
    value, and the runtime of the algorithm.

    Models can't be constructed explicitly, but rather by obtaining a solution from an
    algorithm, or by converting a different solution format with one of the available
    translators. Note that the latter requires the environment the model was created
    in.

    Examples
    --------
    Basic usage, assuming that the algorithm already returns a ``Solution``:

    >>> from aqmodels import Model, Solution
    >>> model: Model = ...
    >>> algorithm = ...
    >>> solution: Solution = algorithm.run(model)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    When you have a ``dimod.Sampleset`` as raw solution format:

    >>> from aqmodels.translator import BqmTranslator    >>> from aqmodels import Model, Solution, SampleSetTranslator
    >>> from dimod import SimulatedAnnealingSampler
    >>> model: Model = ...
    >>> bqm = BqmTranslator.to_bqm(model)
    >>> sampleset = SimulatedAnnealingSampler().sample(bqm)
    >>> solution = SampleSetTranslator.from_dimod_sample_set(sampleset)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    Serialization:

    >>> blob = solution.encode()
    >>> restored = Solution.decode(blob)
    >>> restored.samples
    [[1, 0, 1], [0, 0, 1]]

    Notes
    -----
    - To ensure metadata like objective values or feasibility, use ``model.evaluate(solution)``.
    - Use ``encode()`` and ``decode()`` to serialize and recover solutions.
    """

    @dispatched
    def __str__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __repr__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @dispatched
    def __eq__(self, item):
        return item

    @property
    @dispatched
    def results(self):
        """Iterate over the single results of the solution"""
        return

    @property
    @dispatched
    def samples(self):
        """Get a view into the samples of the solution"""
        return

    @property
    @dispatched
    def obj_values(self):
        """Get the objective values of the single samples as a ndarray. A value will be
        None if the sample hasn't yet been evaluated."""
        return

    @property
    @dispatched
    def raw_energies(self):
        """Get the raw energy values of the single samples as returned by the solver /
        algorithm. Will be None if the solver / algorithms did not provide a value."""
        return

    @property
    @dispatched
    def num_occurrences(self):
        """Get the number of how often each sample occurred in the solution."""
        return

    @property
    @dispatched
    def runtime(self):
        """Get the solver / algorithm runtime."""
        return

    @property
    @dispatched
    def best_sample_idx(self):
        """Get the index of the sample with the best objective value."""
        return

    @dispatched
    def encode(self, compress=True, level=3):
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
        return compress, level

    @dispatched
    def serialize(self, compress=True, level=3):
        """Alias for ``encode()``."""
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
        """
        Reconstruct a solution object from binary data.

        Parameters
        ----------
        data : bytes
            Serialized model blob created by ``encode()``.

        Returns
        -------
        Solution
            The reconstructed solution.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """Alias for ``decode()``."""
        return data


    @staticmethod
    def build(
        num_occurrences,
        component_types,
        binary_cols,
        spin_cols,
        int_cols,
        real_cols,
        raw_energies,
        timing,
    ):
        """
        Build a Solution based on the provided input data. The solution is constructed
        based on a column layout of the solution. Let's take the following sample-set with three
        samples as an example:

            [ 0  1  -1  3  2.2  1 ]
            [ 1  0  -1  6  3.8  0 ]
            [ 1  1  +1  2  2.4  0 ]

        Each row encodes a single sample. However, the variable types vary, the first, second and last
        columns all represent a Binary variable (index 0, 1, 5). The third column represents a variable
        of type Spin (index 2). The fourth column (index 3) a variable of type Integer and the fith column
        (index 4) a real valued variable.

        Thus, the `component_types` list is: 

            >>> component_types = [Vtype.Binary, Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary]

        Now we can extract all columns for a binary valued variable and append them to a new list:

            >>> binary_cols = [[0, 1, 1], [1, 0, 1], [1, 0, 0]]

        where the first element in the list represents the first column, the second element the second
        column and the third element the fith column.
        We do the same for the remaning variable types:

            >>> spin_cols = [[-1, -1, +1]]
            >>> int_cols = [[3, 6, 2]]
            >>> real_cols = [[2.2, 3.8, 2.4]]

        If we know the raw energies we can construct them as well:

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
            ...     num_occurrences=[1, 1, 1]
            ... )
            >>> sol

        In this example, we could also neglect the `num_occurrences` as it defaults to `1`
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
        binary_cols : list[list[int]] | None
           The data of all binary valued columns. Each inner list encodes a single binary valued column.
           Required if any element in the `component_types` is `Vtype.Binary`.
        spin_cols : list[list[int]] | None
           The data of all spin valued columns. Each inner list encodes a single spin valued column.
           Required if any element in the `component_types` is `Vtype.Spin`.
        int_cols : list[list[int]] | None
           The data of all integer valued columns. Each inner list encodes a single integer valued column.
           Required if any element in the `component_types` is `Vtype.Integer`.
        real_cols : list[list[int]] | None
           The data of all real valued columns. Each inner list encodes a single real valued column.
           Required if any element in the `component_types` is `Vtype.Real`.
        raw_energies : list[float | None] | None
           The data of all real valued columns. Each inner list encodes a single real valued column.
        timing : Timing | None
           The timing data.
        num_occurrences : list[int] | None
           The number each sample in the solution has occurred. By default 1 for all samples.

        Returns
        -------
        Solution
            The constructed solution
        """
        return num_occurrences, component_types, binary_cols, spin_cols, int_cols, real_cols, raw_energies, timing
