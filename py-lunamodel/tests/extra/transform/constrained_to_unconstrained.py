import pytest

from luna_model import Model, Vtype
from luna_model.transformation import PassManager, pipelines

constrained_model = Model("constrained_model")
x = constrained_model.add_variable("x")
y = constrained_model.add_variable("y")
z = constrained_model.add_variable("z")

constrained_model.objective = 2 * x * z + 3 * x * y + 4 * y * z + 10 * x + 3 * y + 12 * z - 12

constrained_model.constraints += x + y + z <= 3
constrained_model.constraints += x - y - z >= 0
constrained_model.constraints += x + y == 2


constr_to_unconstr_pipeline = pipelines.ConstrainedToUnconstrainedPipeline()

print(constr_to_unconstr_pipeline)

pm = PassManager([constr_to_unconstr_pipeline])# penalty_factor=10
ir = pm.run(constrained_model)
unconstrained_model = ir.model
unconstrained_model.name = "unconstrained_model"
print(unconstrained_model.objective)
# 2 x z + 3 x y + 4 y z + 10 x + 3 y + 12 z - 12 +
#  120 (x + y + z + slack_c0_b0 + 2*slack_c0_b1 )^2 + 120 (-x + y + z + slack_c1_b0 -1 )^2 + 120 (x + y - 2)^2
print(len(unconstrained_model.constraints))
print()
print(unconstrained_model)
