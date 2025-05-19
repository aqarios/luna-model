from typing import overload

from qiskit.primitives import PrimitiveResult, PubResult
from qiskit_optimization import QuadraticProgram

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class IbmTranslator:
    """
    Utility class for converting between an IBM solution and our solution format.


    `IbmTranslator` provides methods to:
    - Convert an IBM-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external ibm solvers/samplers or
    libraries that operate on ibm-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> ...
    >>> ibm_result = ...
    >>> aqs = lq.translator.IbmTranslator.to_aq(ibm_result)
    """
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult], quadratic_program: QuadraticProgram
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        *,
        env: Environment | None,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
        *,
        env: Environment | None,
    ) -> Solution:
        """
        Convert an IBM solution to our solution format.

        Parameters
        ----------
        result : PrimitiveResult[PubResult]
            The ibm result.
        quadratic_program : QuadraticProgram
            The quadratic program defining the optimization problem.
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
