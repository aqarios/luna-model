from aqmodels import Model
from aqmodels._core import Sense, Variable
from aqmodels.transformations import ChangeSensePass, PassManager

# pm = PassManager()
# print(pm)

aqm = Model()
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * y

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

pm = PassManager([p])
model2, cache = pm.run(aqm)

print(cache.max_bias())

