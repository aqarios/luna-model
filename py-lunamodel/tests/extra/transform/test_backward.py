import pytest

from luna_model import Model, Vtype, Solution
from luna_model.transformation import PassManager, passes


def test_sol_backward_no_obj():
    model = Model("model_mixed")
    u = model.add_variable("u", vtype=Vtype.BINARY)
    v = model.add_variable("v", vtype=Vtype.BINARY)
    w = model.add_variable("w", vtype=Vtype.BINARY)
    t = model.add_variable("t", vtype=Vtype.BINARY)
    
    model.objective =    7 * u - 2 * ~v + 3 * u * w - 5 * ~u * v * t + 2 * ~w + 9
    model.constraints += u + v + ~w + t <= 3, "c0"
    model.constraints += u * ~v + ~u * w + v * t >= 1, "c1"
    model.constraints += ~u * v * ~w + t == 1, "c2"

    pm = PassManager([passes.ReduceInvertedBinaryPass()])
    out = pm.run(model)

    sol = Solution.from_dicts([
        {"u": 1, "v": 1, "w": 0, "t": 1},
        {"u": 0, "v": 1, "w": 1, "t": 1},
        {"u": 1, "v": 0, "w": 0, "t": 0},
        {"u": 0, "v": 0, "w": 0, "t": 0},
    ], model=out.model)

    sol_backward = out.record.backward(sol)
    assert sol_backward.obj_values is None
