"""Helper script to test out transformations."""

from luna_model import Model
from luna_model._core import Sense, Variable
from luna_model.transformations import (
    AnalysisCache,
    AnalysisPass,
    MaxBiasAnalysis,
    PassManager,
)

lm = Model()
lm.set_sense(sense=Sense.Max)
with lm.environment:
    x = Variable("x")
    y = Variable("y")
lm.objective = x * 20 * y


class PyMaxBiasAnalysis(AnalysisPass):
    """MaxBiasAnalysis in Python."""

    @property
    def name(self) -> str:
        """Name of the AnalysisPass."""
        return "py-max-bias"

    def run(self, model: Model, cache: AnalysisCache) -> float:
        """Run the AnalysisPass."""
        _ = cache
        max_val = 0.0
        for _, bias in model.objective.items():  # noqa: PERF102
            max_val = max(max_val, bias)
        return max_val


m = MaxBiasAnalysis()
pym = PyMaxBiasAnalysis()
pm = PassManager([m, pym])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(lm)
print("Builtin MaxBiasAnalysis =", ir.cache[m.name].val)  # noqa: T201
print("Custom PyMaxBiasAnalysis =", ir.cache[pym.name])  # noqa: T201
