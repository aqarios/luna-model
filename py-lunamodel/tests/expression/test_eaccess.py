from luna_model import Environment, Expression, Variable, Linear, HigherOrder, Quadratic


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
    assert expr.is_constant()
    assert not expr.has_quadratic()
    assert not expr.has_higher_order()
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

    for elem, bias in expr.items():
        assert 1.0 == bias
        assert isinstance(elem, Linear)
        assert not isinstance(elem, Quadratic)
        assert not isinstance(elem, HigherOrder)

    assert a in list(expr.variables())
    assert b in list(expr.variables())

    assert 2 == len(list(expr.linear_items()))
    assert (a, 1.0) in list(expr.linear_items())
    assert (b, 1.0) in list(expr.linear_items())

    assert 0 == len(list(expr.quadratic_items()))
    assert 0 == len(list(expr.higher_order_items()))
    assert not expr.is_constant()
    assert not expr.has_quadratic()
    assert not expr.has_higher_order()
    assert expr.is_equal(expr)
    assert expr.is_equal_contents(expr)
