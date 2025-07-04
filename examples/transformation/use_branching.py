"""Helper script to test out transformations."""

from aqmodels import Model
from aqmodels._core import Sense, Variable
from aqmodels.transformations import (
    AnalysisCache,
    ChangeSensePass,
    PassManager,
    IfElsePass,
    Pipeline,
)
from aqmodels.decorators import analyse

aqm = Model("Model To transform")
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * 20 * y


@analyse(name="identify-sense")
def identify_sense(model: Model, _: AnalysisCache) -> Sense:
    return model.sense


p_change_to_max = Pipeline([ChangeSensePass(Sense.Max)])
p_change_to_min = Pipeline([ChangeSensePass(Sense.Min)])

pm = PassManager(
    [
        identify_sense,
        IfElsePass(
            required=["identify-sense"],
            condition=lambda c: c["identify-sense"] == Sense.Min,
            then=p_change_to_max,
            otherwise=p_change_to_min,
        ),
    ]
)

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(aqm)

print("=== Model Before Transformation ===")  # noqa: T201
print(aqm)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(ir.model)  # noqa: T201

print("=== Analysis Cache ===")  # noqa: T201
print(ir.cache)
print("=== Execution Log ===")  # noqa: T201
for item in ir.execution_log:
    print(item.pass_name, item.kind, item.timing)
