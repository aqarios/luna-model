from luna_model import Constant, Environment, HigherOrder, Linear, Quadratic, Variable


def test_expression_filter_constant():
    with Environment():
        a = Variable("a")

        expr = a + 5

    filtered = expr.filter(lambda term, _: not isinstance(term, Constant))

    assert filtered.is_equal(a + 0)


def test_expression_filter_linear():
    with Environment():
        a = Variable("a")
        b = Variable("b")

        expr = a + 2 * b

    filtered = expr.filter(lambda term, _: not (isinstance(term, Linear) and term.var.is_equal(b)))

    assert filtered.is_equal(a + 0)


def test_expression_filter_quadratic():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = a * b + c

    filtered = expr.filter(lambda term, _: not isinstance(term, Quadratic))

    assert filtered.is_equal(c + 0)


def test_expression_filter_higher_order():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = a * b * c + a

    filtered = expr.filter(lambda term, _: not isinstance(term, HigherOrder))

    assert filtered.is_equal(a + 0)


def test_expression_filter_all_kinds():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a + b * c + b * c * d + 5

    filtered = expr.filter(lambda _term, bias: bias > 0)

    assert filtered.is_equal(a + b * c + b * c * d + 5)

    filtered = expr.filter(lambda term, _: isinstance(term, Linear))

    assert filtered.is_equal(a + 0)
