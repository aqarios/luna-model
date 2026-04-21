from __future__ import annotations

from luna_model import Model, Sense, Solution, Vtype
from luna_model.transformation import (
        TransformationPass, 
        PassContext, 
        PassManager, 
        TransformationRecord, 
        TransformationPassArtifact, 
)
from luna_model.transformation.passes import IntegerToBinaryPass

class ChangeSenseArtifact(TransformationPassArtifact):
    _did_chage: bool

    def __init__(self, did_change: bool) -> None:
        self._did_chage = did_change

    @property
    def did_change(self) -> bool:
        return self._did_chage

    def serialize(self) -> bytes:
        return b"\x01" if self._did_chage else b"\x00"

    @classmethod
    def deserialize(cls, buf: bytes) -> ChangeSenseArtifact:
        if len(buf) != 1:
            msg = f"Invalid ChangeSenseArtifact payload length: {len(buf)}"
            raise ValueError(msg)
        if buf[0] not in (0, 1):
            msg = f"Invalid ChangeSenseArtifact flag byte: {buf[0]}"
            raise ValueError(msg)
        return cls(did_change=bool(buf[0]))


class ChangeSense(TransformationPass[ChangeSenseArtifact]):
    _target: Sense

    def __init__(self, sense: Sense):
        self._target = sense

    def name(self):
        return "change-sense"

    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, ChangeSenseArtifact]:
        _ = ctx

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

pm = PassManager([ChangeSense(Sense.MAX), IntegerToBinaryPass()])
out = pm.run(model)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print("-------------------------")
print("BASE SOLUTION (FROM ALG.)")
print("-------------------------")
solution_in = out.model.evaluate(
    Solution([{"x_b0": 0, "x_b1": 1, "y_b0": 1, "y_b1": 1}], env=out.model.environment)
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
record = TransformationRecord.decode(blob)
sol = record.backward(solution_in)
print(sol)
