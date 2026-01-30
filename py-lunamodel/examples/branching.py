from luna_model import Model, Sense, Variable
from luna_model.transformation import (
    AnalysisCache,
    IfElsePass,
    PassManager,
    Pipeline,
)
from luna_model.transformation.decorators import analyse
from luna_model.transformation.passes import (
    ChangeSensePass,
    MaxBiasAnalysis,
)

lm = Model("Model To transform")
lm.set_sense(sense=Sense.MAX)
with lm.environment:
    x = Variable("x")
    y = Variable("y")
lm.objective = x * 20 * y


@analyse()
def identify_sense(model: Model, _: AnalysisCache) -> Sense:
    """AnalysisPass to identify the sense."""
    return model.sense


if_else_s = IfElsePass(
    requires=["identify-sense"],
    condition=lambda c: c["identify-sense"] == Sense.MIN,
    then=Pipeline([identify_sense]),
    otherwise=Pipeline([]),
)
p_change_to_max = Pipeline([MaxBiasAnalysis(), ChangeSensePass(Sense.MAX), identify_sense])
p_change_to_min = Pipeline([ChangeSensePass(Sense.MIN), if_else_s, MaxBiasAnalysis()])
if_else_r = IfElsePass(
    requires=["identify-sense"],
    condition=lambda c: c["identify-sense"] == Sense.MIN,
    then=p_change_to_max,
    otherwise=p_change_to_min,
)

pm = PassManager([identify_sense, if_else_r])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(lm)

print("=== Model Before Transformation ===")  # noqa: T201
print(lm)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(ir.model)  # noqa: T201

print("=== Analysis Cache ===")  # noqa: T201
print(ir.cache)  # noqa: T201
print("=== Execution Log ===")  # noqa: T201
for item in ir.execution_log:
    print(item.pass_name, item.kind, item.timing)  # noqa: T201
