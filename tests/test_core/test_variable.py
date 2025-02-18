import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


# @pytest.mark.variable
# def test_create_variable_default_environment():
#     _ = Variable("x")
#
#
# @pytest.mark.variable
# def test_create_variable_del_create():
#     x = Variable("x")
#     del x
#     _ = Variable("x")


@pytest.mark.variable
def test_delete_in_creation_order():
    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    del x
    del y
    del z


@pytest.mark.variable
def test_delete_unordered():
    w = Variable("w")
    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    del w
    del y
    del x
    del z


@pytest.mark.variable
def test_delete_newest_first():
    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    del z
    del y
    del x


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
    assert result.num_variables() == 1
    assert result.get_linear(x) == 1


@pytest.mark.variable
def test_add_two_variables():
    x = Variable("x")
    y = Variable("y")
    result = x + y
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
