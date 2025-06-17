from aqmodels import Model
from aqmodels._core import Sense, Variable
from aqmodels.transformations import (
    ChangeSensePass,
    PassManager,
    MaxBiasAnalysis,
    TransformationPass,
    TransformationType,
)

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


class PyChangeSensePass(TransformationPass):
    sense: Sense

    def __init__(self, sense: Sense) -> None:
        self.sense = sense

    @property
    def name(self) -> str:
        return "py-change-sense"

    @property
    def requires(self) -> list[str]:
        return []

    def run(self, model: Model, cache) -> tuple[Model, TransformationType]:
        if self.sense == model.sense:
            return model, TransformationType.NoTranform

        model.objective *= -1
        model.set_sense(self.sense)
        return model, TransformationType.DidTransform


p = ChangeSensePass()
print(
    p.name,
    p.requires,
    p.sense,
)

m = MaxBiasAnalysis()

pycsp = PyChangeSensePass(Sense.Min)
print(
    pycsp.name,
)

# print(pycsp.run(aqm, None))

# pm = PassManager([p, m])
pm = PassManager([m, pycsp])

print(pm)
model2, cache = pm.run(aqm)

# print(cache.max_bias().val)
print(model2)
