from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class ZibTranslator:
    """
    Utility class for converting between a Zib solution and an AqSolution (ours).

    `ZibTranslator` provides methods to:

        - Convert a Zib-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external zib solvers/samplers or libraries that operate on zib-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> from pyscipopt import Model
    >>> model = Model()
    >>> model.readProblem("./path/to/my/model.lp")
    >>> model.optimize()
    >>> aqs = aqm.translator.ZibTranslator.to_aq(model)
    """

    @dispatched
    @staticmethod
    def to_aq(model, timing, env):
        return model, timing, env
