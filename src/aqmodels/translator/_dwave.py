from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class DwaveTranslator:
    """
    Utility class for converting between a DWAVE solution and an AqSolution (ours).

    `DWaveSolutionTranslator` provides methods to:
    - Convert a dimod-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external dwave/dimod solvers/samplers or libraries that operate on dwave/dimod-based problem solving/sampling.

    Examples
    --------
    >>> import dimod
    >>> import aqmodels as aqm
    >>> dwave_sampleset = ...
    >>> aqs = aqm.translator.DwaveTranslator.to_aq(dwave_sampleset)
    """

    @dispatched
    @staticmethod
    def to_aq(sample_set, timing, env):
        return sample_set, timing, env
