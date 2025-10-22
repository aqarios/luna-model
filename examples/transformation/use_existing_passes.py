"""Helper script to test out transformations."""

from luna_model import Model
from luna_model._core import Sense, Variable
from luna_model.transformations import ChangeSensePass, MaxBiasAnalysis, PassManager

lm = Model("Model To transform")
lm.set_sense(sense=Sense.Max)
with lm.environment:
    x = Variable("x")
    y = Variable("y")
lm.objective = x * 20 * y


m = MaxBiasAnalysis()
c = ChangeSensePass(Sense.Min)

pm = PassManager([m, c])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(lm)

print("=== Model Before Transformation ===")  # noqa: T201
print(lm)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(ir.model)  # noqa: T201
