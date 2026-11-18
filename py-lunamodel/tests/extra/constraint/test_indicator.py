import pytest

from luna_model import Bounds, Environment, Indicator, Variable, Vtype
from luna_model.errors import DifferentEnvsError, UnsupportedOperationError


def _vars():
    with Environment():
        z = Variable("z")
        x = Variable("x", vtype=Vtype.INTEGER, bounds=Bounds(0, 9))
        y = Variable("y", vtype=Vtype.INTEGER, bounds=Bounds(0, 9))
    return z, x, y


@pytest.mark.parametrize("value", [0, 1])
def test_when_and_indicator_are_inverses(value):
    z, x, y = _vars()
    constraint = x + 2 * y <= 5
    constraint.name = "cap"

    conditional = constraint.when(z, value)
    assert conditional.rhs == 0.0
    assert conditional.lhs.degree() == 2
    assert conditional.name == "cap"

    indicator = conditional.indicator()
    assert isinstance(indicator, Indicator)
    assert indicator.variable.is_equal(z)
    assert indicator.value == value
    assert indicator.constraint.equal_contents(constraint)
    assert indicator.constraint.name == "cap"


def test_indicator_unpacks():
    z, x, _ = _vars()
    variable, value, constraint = (x >= 2).when(z).indicator()
    assert variable.is_equal(z)
    assert value == 1
    assert constraint.equal_contents(x >= 2)


def test_when_is_for_the_value_one_by_default():
    z, x, _ = _vars()
    assert (x <= 3).when(z).indicator().value == 1


def test_when_takes_the_value_as_a_bool_too():
    z, x, _ = _vars()
    assert (x <= 3).when(z, value=False).indicator().value == 0
    assert (x <= 3).when(z, value=True).indicator().value == 1


def test_when_rejects_a_value_a_binary_cannot_have():
    z, x, _ = _vars()
    with pytest.raises(ValueError, match="0 or 1"):
        (x <= 3).when(z, value=2)


def test_an_inverted_variable_stands_for_the_other_value():
    z, x, _ = _vars()
    constraint = x <= 3

    indicator = constraint.when(z.inv()).indicator()
    assert indicator.variable.is_equal(z)
    assert indicator.value == 0

    indicator = constraint.when(z.inv(), value=0).indicator()
    assert indicator.variable.is_equal(z)
    assert indicator.value == 1


def test_constraints_of_that_form_are_indicators_however_written():
    z, x, _ = _vars()
    # z * x - 3 * z <= 0 is z == 1 implies x <= 3.
    indicator = (z * x - 3 * z <= 0).indicator()
    assert indicator.variable.is_equal(z)
    assert indicator.value == 1
    assert indicator.constraint.equal_contents(x <= 3)


def test_other_constraints_are_no_indicators():
    z, x, y = _vars()
    assert (x + y <= 5).indicator() is None
    # A term without the binary.
    assert (z * x + y <= 0).indicator() is None
    # A right-hand side that is not 0.
    assert (z * x <= 3).indicator() is None
    # Only fixes the binary.
    assert (z <= 0).indicator() is None


def test_when_rejects_a_variable_that_is_not_binary():
    _, x, y = _vars()
    with pytest.raises(UnsupportedOperationError):
        (x <= 3).when(y)


def test_when_rejects_a_variable_of_another_environment():
    _, x, _ = _vars()
    with Environment():
        other = Variable("z")
    with pytest.raises(DifferentEnvsError):
        (x <= 3).when(other)
