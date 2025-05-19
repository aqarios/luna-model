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
    >>> import luna_quantum as lq
    >>> from pyscipopt import Model
    >>> model = Model()
    >>> model.readProblem("./path/to/my/model.lp")
    >>> model.optimize()
    >>> aqs = lq.translator.ZibTranslator.to_aq(model)
    """

    @dispatched
    @staticmethod
    def to_aq(model, timing, env):
        """
        Extract an AqSolution from a ZIB model.

        Parameters
        ----------
        model : pyscipopt.Model
            The Model that ran the optimization.
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
        return model, timing, env
