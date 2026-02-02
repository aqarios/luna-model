from luna_model import Model, Sense, Variable
from luna_model.transformation import AnalysisCache, TransformationOutcome, transform, PassManager
from luna_model.transformation.action_type import ActionType


@transform()
def normalize_constraints(
    model: Model, cache: AnalysisCache
) -> tuple[Model, ActionType, dict[str, float]] | TransformationOutcome:
    """Normalize constraints into exptected format."""
    _ = cache
    if len(model.constraints) == 0:
        return TransformationOutcome.nothing(model)
    scaling = {"abc": 15.0}
    return model, ActionType.DID_TRANSFORM, scaling


lm = Model()
lm.set_sense(sense=Sense.MAX)
with lm.environment:
    x = Variable("x")
    y = Variable("y")
lm.objective = x * 20 * y
lm.constraints += x + y <= 30


pm = PassManager([normalize_constraints])
ir = pm.run(lm)

print(ir)
