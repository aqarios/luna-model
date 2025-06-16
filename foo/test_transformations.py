from aqmodels import Model
from aqmodels._core import Sense, Variable
from aqmodels.transformations import ChangeSensePass, PassManager, MaxBiasAnalysis

# pm = PassManager()
# print(pm)

aqm = Model()
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * 20 * y

# class Trp(TransformationPass):
#     def __new__(cls):
#         return super().__new__(cls, "test-transformation")
#
#
#     def run(self, model: Model) -> Model:
#         print("Hello from Python")
#         return model
# trp = Trp()

# trp.run(aqm)

p = ChangeSensePass()
print(
    p.name,
    p.requires,
    p.sense,
)

m = MaxBiasAnalysis()

pm = PassManager([p, m])

print(pm)
model2, cache = pm.run(aqm)

print(cache.max_bias().val)

