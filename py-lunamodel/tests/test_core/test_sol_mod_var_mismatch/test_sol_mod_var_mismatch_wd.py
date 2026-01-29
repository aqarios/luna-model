from pathlib import Path

import numpy as np

from luna_model import Model, Solution


def test_mismatch_var_order():
    model_path = Path(__file__).parent / "model.mod"
    sol_path = Path(__file__).parent / "solution_pre.sol"
    model = Model.decode(model_path.read_bytes())
    sol_pre = Solution.decode(sol_path.read_bytes())
    sol_after = model.evaluate(sol_pre)
    assert sol_pre.obj_values is not None
    assert sol_after.obj_values is not None
    for a, b in zip(sol_pre.obj_values, sol_after.obj_values):
        assert np.isclose(a, b)
    assert sol_pre.raw_energies is not None
    assert sol_after.raw_energies is not None
    for a, b in zip(sol_pre.raw_energies, sol_after.raw_energies):
        assert np.isclose(a, b)
    for a, b in zip([r.feasible for r in sol_pre], [r.feasible for r in sol_after]):
        assert a == b
    assert sol_pre.feasibility_ratio() == sol_after.feasibility_ratio()
