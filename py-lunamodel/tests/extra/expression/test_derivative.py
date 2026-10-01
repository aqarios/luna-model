from luna_model import Environment, Expression, Variable


def test_expression_derivative_constant():
    with Environment():
        a = Variable("a")

        expr = Expression.const(5.0)

    assert expr.derivative(a).is_equal(Expression.const(0.0, env=expr.environment))


def test_expression_derivative_linear():
    with Environment():
        a = Variable("a")
        b = Variable("b")

        expr = 3 * a + 2 * b + 7

    assert expr.derivative(a).is_equal(Expression.const(3.0, env=expr.environment))
    assert expr.derivative(b).is_equal(Expression.const(2.0, env=expr.environment))


def test_expression_derivative_quadratic():
    with Environment():
        a = Variable("a")
        b = Variable("b")

        expr = 4 * a * b + a

    assert expr.derivative(a).is_equal(4 * b + 1)
    assert expr.derivative(b).is_equal(4 * a)


def test_expression_derivative_higher_order():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = 5 * a * b * c + a

    assert expr.derivative(a).is_equal(5 * b * c + 1)
    assert expr.derivative(b).is_equal(5 * a * c)


def test_expression_derivative_absent_variable():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a + b * c

    assert expr.derivative(d).is_equal(Expression.const(0.0, env=expr.environment))
