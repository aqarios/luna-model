from aqmodels import Model, Solution, Vtype
from aqmodels.transformations import BinarySpinAnalysis, BinarySpinPass, PassManager

pm = PassManager([BinarySpinAnalysis(Vtype.Binary), BinarySpinPass()])
print(pm)


model = Model()
x = model.add_variable("x")
y = model.add_variable("s", vtype=Vtype.Spin)

model.objective = x - y

ir = pm.run(model)


print(ir.model)
sol = Solution.from_dict({"x_s": 0, "x": 1}, model=ir.model)

print(sol)
