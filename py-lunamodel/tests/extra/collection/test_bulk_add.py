from luna_model.constraint.collection import ConstraintCollection
from luna_model.environment.env import Environment
from luna_model.variable.var import Variable


def test_bulk_addition():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    cc = ConstraintCollection()
    cc += [
         x + y <= 10,
         x - y >= 0,
         2 * x + 3 * y <= 20
    ]
    assert 3 == len(cc)

def test_cc_addition():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    cc = ConstraintCollection()
    cc += x + y <= 10
    cc += x - y >= 0
    cc += 2 * x + 3 * y <= 20

    cc2 = ConstraintCollection()
    cc2 += cc
    assert 3 == len(cc2)

def test_bulk_addition_mixed_named():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    cc = ConstraintCollection()
    named = (x - y >= 0, "my-constr")
    cc += [
         x + y <= 10,
         named,
         2 * x + 3 * y <= 20
    ]
    assert 3 == len(cc)
    assert named[0].equal_contents(cc[named[1]])
