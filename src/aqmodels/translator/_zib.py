from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class ZibTranslator:
    """
    Utility class for converting between a DIMOD solution and an AqSolution (ours).

    `DimodSolutionTranslator` provides mehtods to:
    - Convert a Dimod-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external dimod solvers/samplers or libraries that operate on dimod-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> from pyscipopt import Model
    >>> model = Model()
    >>> model.readProblem("./path/to/my/model.lp")
    >>> model.optimize()
    >>> aqs = aqm.translator.ZibTranslator.from_zib(model)
    """

    @dispatched
    @staticmethod
    def from_zib(model, timing, env):
        return model, timing, env
