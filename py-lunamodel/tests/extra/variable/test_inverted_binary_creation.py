import pickle

import pytest

from luna_model import Environment, Model, Variable, Vtype
from luna_model.errors import VariableCreationError


def test_add_variable_rejects_inverted_binary():
    m = Model()
    with pytest.raises(VariableCreationError, match="invert a binary variable"):
        m.add_variable("x", vtype=Vtype.INVERTED_BINARY)
    assert len(m.environment.variables()) == 0


def test_add_variables_rejects_inverted_binary():
    m = Model()
    with pytest.raises(VariableCreationError):
        m.add_variables("x", 3, vtype=Vtype.INVERTED_BINARY)
    assert len(m.environment.variables()) == 0


def test_variable_constructor_rejects_inverted_binary():
    with Environment(), pytest.raises(VariableCreationError):
        Variable("x", vtype=Vtype.INVERTED_BINARY)


def test_add_variables_reports_errors_instead_of_panicking():
    m = Model()
    m.add_variables("x", (2, 2))
    with pytest.raises(Exception, match="x0,0") as e:
        m.add_variables("x", (2, 2))
    assert "panic" not in str(e.value)


# Inverting a binary variable still works and the type stays readable


def test_inverted_binary_from_binary():
    m = Model()
    b = m.add_variable("b")
    nb = ~b
    assert nb.vtype == Vtype.INVERTED_BINARY
    assert (~nb).name == "b"
    assert b.inv().name == nb.name


def test_inverted_binary_survives_round_trip():
    m = Model()
    b = m.add_variable("b")
    m.objective = b + ~b
    assert str(pickle.loads(pickle.dumps(m)).objective) == "b + ~b"
    assert str(Model.decode(m.encode()).objective) == "b + ~b"
