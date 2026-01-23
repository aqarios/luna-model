from luna_model import Bounds, Environment, Variable, Vtype


def test_variable_properties():
    with Environment():
        x = Variable("x", vtype=Vtype.INTEGER, bounds=Bounds(lower=-5, upper=42))
        assert x.vtype == Vtype.INTEGER
        assert x.name == "x"
        assert x.bounds.lower == -5
        assert x.bounds.upper == 42
