from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class DwaveTranslator:
    """
    Utility class for converting between a DIMOD solution and an AqSolution (ours).

    `DimodSolutionTranslator` provides mehtods to:
    - Convert a Dimod-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external dimod solvers/samplers or libraries that operate on dimod-based problem solving/sampling.

    Examples
    --------
    >>> import dimod
    >>> import aqmodels as aqm
    >>> dimod_sampleset = ...
    >>> aqs = aqm.translator.DwaveTranslator.to_aq(dimod_sampleset)
    """

    @dispatched
    @staticmethod
    def to_aq(sample_set, timing, env):
        return sample_set, timing, env
