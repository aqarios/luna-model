import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


@pytest.mark.variable
def test_create_variable_default_environment():
    _ = Variable("x")


@pytest.mark.variable
def test_create_variable_with_custom_environment():
    _ = Variable("x", env=Environment())


@pytest.mark.variable
def test_add_two_variables():
    a = Variable("a")
    b = Variable("b")

    result = a + b
    assert type(result) == Expression
