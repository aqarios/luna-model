from __future__ import annotations

from luna_model import Model, Sense, Vtype
from luna_model.transformation import PassManager, PassContext, control_flow
from luna_model.transformation.control_flow import ControlFlowPlan
from luna_model.transformation.passes import IntegerToBinaryPass, BinarySpinPass
from luna_model.transformation.pipeline import Pipeline

model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y

THEN_PASSES = Pipeline("then_steps", [BinarySpinPass(Vtype.SPIN)])
ELSE_PASSES = []

@control_flow(name="has_binary_conditional", requires=THEN_PASSES.requires(), invalidates=THEN_PASSES.invalidates(), provides=THEN_PASSES.provides())
def conditional(model: Model, _: PassContext) -> ControlFlowPlan:
    if Vtype.BINARY in model.vtypes():
        return ControlFlowPlan("has_binary_condition::then", THEN_PASSES)
    return ControlFlowPlan("has_binary_condition::else", ELSE_PASSES)

pm = PassManager([IntegerToBinaryPass(), conditional])
out = pm.run(model)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print(out.model.variables())
print(out.model.vtypes())
