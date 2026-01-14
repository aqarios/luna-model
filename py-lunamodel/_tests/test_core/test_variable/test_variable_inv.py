import pytest
from luna_model import Variable, Environment, Vtype, Solution
from luna_model.errors import UnsupportedOperationError

def test_inverse_binary():
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    inv_b = ~b
    assert f"~{b.name}" == inv_b.name
    assert b != inv_b
    assert inv_b != b
    assert ~inv_b == b
    assert b == ~inv_b
    assert ~(~b) == ~inv_b
    assert ~(~b) == b
    assert ~inv_b == ~(~b)
    assert b == ~(~b)

def test_inverse_binary_lin():
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    expr = 5 * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0}, env=env))[0]
    assert 5.0 == sol, "evaluated value is not 5"


def test_inverse_binary_quad():
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    expr = 5 * ~b * b
    sol = expr.evaluate(Solution.from_dict({"b": 0}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"


def test_inverse_binary_quad2():
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    expr = 5 * b * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"


def test_inverse_binary_ho():
    env = Environment()
    a = Variable("a", vtype=Vtype.BINARY, env=env)
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    c = Variable("c", vtype=Vtype.BINARY, env=env)
    expr = 5 * a * b * c * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0, "a": 1, "c": 1}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"
    assert 0.0 == expr.get_offset()
    assert 0 == len(expr.linear_items())
    assert 0 == len(expr.quadratic_items())
    assert 0 == len(expr.higher_order_items())


def test_inverse_binary_ho2():
    env = Environment()
    a = Variable("a", vtype=Vtype.BINARY, env=env)
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    c = Variable("c", vtype=Vtype.BINARY, env=env)
    expr = 5 * a * c * b * ~b
    sol = expr.evaluate(Solution.from_dict({"b": 0, "a": 1, "c": 1}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"
    assert 0.0 == expr.get_offset()
    assert 0 == len(expr.linear_items())
    assert 0 == len(expr.quadratic_items())
    assert 0 == len(expr.higher_order_items())


def test_inverse_binary_ho3():
    env = Environment()
    a = Variable("a", vtype=Vtype.BINARY, env=env)
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    c = Variable("c", vtype=Vtype.BINARY, env=env)
    expr = 5 * a * ~b * c * b
    sol = expr.evaluate(Solution.from_dict({"b": 0, "a": 1, "c": 1}, env=env))[0]
    assert 0.0 == sol, "evaluated value is not 0.0"
    assert 0.0 == expr.get_offset()
    assert 0 == len(expr.linear_items())
    assert 0 == len(expr.quadratic_items())
    assert 0 == len(expr.higher_order_items())


def test_inverse_int():
    env = Environment()
    b = Variable("b", vtype=Vtype.INTEGER, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


def test_inverse_real():
    env = Environment()
    b = Variable("b", vtype=Vtype.REAL, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


def test_inverse_spin():
    env = Environment()
    b = Variable("b", vtype=Vtype.SPIN, env=env)
    with pytest.raises(UnsupportedOperationError):
        _ = ~b


if __name__ == "__main__":
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    expr = 5 * ~b
    print(expr)
    sol = Solution.from_dict({"b": 0}, env=env)
    sol = expr.evaluate(sol)[0]
    print(sol)
