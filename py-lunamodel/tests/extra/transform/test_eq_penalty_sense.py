import pytest

from luna_model import Model, Sense, Solution
from luna_model.transformation import PassManager
from luna_model.transformation.passes import EqualityConstraintsToQuadraticPenaltyPass, MaxBiasAnalysis


@pytest.mark.parametrize("sense", [Sense.MIN, Sense.MAX])
def test_penalty_makes_violation_worse(sense):
    m = Model(sense=sense)
    x = m.add_variable("x")
    y = m.add_variable("y")
    m.objective = x + 3 * y
    m.add_constraint(x + y == 1, "one")

    t = PassManager([MaxBiasAnalysis(), EqualityConstraintsToQuadraticPenaltyPass()]).run(m).model
    feasible = Solution.from_dict({"x": 0, "y": 1}, model=t).obj_values[0]
    infeasible = Solution.from_dict({"x": 1, "y": 1}, model=t).obj_values[0]

    if sense == Sense.MAX:
        assert feasible > infeasible
    else:
        assert feasible < infeasible
