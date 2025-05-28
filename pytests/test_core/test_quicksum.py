from aqmodels import quicksum, Environment, Variable, Expression


def test_quicksum():
    with Environment():
        variables = [Variable(str(i)) for i in range(20)]
        expr = quicksum(variables)
    assert isinstance(expr, Expression)
