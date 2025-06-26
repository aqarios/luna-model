"""Helper script to test out transformations."""

from typing import Any
from aqmodels import Model
from aqmodels._core import Sense, Solution, Variable
from aqmodels.transformations import (
    AnalysisCache,
    ChangeSensePass,
    MaxBiasAnalysis,
    PassManager,
    TransformationPass,
    AnalysisPass,
    TransformationType,
)

aqm = Model()
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * 20 * y


# class PyChangeSensePass(TransformationPass):
#     """Transformation pass to change the sense to the target sense."""
# 
#     target_sense: Sense
# 
#     def __init__(self, sense: Sense) -> None:
#         self.target_sense = sense
# 
#     @property
#     def name(self) -> str:
#         """Get the name."""
#         return "py-change-sense"
# 
#     def run(
#         self, model: Model, cache: AnalysisCache
#     ) -> tuple[Model, TransformationType]:
#         """Run method."""
#         _ = cache
#         if self.target_sense == model.sense:
#             return model, TransformationType.NoTranform
# 
#         model.objective *= -1
#         model.set_sense(self.target_sense)
#         return model, TransformationType.DidTransform
# 
#     def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
#         # return super().backwards(solution, cache)
#         return solution
# 
# 
# # p = ChangeSensePass()
# # print(
# #     p.name,
# #     p.requires,
# #     p.sense,
# # )
# 
# class PyMaxBiasAnalysis(AnalysisPass):
#     @property
#     def name(self) -> str:
#         return "py-max-bias"
# 
#     def run(self, model: Model, cache: AnalysisCache) -> float:
#         max_val = 0.0
#         for _, bias in model.objective.items():
#             max_val = max(max_val, bias)
#         return max_val



m = MaxBiasAnalysis()
c = ChangeSensePass(Sense.Min)
# pym = PyMaxBiasAnalysis()
# 
# pycsp = PyChangeSensePass(Sense.Min)
# print(
#     pycsp.name,
# )
# pm = PassManager([m, pycsp, pym, ChangeSensePass(Sense.Max)])
pm = PassManager([m, c])

print("=== PassManager ===")
print(pm)
model2, cache = pm.run(aqm)

print("=== Model ===")
print(model2)

# print(cache)
# print(cache[m.name].val)
print("=== Model ===")
print(model2)

# print(cache)
# print(cache[pym.name])
