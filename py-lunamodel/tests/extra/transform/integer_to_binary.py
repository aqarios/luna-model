from luna_model import Model, Vtype
from luna_model.transformation import PassManager, passes

constrained_model = Model("constrained_model")
x = constrained_model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = constrained_model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=8)
z = constrained_model.add_variable("z", vtype=Vtype.INTEGER, lower=0, upper=7)

constrained_model.objective = x + y + z

# constrained_model.constraints += x + y + z <= 3
# constrained_model.constraints += x - y - z >= 0
# constrained_model.constraints += x + y == 2


pm = PassManager([passes.IntegerToBinaryPass()])
ir = pm.run(constrained_model)
unconstrained_model = ir.model
print(unconstrained_model.objective)

