import pytest

from aqmodels import Environment, Variable, Vtype


@pytest.mark.expression_substitution
def test_expression_simple():
    with Environment():
        a = Variable("a", vtype=Vtype.Integer)
        target = Variable("target", vtype=Vtype.Integer)

        b1 = Variable("b1")
        b2 = Variable("b2")
        b3 = Variable("b3")

    base = a * 3.4 + 10.10 * target
    replacement = b1 + b2 + b3
    expected = 3.4 * a + 10.10 * (b1 + b2 + b3)

    result = base.substitute(target, replacement)
    assert expected.is_equal(result)


@pytest.mark.expression_substitution
def test_expression_quadratic():
    with Environment():
        target = Variable("n", vtype=Vtype.Integer)

        x1 = Variable("x_1")
        x2 = Variable("x_2")
        x3 = Variable("x_3")

    base = target * target
    replacement = x1 + 2 * x2 + 4 * x3
    expected = x1 + 4 * x2 + 16 * x3 + 4 * x1 * x2 + 8 * x1 * x3 + 16 * x2 * x3

    result = base.substitute(target, replacement)
    assert expected.is_equal(result)


@pytest.mark.expression_substitution
def test_expression_higher_order():
    with Environment():
        target = Variable("n", vtype=Vtype.Integer)

        x1 = Variable("x_1")
        x2 = Variable("x_2")
        x3 = Variable("x_3")

    base = target * target * target
    replacement = x1 + 2 * x2 + 4 * x3
    expected = (
        (x1**3)
        + 6 * (x1**2) * x2
        + 12 * (x1**2) * x3
        + 12 * (x2**2) * x1
        + 48 * x1 * x2 * x3
        + 48 * (x3**2) * x1
        + 8 * (x2**3)
        + 48 * (x2**2) * x3
        + 96 * (x3**2) * x2
        + 64 * (x3**3)
    )

    result = base.substitute(target, replacement)
    assert expected.is_equal(result)
