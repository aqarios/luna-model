from luna_model import Environment, Variable, Constant, Linear, Quadratic, HigherOrder


def test_expr_iter():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    expr = a + a * b + c * d * e

    for elem, bias in expr.items():
        match elem:
            case Linear(v):
                assert a.is_equal(v)
            case Quadratic(u, v):
                assert (a.is_equal(u) and b.is_equal(v)) or (
                    a.is_equal(v) and b.is_equal(u)
                )
            case HigherOrder(vars):
                assert vars[0].is_equal(c)
                assert vars[1].is_equal(d)
                assert vars[2].is_equal(e)
            case Constant():
                assert 0.0 == bias
