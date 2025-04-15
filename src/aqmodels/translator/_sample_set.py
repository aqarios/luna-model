from aqmodels._api_utils import export


@export("solution_translator", "top")
class SampleSetTranslator:
    """
    Utility class for converting between a DIMOD solution and an AqModels (our) Solution.

    `DimodSolutionTranslator` provides mehtods to:
    - Convert a Dimod-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external dimod solvers/samplers or libraries that operate on dimod-based problem solving/sampling.

    Examples
    --------
    >>> import dimod
    >>> import aqmodels as aqm
    >>> dimod_samplset = ...
    >>> aqs = aqm.translator.DimodSolutionTranslator.from_sampleset(dimod_sampleset)
    """

    @staticmethod
    def from_dimod_sample_set(sample_set, timing=None, env=None):
        return sample_set, timing, env
