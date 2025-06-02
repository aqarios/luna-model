import pytest
from aqmodels import quicksum, Environment, Variable, Expression


N: int = 20


@pytest.fixture
def variables() -> list[Variable]:
    with Environment():
        return [Variable(f"v0{i}") for i in range(N)]


def make_expression(variables: list[Variable]) -> Expression:
    env = variables[0]._environment
    with env:
        expr = Expression()
        for var in variables:
            expr += var
    return expr


def truth(variables: list[Variable]) -> Expression:
    return make_expression(variables)


def truth_expr(variables: list[Variable]) -> Expression:
    base = Expression(variables[0]._environment)
    for _ in range(N):
        base += make_expression(variables)
    return base


def test_quicksum(variables: list[Variable]):
    t: Expression = truth(variables)
    with t._environment:
        expr = quicksum(variables)
    assert isinstance(expr, Expression)
    assert expr.is_equal(t)


def test_quicksum_iterable_exprs(variables: list[Variable]):
    te: Expression = truth_expr(variables)
    expr = quicksum(make_expression(variables) for _ in range(N))
    assert isinstance(expr, Expression)
    assert expr.is_equal(te)


def test_quicksum_iterable_nums():
    with pytest.raises(TypeError):
        _ = quicksum(i for i in range(N))

def test_quicksum_iterable_start_variable():
    with pytest.raises(TypeError):
        _ = quicksum((i for i in range(N)), start=Variable("x", env=Environment())) # type: ignore
