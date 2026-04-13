from __future__ import annotations

from luna_model import Model, Sense, Solution, Vtype
from luna_model.transformation import (
        TransformationPass, 
        PassContext, 
        PassManager, 
        TransformationPassArtifact, 
)

from luna_model.transformation import analyze

class DebugArtifact(TransformationPassArtifact):
    def serialize(self) -> bytes:
        return b""

    @classmethod
    def deserialize(cls, buf: bytes) -> DebugArtifact:
        _ = buf
        return cls()


class DebugTransformation(TransformationPass[DebugArtifact]):
    def name(self):
        return "debug"

    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, DebugArtifact]:
        max_bias = ctx.require_analysis(max_bias_analysis.key())
        print(f"analysis element is: {max_bias} with value: {max_bias.value}")
        return model, DebugArtifact()

    @classmethod
    def backward(cls, artifact: DebugArtifact, solution: Solution) -> Solution:
        _ = artifact
        return solution

class MaxBias:
    _val: float

    def __init__(self, val: float) -> None:
        self._val = val

    @property
    def value(self):
        return self._val

@analyze(name="max-bias")
def max_bias_analysis(model: Model, ctx: PassContext) -> MaxBias:
    _ = ctx
    max_val = 0.0

    max_val_lin = max([bias for _, bias in model.objective.linear_items()])
    max_val = max(max_val_lin, max_val)

    if model.objective.has_quadratic():
        max_val_quad = max([bias for _, _, bias in model.objective.quadratic_items()])
        max_val = max(max_val_quad, max_val)

    if model.objective.has_higher_order():
        max_val_ho = max([bias for _, bias in model.objective.higher_order_items()])
        max_val = max(max_val_ho, max_val)

    return MaxBias(max_val)


model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y

pm = PassManager([max_bias_analysis, DebugTransformation()])
out = pm.run(model)
print(out)
