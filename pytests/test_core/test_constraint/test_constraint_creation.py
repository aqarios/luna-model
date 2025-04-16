import pytest
from aqmodels import Constraint, Constraints, Environment, Expression, Variable


@pytest.fixture
def expression() -> Expression:
    with Environment():
        x = Variable("x")
        y = Variable("y")
        return x + y

@pytest.fixture
def variable() -> Variable:
    with Environment():
        x = Variable("x")
        return x


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

@pytest.mark.constraint
def test_constraint_creation_eq_var(variable: Variable):
    constr = variable == 0
    assert isinstance(constr, Constraint)
    assert constr.name is None


@pytest.mark.constraint
def test_constraint_creation_le_var(variable: Variable):
    constr = variable <= 0.0
    assert isinstance(constr, Constraint)
    assert constr.name is None


@pytest.mark.constraint
def test_constraint_creation_ge_var(variable: Variable):
    constr = variable >= 0
    assert isinstance(constr, Constraint)
    assert constr.name is None

