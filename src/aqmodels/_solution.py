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
