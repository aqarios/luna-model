from luna_model import (
    Constant,
    Environment,
    Expression,
    HigherOrder,
    Linear,
    Quadratic,
    Variable,
)


def test_expr_access():
    env = Environment()
    expr = Expression(env)
    assert env == expr.environment
    assert expr.num_variables == 0
    assert expr.get_offset() == 0
    assert expr.degree() == 0
    assert len(list(expr.items())) == 0  # offset is 0 so this is zero as well (everything is zero)
    assert len(list(expr.variables())) == 0
    assert len(list(expr.linear_items())) == 0
    assert len(list(expr.quadratic_items())) == 0
    assert len(list(expr.higher_order_items())) == 0
    assert expr.is_constant()
    assert not expr.has_quadratic()
    assert not expr.has_higher_order()
    assert expr.is_equal(expr)
    assert expr.equal_contents(expr)


def test_expr_access_lin():
    env = Environment()
    expr = Expression(env)
    a = Variable("a", env=env)
    b = Variable("b", env=env)
    expr += a + b
    assert env == expr.environment
    assert expr.num_variables == 2
    assert expr.get_offset() == 0
    assert expr.degree() == 1
    assert len(list(expr.items())) == 2  # constant is zero so not included.
    assert len(list(expr.variables())) == 2

    for elem, bias in expr.items():
        match elem:
            case Linear(_):
                assert bias == 1.0
            case Quadratic(_, _):
                assert False
            case HigherOrder(_):
                assert False
            case Constant():
                assert bias == 0.0

    assert a in list(expr.variables())
    assert b in list(expr.variables())

    assert len(list(expr.linear_items())) == 2
    assert (a, 1.0) in list(expr.linear_items())
    assert (b, 1.0) in list(expr.linear_items())

    assert len(list(expr.quadratic_items())) == 0
    assert len(list(expr.higher_order_items())) == 0
    assert not expr.is_constant()
    assert not expr.has_quadratic()
    assert not expr.has_higher_order()
    assert expr.is_equal(expr)
    assert expr.equal_contents(expr)
