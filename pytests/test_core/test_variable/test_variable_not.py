import pytest
from luna_model import Variable, Environment, Vtype, Solution
from luna_model.errors import UnsupportedOperationError


@pytest.mark.variable()
def test_inverse_binary_lin():
    env = Environment()
    b = Variable("b", vtype=Vtype.Binary, env=env)
    expr = 5 * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0}, env=env))[0]
    assert 5.0 == sol, "evaluated value is not 5"

@pytest.mark.variable()
def test_inverse_binary_quad():
    env = Environment()
    b = Variable("b", vtype=Vtype.Binary, env=env)
    expr = 5 * ~b * b
    sol = expr.evaluate(Solution.from_dict({"b": 0}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"

@pytest.mark.variable()
def test_inverse_binary_ho():
    env = Environment()
    a = Variable("a", vtype=Vtype.Binary, env=env)
    b = Variable("b", vtype=Vtype.Binary, env=env)
    expr = 5 * ~b * b * ~b + a * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0, "a": 1}, env=env))[0]
    assert 1.0 == sol, "evaluated value is not 0.0"

@pytest.mark.variable()
def test_inverse_int():
    env = Environment()
    b = Variable("b", vtype=Vtype.Integer, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


@pytest.mark.variable()
def test_inverse_real():
    env = Environment()
    b = Variable("b", vtype=Vtype.Real, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


@pytest.mark.variable()
def test_inverse_spin():
    env = Environment()
    b = Variable("b", vtype=Vtype.Spin, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


if __name__ == "__main__":
    env = Environment()
    b = Variable("b", vtype=Vtype.Binary, env=env)
    expr = 5 * ~b
    print(expr)
    sol = Solution.from_dict({"b": 0}, env=env)
    sol = expr.evaluate(sol)[0]
    print(sol)
