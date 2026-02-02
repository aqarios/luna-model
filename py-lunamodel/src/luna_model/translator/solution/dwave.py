"""D-Wave solution translator for LunaModel.

This module provides translation from D-Wave Ocean SDK's SampleSet format
to LunaModel's Solution format. This enables seamless integration with
D-Wave quantum annealers and hybrid solvers.
"""

# type: ignore[reportPossiblyUnboundVariable]
from typing import TYPE_CHECKING

import numpy as np

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import SampleSet

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False

if TYPE_CHECKING:
    from dimod import SampleSet


class DwaveTranslator:
    """Translator for D-Wave solution format.

    DwaveTranslator converts D-Wave Ocean SDK's SampleSet objects to LunaModel's
    Solution format. SampleSets are returned by D-Wave samplers (quantum annealers,
    hybrid solvers, and classical samplers) and contain solution samples, energies,
    and occurrence counts.

    The translator automatically aggregates duplicate solutions and extracts:
    - Variable assignments for each sample
    - Energy values (objective function values)
    - Occurrence counts (number of times each solution was found)

    Requires the ``dimod`` package from D-Wave Ocean SDK.

    Examples
    --------
    Convert D-Wave SampleSet to LunaModel solution:

    >>> from dwave.system import DWaveSampler, EmbeddingComposite
    >>> from luna_model.translator import BqmTranslator, DwaveTranslator
    >>> from luna_model import Model, Timing
    >>> # Create and solve model on D-Wave
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y - 2 * x + y
    >>> bqm = BqmTranslator.from_lm(model)
    >>> sampler = EmbeddingComposite(DWaveSampler())
    >>> sampleset = sampler.sample(bqm, num_reads=100)
    >>> # Convert to LunaModel solution
    >>> solution = DwaveTranslator.to_lm(sampleset)
    >>> print(solution.best())

    With timing information:

    >>> timing = Timing(
    ...     solver=sampleset.info["timing"]["qpu_access_time"] / 1e6,
    ...     total=sampleset.info["timing"]["total_real_time"] / 1e6,
    ... )
    >>> solution = DwaveTranslator.to_lm(sampleset, timing=timing, env=model.env)

    Notes
    -----
    The translator aggregates identical samples, summing their occurrence counts.
    This reduces the solution size when the sampler returns many duplicate solutions.

    D-Wave samplers may return solutions in different variable types (BINARY or SPIN).
    The translator preserves the variable type from the SampleSet.

    See Also
    --------
    BqmTranslator : D-Wave BQM format translator
    CqmTranslator : D-Wave CQM format translator
    NumpyTranslator : NumPy array solution translator
    """

    @staticmethod
    def to_lm(
        sample_set: "SampleSet",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert D-Wave SampleSet to LunaModel solution.

        Converts a D-Wave Ocean SDK SampleSet to a LunaModel Solution object,
        preserving all samples, energies, and occurrence counts.

        Parameters
        ----------
        sample_set : SampleSet
            D-Wave SampleSet returned by a D-Wave sampler. Contains samples,
            energies, and metadata.
        timing : Timing | None, optional
            Timing information for the solution process. Can be extracted from
            ``sample_set.info['timing']`` for detailed D-Wave timing data.
        env : Environment | None, optional
            Environment containing variable information. If provided, maps
            SampleSet variables to the model's variable space.

        Returns
        -------
        Solution
            LunaModel Solution object with aggregated samples.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.

        Examples
        --------
        Basic usage with quantum annealer:

        >>> from dwave.system import DWaveSampler, EmbeddingComposite
        >>> from dimod import BinaryQuadraticModel
        >>> from luna_model.translator import DwaveTranslator
        >>> # Sample on D-Wave
        >>> bqm = BinaryQuadraticModel({"x": -1, "y": -1}, {("x", "y"): 2}, 0.0, "BINARY")
        >>> sampler = EmbeddingComposite(DWaveSampler())
        >>> sampleset = sampler.sample(bqm, num_reads=1000)
        >>> solution = DwaveTranslator.to_lm(sampleset)
        >>> print(f"Best energy: {solution.best_energy()}")
        >>> print(f"Number of unique solutions: {len(solution)}")

        With hybrid solver:

        >>> from dwave.system import LeapHybridSampler
        >>> sampler = LeapHybridSampler()
        >>> sampleset = sampler.sample(bqm)
        >>> solution = DwaveTranslator.to_lm(sampleset)

        Extracting timing information:

        >>> timing_dict = sampleset.info.get("timing", {})
        >>> timing = Timing(
        ...     solver=timing_dict.get("qpu_access_time", 0) / 1e6, total=timing_dict.get("total_real_time", 0) / 1e6
        ... )
        >>> solution = DwaveTranslator.to_lm(sampleset, timing=timing)

        Notes
        -----
        The SampleSet is automatically aggregated before conversion, combining
        duplicate solutions and summing their occurrence counts.
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the DwaveTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        sampleset = sample_set.aggregate()
        variables = sampleset.variables
        record = sampleset.record

        samples = record.sample.astype(np.float64, order="C")
        counts = record.num_occurrences.astype(np.int64, order="C")
        energies = record.energy.astype(np.float64, order="C")

        return Solution.from_arrays(
            data=samples,
            variables=variables,
            counts=counts,
            energies=energies,
            timing=timing,
            env=env,
        )
