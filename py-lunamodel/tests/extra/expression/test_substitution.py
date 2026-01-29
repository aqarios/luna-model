from luna_model import Environment, Variable, Vtype


def test_expr_substitution():
    with Environment():
        target = Variable("target", vtype=Vtype.INTEGER)
        a = Variable("a", vtype=Vtype.INTEGER)

    expr = 10.10 * target + a * 3.4
    replacement = 2 * target
    expected = 20.20 * target + a * 3.4
    expr = expr.substitute(target, replacement)
    assert expected.is_equal(expr)


def test_expr_substitution2():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        target = Variable("target")
        c = Variable("c")
        d = Variable("d")

    expr = (a + target + c + d - 3) ** 2
    expected = (a + b + c + d - 3) ** 2

    expr_subst = expr.substitute(target, b)
    assert expected.is_equal(expr_subst)
