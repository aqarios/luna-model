from __future__ import annotations

from luna_model import Model, Sense, Solution, Vtype
from luna_model.transformation import (
        PassContext, 
        PassManager, 
        TransformationRecord, 
        TransformationPassArtifact, 
)
from luna_model.transformation import NothingArtifact
from luna_model.transformation.passes import IntegerToBinaryPass

from luna_model.transformation import transform

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

def change_sense_to_max_backward(artifact: ChangeSenseArtifact, solution: Solution) -> Solution:
    print("change_sense_to_max_backward")
    if artifact.did_change:
        if solution.obj_values is not None:
            solution.obj_values *= -1
        solution.sense = Sense.MIN
    return solution

@transform(name="change-sense-to-max", backward=change_sense_to_max_backward)
def change_sense_to_max(model: Model, ctx: PassContext) -> tuple[Model, ChangeSenseArtifact]:
    print("change_sense_to_max forward")
    _ = ctx

    if model.sense == Sense.MAX:
        return model, ChangeSenseArtifact(did_change=False)

    model.objective *= -1
    model.sense = Sense.MAX
    return model, ChangeSenseArtifact(did_change=True)

def mul_by_100_backward(_: NothingArtifact, solution: Solution) -> Solution:
    print("mul_by_100_backward")
    if solution.obj_values is not None:
        solution.obj_values /= 100
    return solution

@transform(backward=mul_by_100_backward)
# @transform(backward=lambda _, s: s if s.obj_values is not None else s)
def mul_by_100(model: Model, ctx: PassContext) -> tuple[Model, NothingArtifact]:
    print("mul_by_100 forward")
    _ = ctx
    model.objective *= 100
    return model, NothingArtifact()


model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y

# print(mul_by_100_backward.__module__)
# print(mul_by_100_backward.__name__)
# print(mul_by_100_backward.__qualname__)
# print()
# print(something.__module__)
# print(something.__name__)
# print(something.__qualname__)
# exit(1)

pm = PassManager([change_sense_to_max, IntegerToBinaryPass(), mul_by_100])
out = pm.run(model)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print("-------------------------")
print("BASE SOLUTION (FROM ALG.)")
print("-------------------------")
solution_in = out.model.evaluate(
    Solution([{"x_b0": 0, "x_b1": 1, "y_b0": 1, "y_b1": 1}], env=out.model.environment, sense=out.model.sense)
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
print(sol.sense)
