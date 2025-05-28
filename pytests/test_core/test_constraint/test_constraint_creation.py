import pytest
from aqmodels import (
    Comparator,
    Constraint,
    Constraints,
    Environment,
    Expression,
    Variable,
)


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


@pytest.fixture
def variable_and_variable() -> tuple[Variable, Variable]:
    with Environment():
        x = Variable("x")
        y = Variable("y")
    return x, y


@pytest.fixture
def variable_and_expression() -> tuple[Variable, Expression]:
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
    return x, y + z


@pytest.fixture
def expression_and_var() -> tuple[Expression, Variable]:
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
    return x + y, z


@pytest.fixture
def expression_and_expression() -> tuple[Expression, Expression]:
    with Environment():
        a = Variable("a")
        b = Variable("b")
        x = Variable("x")
        y = Variable("y")
    return a + b, x + y


# INDEXING


@pytest.mark.constraint
def test_constraints_out_of_bounds_access(expression: Expression):
    constr = Constraints()
    constr += expression <= 2
    with pytest.raises(IndexError):
        _ = constr[3]


@pytest.mark.constraint
def test_constraint_creation_neg_constant_to_rhs():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    expr = x + y - 1
    constr = expr == 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(x + y)
    assert constr.rhs == 1.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_pos_constant_to_rhs():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    expr = x + y + 2
    constr = expr == 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(x + y)
    assert constr.rhs == -2.0
    assert constr.comparator == Comparator.Eq


# LHS: EXPRESSION, RHS: FLOAT/INT


@pytest.mark.constraint
def test_constraint_creation_eq(expression: Expression):
    constr = expression == 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le(expression: Expression):
    constr = expression <= 0.0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge(expression: Expression):
    constr = expression >= 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


@pytest.mark.constraint
def test_constraint_creation_eq_direct(expression: Expression):
    constr = Constraint(expression, 0.0, Comparator.Eq)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_direct(expression: Expression):
    constr = Constraint(expression, 0.0, Comparator.Le)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_direct(expression: Expression):
    constr = Constraint(expression, 0.0, Comparator.Ge)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


# LHS: EXPRESSION, RHS: VARIABLE


@pytest.mark.constraint
def test_constraint_creation_eq_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = expression == var
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = expression <= var
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = expression >= var
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


@pytest.mark.constraint
def test_constraint_creation_eq_direct_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = Constraint(expression, var, Comparator.Eq)
    assert isinstance(constr, Constraint)
    assert constr == (expression == var)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_direct_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = Constraint(expression, var, Comparator.Le)
    assert isinstance(constr, Constraint)
    assert constr == (expression <= var)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_direct_rhs_var(
    expression_and_var: tuple[Expression, Variable],
):
    expression, var = expression_and_var
    constr = Constraint(expression, var, Comparator.Ge)
    assert isinstance(constr, Constraint)
    assert constr == (expression >= var)
    assert constr.name is None
    assert constr.lhs.is_equal(expression - var)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


# LHS: EXPRESSION, RHS: EXPRESSION


@pytest.mark.constraint
def test_constraint_creation_eq_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = lhs == rhs
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = lhs <= rhs
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = lhs >= rhs
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


@pytest.mark.constraint
def test_constraint_creation_eq_direct_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = Constraint(lhs, rhs, Comparator.Eq)
    assert isinstance(constr, Constraint)
    assert constr == (lhs == rhs)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_direct_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = Constraint(lhs, rhs, Comparator.Le)
    assert isinstance(constr, Constraint)
    assert constr == (lhs <= rhs)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_direct_rhs_expr(
    expression_and_expression: tuple[Expression, Expression],
):
    lhs, rhs = expression_and_expression
    constr = Constraint(lhs, rhs, Comparator.Ge)
    assert isinstance(constr, Constraint)
    assert constr == (lhs >= rhs)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


# LHS: VARIABLE, RHS: FLOAT/INT


@pytest.mark.constraint
def test_constraint_creation_eq_var(variable: Variable):
    constr = variable == 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_var(variable: Variable):
    constr = variable <= 0.0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_var(variable: Variable):
    constr = variable >= 0
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


@pytest.mark.constraint
def test_constraint_creation_eq_var_direct(variable: Variable):
    constr = Constraint(variable, 0.0, Comparator.Eq)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_var_direct(variable: Variable):
    constr = Constraint(variable, 0.0, Comparator.Le)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_var_direct(variable: Variable):
    constr = Constraint(variable, 0.0, Comparator.Ge)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(variable * 1)
    assert constr.rhs == 0.0
    assert constr.comparator == Comparator.Ge


# LHS: VARIABLE, RHS: VARIABLE


@pytest.mark.constraint
def test_constraint_creation_eq_var_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    are_equal = lhs == rhs
    assert not isinstance(are_equal, Constraint)
    assert isinstance(are_equal, bool)
    assert not are_equal


@pytest.mark.constraint
def test_constraint_creation_le_var_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    constr = lhs <= rhs
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == rhs
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_var_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    constr = lhs >= rhs
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == rhs
    assert constr.comparator == Comparator.Ge


@pytest.mark.constraint
def test_constraint_creation_eq_var_direct_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    constr = Constraint(lhs, rhs, Comparator.Eq)
    assert isinstance(constr, Constraint)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == rhs
    assert constr.comparator == Comparator.Eq


@pytest.mark.constraint
def test_constraint_creation_le_var_direct_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    constr = Constraint(lhs, rhs, Comparator.Le)
    assert isinstance(constr, Constraint)
    assert constr == (lhs <= rhs)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == rhs
    assert constr.comparator == Comparator.Le


@pytest.mark.constraint
def test_constraint_creation_ge_var_direct_rhs_var(
    variable_and_variable: tuple[Variable, Variable],
):
    lhs, rhs = variable_and_variable
    constr = Constraint(lhs, rhs, Comparator.Ge)
    assert isinstance(constr, Constraint)
    assert constr == (lhs >= rhs)
    assert constr.name is None
    assert constr.lhs.is_equal(lhs - rhs)
    assert constr.rhs == rhs
    assert constr.comparator == Comparator.Ge
