from typing import overload
from qiskit.primitives import PrimitiveResult, PubResult
from qiskit_optimization import QuadraticProgram
from aqmodels._solution import Solution, Timing
from aqmodels._environment import Environment

class IbmTranslator:
    @overload
    @staticmethod
    def from_ibm(
        result: PrimitiveResult[PubResult], quadratic_program: QuadraticProgram
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
