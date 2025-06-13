from aqmodels import Model, Variable, Vtype


def test_iadd():
    model = Model()
    with model.environment:
        x = Variable("x")
        y = Variable("y", vtype=Vtype.Real)
        z = Variable("z", vtype=Vtype.Integer)

    model.objective = 2 * x
    model.objective += y**2
    model.objective += z**3

    assert model.objective.is_equal(2 * x + y**2 + z**3)


def test_iadd_with_passed_objective():
    model = Model()
    with model.environment:
        x = Variable("x")
        y = Variable("y", vtype=Vtype.Real)
        z = Variable("z", vtype=Vtype.Integer)

    model.objective = 2 * x

    def add_rest(objective):
        objective += y**2
        objective += z**3

    add_rest(model.objective)

    assert model.objective.is_equal(2 * x + y**2 + z**3)
