from typing import overload
from qiskit.primitives import PrimitiveResult, PubResult
from qiskit_optimization import QuadraticProgram
from aqmodels import Solution
from aqmodels import Timing
from aqmodels import Environment

class IbmTranslator:
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
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
