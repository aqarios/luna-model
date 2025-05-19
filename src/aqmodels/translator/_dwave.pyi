from typing import overload

from dimod import SampleSet

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class DwaveTranslator:
    """
    Utility class for converting between a DWAVE solution and our solution format.

    `DWaveSolutionTranslator` provides methods to:
    - Convert a dimod-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external dwave/dimod
    solvers/samplers or libraries that operate on dwave/dimod-based problem-solving/sampling.

    Examples
    --------
    >>> import dimod
    >>> import luna_quantum as lq
    >>> dwave_sampleset = ...
    >>> aqs = lq.translator.DwaveTranslator.to_aq(dwave_sampleset)
    """
    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet, timing: Timing | None = ...) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet, *, env: Environment | None) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        sample_set: SampleSet, timing: Timing | None = ..., *, env: Environment | None
    ) -> Solution:
        """
        Convert a DWave SampleSet to our solution format.

        Parameters
        ----------
        sample_set : SampleSet
            The SampleSet returned by a DWave solver.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of the
                two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...
