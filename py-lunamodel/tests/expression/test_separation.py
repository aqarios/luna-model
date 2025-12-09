from luna_model import Environment, Expression, Variable


def test_expression_separation_linear():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a + b + c + d - 3

    left, right = expr.separate([a, c])

    assert left.is_equal(a + c)
    assert right.is_equal(b + d - 3)
    assert (left + right).is_equal(expr)


def test_expression_separation_quadratic_simple():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a * b + c * d

    left, right = expr.separate([a, b])
    assert left.is_equal(a * b)
    assert right.is_equal(c * d)
    assert (left + right).is_equal(expr)


def test_expression_separation_quadratic():
    print()
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = (a + b + c + d - 3) ** 2

    left, right = expr.separate([a, c])
    left_target = (a + c) ** 2 + 2 * (a + c) * (b + d) - 6 * (a + c)
    right_target = (b + d) ** 2 + 9 - 6 * (b + d)
    assert left.is_equal(left_target)
    assert right.is_equal(right_target)
    assert (left + right).is_equal(expr)


def test_expression_separation_higher_order():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        empty = Expression()

        expr = a * b * c - b * c * d

    left, right = expr.separate([a])

    assert left.is_equal(a * b * c)
    assert right.is_equal(-b * c * d)
    assert (left + right).is_equal(expr)

    left, right = expr.separate([a, b])
    assert left.is_equal(expr)
    assert right.is_equal(empty)
    assert (left + right).is_equal(expr)
