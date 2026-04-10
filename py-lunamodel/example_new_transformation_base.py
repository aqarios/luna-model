from __future__ import annotations

from abc import abstractmethod
from typing import Generic, TypeVar

from luna_model._lm import (
    PyIntegerToBinaryPass,
    PyModel,
    PyPassContext,
    PyPassManager,
    PySolution,
    PyTransformationPass,
    PyTransformationRecord,
)

from luna_model import Environment, Model, Sense, Solution, Vtype
from luna_model.transformation.passes import MaxBiasAnalysis


class PassContext:
    _c: PyPassContext

    @classmethod
    def _from_pyctx(cls, py_ctx: PyPassContext) -> PassContext:
        ctx = cls.__new__(cls)
        ctx._c = py_ctx
        return ctx


A = TypeVar("A")


class AbstractTransformationPass(PyTransformationPass, Generic[A]):
    @abstractmethod
    def name(self) -> str: ...

    @abstractmethod
    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, A]: ...

    @classmethod
    @abstractmethod
    def backward(cls, artifact: A, solution: Solution) -> Solution: ...

    def requires(self) -> list[str]:
        return []

    def invalidates(self) -> list[str]:
        return []

    def _forward(self, model: PyModel, ctx: PyPassContext) -> tuple[PyModel, A]:
        m, a = self.forward(Model._from_pym(model), PassContext._from_pyctx(ctx))
        return m._m, a

    @classmethod
    def _backward(cls, artifact: A, solution: PySolution) -> PySolution:
        return cls.backward(artifact, Solution._from_pys(solution))._s


class IntegerToBinaryPass(PyIntegerToBinaryPass):
    """Todo."""


class ChangeSenseArtifact:
    """Todo."""

    _did_chage: bool

    def __init__(self, did_change: bool) -> None:
        self._did_chage = did_change

    @property
    def did_change(self) -> bool:
        return self._did_chage

    def serialize(self) -> bytes:
        """Todo."""
        return b"\x01" if self._did_chage else b"\x00"

    @classmethod
    def deserialize(cls, buf: bytes) -> ChangeSenseArtifact:
        """Todo."""
        if len(buf) != 1:
            msg = f"Invalid ChangeSenseArtifact payload length: {len(buf)}"
            raise ValueError(msg)
        if buf[0] not in (0, 1):
            msg = f"Invalid ChangeSenseArtifact flag byte: {buf[0]}"
            raise ValueError(msg)
        return cls(did_change=bool(buf[0]))


class ChangeSense(AbstractTransformationPass[ChangeSenseArtifact]):
    """todo."""

    _target: Sense

    def __init__(self, sense: Sense):
        self._target = sense

    def name(self):
        return "change-sense"

    def forward(self, model: Model, _: PassContext) -> tuple[Model, ChangeSenseArtifact]:
        if model.sense == self._target:
            return model, ChangeSenseArtifact(did_change=False)

        model.objective *= -1
        model.sense = self._target
        return model, ChangeSenseArtifact(did_change=True)

    @classmethod
    def backward(cls, artifact: ChangeSenseArtifact, solution: Solution) -> Solution:
        if artifact.did_change:
            if solution.obj_values is not None:
                solution.obj_values *= -1
            solution.sense = Sense.MAX if solution.sense == Sense.MIN else Sense.MIN
        return solution


model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y

pm = PyPassManager([MaxBiasAnalysis(), ChangeSense(Sense.MAX), IntegerToBinaryPass()])
out = pm.run(model._m)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print("-------------------------")
print("BASE SOLUTION (FROM ALG.)")
print("-------------------------")
solution_in = out.model.evaluate(
    Solution([{"x_b0": 0, "x_b1": 1, "y_b0": 1, "y_b1": 1}], env=Environment._from_pyenv(out.model.environment))._s
)
print(solution_in)
print("--------------------------------------")
print("BACKWARD SOLUTION (FOR ORIGINAL MODEL)")
print("--------------------------------------")
sol = out.record.backward(solution_in)
print(sol)
print("--------------------------------------")
print("BACKWARD SOLUTION VIA BLOB (FOR ORIGINAL MODEL)")
print("--------------------------------------")
blob = out.record.encode()
record = PyTransformationRecord.decode(blob)
sol = record.backward(solution_in)
print(sol)
print(out.context)
