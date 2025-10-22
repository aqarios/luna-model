from luna_model._core import Environment, Variable
from luna_model.utils import quicksum


def test_expression_variables():
    with Environment():
        x = [Variable(f"x_{i}") for i in range(10)]

        e1 = x[0] + x[2] + x[4]
        e2 = x[1] + x[2] + x[3] + x[4] + x[5]
        e3 = quicksum(x[5:])

    assert set(e1.variables()) == set((x[0], x[2], x[4]))
    assert set(e2.variables()) == set((x[1], x[2], x[3], x[4], x[5]))
    assert set(e3.variables()) == set(x[5:])
