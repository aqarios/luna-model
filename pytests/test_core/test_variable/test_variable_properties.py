from aqmodels import Environment, Variable, Vtype, Bounds


def test_variable_properties():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer, bounds=Bounds(lower=-5, upper=42))
        assert x.vtype == Vtype.Integer
        assert x.name == "x"
        assert x.bounds.lower == -5
        assert x.bounds.upper == 42
