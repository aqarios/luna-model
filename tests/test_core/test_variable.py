import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


@pytest.mark.variable
def test_create_variable_default_environment():
    _ = Variable("x")


@pytest.mark.variable
def test_create_variable_del_create():
    x = Variable("x")
    del x
    _ = Variable("x")


@pytest.mark.variable
def test_create_variable_with_custom_environment():
    _ = Variable("x", env=Environment())


@pytest.mark.variable
def test_create_variable_with_same_name_different_evironment():
    _ = Variable("x")
    _ = Variable("x", env=Environment())


@pytest.mark.variable
def test_add_variable_and_float():
    x = Variable("x")
    result = x + 1
    assert type(result) == Expression


@pytest.mark.variable
def test_add_two_variables():
    a = Variable("a")
    b = Variable("b")

    result = a + b
    assert type(result) == Expression
