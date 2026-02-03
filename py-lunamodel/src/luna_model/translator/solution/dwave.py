"""D-Wave solution translator for LunaModel.

This module provides translation from D-Wave SampleSet format
to LunaModel's Solution format.
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

    Converts D-Wave SampleSet objects to LunaModel Solutions.
    Automatically aggregates duplicate solutions.

    Requires the ``dimod`` package.

    Examples
    --------
    >>> from luna_model.translator import BqmTranslator, DwaveTranslator
    >>> bqm = BqmTranslator.from_lm(model)
    >>> # sampler = DWaveSampler()
    >>> # sampleset = sampler.sample(bqm, num_reads=100)
    >>> solution = DwaveTranslator.to_lm(sampleset)

    See Also
    --------
    BqmTranslator : D-Wave BQM format translator
    CqmTranslator : D-Wave CQM format translator
    """

    @staticmethod
    def to_lm(
        sample_set: "SampleSet",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert D-Wave SampleSet to LunaModel solution.

        Parameters
        ----------
        sample_set : SampleSet
            D-Wave SampleSet returned by a sampler.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable mapping. Required either as parameter or active context.

        Returns
        -------
        Solution
            LunaModel Solution with aggregated samples.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.

        Examples
        --------
        >>> from luna_model import Environment
        >>> # Assuming sampleset is obtained from D-Wave sampler
        >>> with Environment():
        ...     # Create variables in environment
        ...     solution = DwaveTranslator.to_lm(sampleset)
        >>> print(f"Best energy: {solution.best_energy()}")
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
