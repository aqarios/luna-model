from luna_model.constraint.collection import ConstraintCollection
from luna_model.environment.env import Environment
from luna_model.variable.var import Variable


def test_cc_contains():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    cc = ConstraintCollection()
    cc += x + y <= 10, "first_constraint"
    cc += x - y >= 0
    cc += 2 * x + 3 * y <= 20, "another"

    assert "first_constraint" in cc
    assert "c1" in cc
    assert "another" in cc

