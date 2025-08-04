from aqmodels import Constant, Environment, HigherOrder, Linear, Quadratic, Variable


def test_expression_iteration():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = a * b * c - 5.5 * a * b + 2 * c * b - a + 0.0001 * b + 3 * c - 42

        constant = []
        linear = []
        quadratic = []
        higher_order = []
        for e, bias in expr.items():
            match e:
                case Constant():
                    constant.append(bias)
                case Linear(var):
                    linear.append((var, bias))
                case Quadratic(x, y):
                    quadratic.append((x, y, bias))
                case HigherOrder(variables):
                    higher_order.append((variables, bias))

        assert constant == [-42]
        assert set(linear) == {(a, -1), (b, 0.0001), (c, 3)}
        assert set(quadratic) == {(a, b, -5.5), (b, c, 2)}
        assert higher_order == [([a, b, c], 1)]


def test_expression_items():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")

        expr = a * b * c - 5.5 * a * b + 2 * c * b - a + 0.0001 * b + 3 * c - 42

        linear = expr.linear_items()
        quadratic = expr.quadratic_items()
        higher_order = expr.higher_order_items()

        assert set(linear) == {(a, -1), (b, 0.0001), (c, 3)}
        assert set(quadratic) == {(a, b, -5.5), (b, c, 2)}
        assert higher_order == [([a, b, c], 1)]
