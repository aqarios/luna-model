"""Helper script to test out transformations."""

from aqmodels import Model
from aqmodels._core import Sense, Variable
from aqmodels.transformations import ChangeSensePass, MaxBiasAnalysis, PassManager

aqm = Model("Model To transform")
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * 20 * y


m = MaxBiasAnalysis()
c = ChangeSensePass(Sense.Min)

pm = PassManager([m, c])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
model2, cache = pm.run(aqm)

print("=== Model Before Transformation ===")  # noqa: T201
print(aqm)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(model2)  # noqa: T201
