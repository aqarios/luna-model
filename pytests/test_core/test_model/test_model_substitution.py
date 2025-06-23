import pytest

from aqmodels import Model, Variable, Vtype


@pytest.mark.model_substitution
def test_model_substitution():
    m = Model()
    with m.environment:
        a = Variable("a", vtype=Vtype.Integer)
        target = Variable("n", vtype=Vtype.Integer)

        x1 = Variable("x_1")
        x2 = Variable("x_2")
        x3 = Variable("x_3")

    replacement = x1 + 2 * x2 + 4 * x3

    base_obj = a * 3.4 + 10.10 * target
    expected_obj = 3.4 * a + 10.10 * x1 + 20.20 * x2 + 40.40 * x3

    constr_a = target * target
    expected_constr_a = x1 + 4 * x2 + 16 * x3 + 4 * x1 * x2 + 8 * x1 * x3 + 16 * x2 * x3
    constr_b = target * target * target
    expected_constr_b = (
        x1
        + 8 * x2
        + 64 * x3
        + 18 * x1 * x2
        + 60 * x1 * x3
        + 144 * x2 * x3
        + 48 * x1 * x2 * x3
    )

    m.objective = base_obj
    m.constraints += constr_a <= 0, "a"
    m.constraints += constr_b <= 0, "b"
    m.substitute(target, replacement)

    assert expected_obj.is_equal(expected_obj)
    assert expected_constr_a.is_equal(m.constraints[0].lhs)
    assert expected_constr_b.is_equal(m.constraints[1].lhs)
