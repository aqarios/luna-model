import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression
from aq_models import Constraint
from aq_models import Constraints


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
    assert constr.name is None


@pytest.mark.constraint
def test_constraint_creation_le(expression: Expression):
    constr = expression <= 0.0
    assert isinstance(constr, Constraint)
    assert constr.name is None


@pytest.mark.constraint
def test_constraint_creation_ge(expression: Expression):
    constr = expression >= 0
    assert isinstance(constr, Constraint)
    assert constr.name is None


@pytest.mark.constraint
def test_constraints_out_of_bounds_access(expression: Expression):
    constr = Constraints()
    constr += expression <= 2
    with pytest.raises(IndexError):
        _ = constr[3]
