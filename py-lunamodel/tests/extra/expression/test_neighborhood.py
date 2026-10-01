from luna_model import Environment, Variable


def test_expression_neighborhood_linear():
    with Environment():
        a = Variable("a")
        b = Variable("b")

        expr = 3 * a + 2 * b

    assert expr.neighborhood(a) == set()
    assert expr.neighborhood(b) == set()


def test_expression_neighborhood_quadratic():
    with Environment():
        a = Variable("a")
        b = Variable("b")

        expr = 4 * a * b + a

    assert {v.name for v in expr.neighborhood(a)} == {"b"}
    assert {v.name for v in expr.neighborhood(b)} == {"a"}


def test_expression_neighborhood_higher_order():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = 5 * a * b * c + a

    assert {v.name for v in expr.neighborhood(a)} == {"b", "c"}
    assert {v.name for v in expr.neighborhood(b)} == {"a", "c"}


def test_expression_neighborhood_mixed():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a * b + a * c * d + a

    assert {v.name for v in expr.neighborhood(a)} == {"b", "c", "d"}
    assert {v.name for v in expr.neighborhood(b)} == {"a"}


def test_expression_neighborhood_absent_variable():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

        expr = a * b + c

    assert expr.neighborhood(d) == set()
