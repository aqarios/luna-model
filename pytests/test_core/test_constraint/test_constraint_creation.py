import pytest

from aq_models import Constraint, Environment, Expression, Variable


@pytest.fixture
def expression() -> Expression:
    with Environment():
        x = Variable("x")
        y = Variable("y")
        return x + y


@pytest.mark.constraint
def test_constraint_creation_eq(expression: Expression):
    constr = expression == 0
    assert isinstance(constr, Constraint)


@pytest.mark.constraint
def test_constraint_creation_le(expression: Expression):
    constr = expression <= 0.0
    assert isinstance(constr, Constraint)


@pytest.mark.constraint
def test_constraint_creation_ge(expression: Expression):
    constr = expression >= 0
    assert isinstance(constr, Constraint)
