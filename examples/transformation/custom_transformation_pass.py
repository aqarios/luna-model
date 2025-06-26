"""Helper script to test out transformations."""

from aqmodels import Model, Solution
from aqmodels._core import Sense, Variable
from aqmodels.transformations import (
    AnalysisCache,
    ChangeSensePass,
    PassManager,
    TransformationPass,
    TransformationType,
)

aqm = Model()
aqm.set_sense(sense=Sense.Max)
with aqm.environment:
    x = Variable("x")
    y = Variable("y")
aqm.objective = x * 20 * y


class PyChangeSensePass(TransformationPass):
    """Transformation pass to change the sense to the target sense."""

    target_sense: Sense
    last_model: Model | None

    def __init__(self, sense: Sense) -> None:
        self.target_sense = sense
        self.last_model = None

    @property
    def name(self) -> str:
        """Get the name."""
        return "py-change-sense"

    def run(
        self, model: Model, cache: AnalysisCache
    ) -> tuple[Model, TransformationType]:
        """Run method."""
        _ = cache
        if self.target_sense == model.sense:
            return model, TransformationType.NoTranform

        model.objective *= -1
        model.set_sense(self.target_sense)
        self.last_model = model
        return model, TransformationType.DidTransform

    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """Transform solution back."""
        _ = solution, cache
        if not self.last_model:
            raise RuntimeError
        return self.last_model.evaluate(solution)


c = ChangeSensePass(Sense.Min)
pycsp = PyChangeSensePass(Sense.Max)
pm = PassManager([c, pycsp])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
model2, cache = pm.run(aqm)

print("=== Model Before Transformation ===")  # noqa: T201
print(model2)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(model2)  # noqa: T201
