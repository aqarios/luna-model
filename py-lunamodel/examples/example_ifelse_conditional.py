from __future__ import annotations

from luna_model import Model, Sense, Vtype
from luna_model.transformation import PassManager, PassContext
from luna_model.transformation.passes import IntegerToBinaryPass, BinarySpinPass, IfElsePass

model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y

def has_binary_condition(model: Model, _: PassContext) -> bool:
    return Vtype.BINARY in model.vtypes()

conditional = IfElsePass(has_binary_condition, then=[BinarySpinPass(Vtype.SPIN)], otherwise=[], name="has_binary_condition")

pm = PassManager([IntegerToBinaryPass(), conditional])
out = pm.run(model)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print(out.model.variables())
print(out.model.vtypes())
