"""Helper script to test out transformations."""

from luna_model import Model, Solution
from luna_model._core import Sense, Variable
from luna_model.transformations import (
    AnalysisCache,
    ChangeSensePass,
    PassManager,
    TransformationPass,
    TransformationType,
)

lm = Model()
lm.set_sense(sense=Sense.Max)
with lm.environment:
    x = Variable("x")
    y = Variable("y")
lm.objective = x * 20 * y


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


target_sense = Sense.Max
c = ChangeSensePass(target_sense)
pycsp = PyChangeSensePass(target_sense)
pm = PassManager([c])
py_pm = PassManager([pycsp])

print("=== PassManager (builtin pass) ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(lm)

print("=== PassManager (custom pass) ===")  # noqa: T201
print(py_pm)  # noqa: T201
py_ir = py_pm.run(lm)

print("=== Model Before Transformation ===")  # noqa: T201
print(lm)  # noqa: T201

print("=== Model After Transformation (builtin) ===")  # noqa: T201
print(ir.model)  # noqa: T201

print("=== Model After Transformation (custom pass) ===")  # noqa: T201
print(py_ir.model)  # noqa: T201
