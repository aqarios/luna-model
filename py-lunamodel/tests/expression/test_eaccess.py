from luna_model import Environment, Expression, Variable


def test_expr_access():
    env = Environment()
    expr = Expression(env)
    assert env == expr.environment
    assert 0 == expr.num_variables
    assert 0 == expr.get_offset()
    assert 0 == expr.degree()
    assert 0 == len(list(expr.items()))
    assert 0 == len(list(expr.variables()))
    assert 0 == len(list(expr.linear_items()))
    assert 0 == len(list(expr.quadratic_items()))
    assert 0 == len(list(expr.higher_order_items()))
    assert True is expr.is_constant()
    assert expr.is_equal(expr)
    assert expr.is_equal_contents(expr)


def test_expr_access_lin():
    env = Environment()
    expr = Expression(env)
    a = Variable("a", env=env)
    b = Variable("b", env=env)
    expr += a + b
    assert env == expr.environment
    assert 2 == expr.num_variables
    assert 0 == expr.get_offset()
    assert 1 == expr.degree()
    assert 2 == len(list(expr.items()))
    assert 2 == len(list(expr.variables()))
    assert 2 == len(list(expr.linear_items()))
    assert 0 == len(list(expr.quadratic_items()))
    assert 0 == len(list(expr.higher_order_items()))
    assert False is expr.is_constant()
    assert expr.is_equal(expr)
    assert expr.is_equal_contents(expr)
