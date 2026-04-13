from luna_model.transformation import Pipeline
from luna_model.transformation.passes import EqualityConstraintsToQuadraticPenaltyPass

pipeline = Pipeline(name="pipeline_trial", steps=[EqualityConstraintsToQuadraticPenaltyPass()])
pipeline.requires()
# print(pipeline.requires())
# print(pipeline.invalidates())
# print(pipeline.provides())
