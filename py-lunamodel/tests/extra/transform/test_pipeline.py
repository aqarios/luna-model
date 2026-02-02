from luna_model import Sense
from luna_model.transformation import Pipeline
from luna_model.transformation.pass_manager import PassManager
from luna_model.transformation.passes import ChangeSensePass
from luna_model.transformation.passes import MaxBiasAnalysis

print()


p = Pipeline([MaxBiasAnalysis()], name="pipeline")
print(p)
# print(p.passes)
# print(len(p))

print()

p.add(ChangeSensePass(Sense.MIN))
print(p)
# print(p.passes)
# print(len(p))

print()
pm = PassManager([p])

print(pm)
