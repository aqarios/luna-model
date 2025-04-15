import pytest

from aqmodels import Environment, Expression, Variable


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3])
def test_sub_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = x - scalar
    assert isinstance(result, Expression)
    assert result.num_variables() == 1
    assert result.get_linear(x) == 1
    assert result.get_offset() == -scalar


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3])
def test_rsub_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = scalar - x
    assert isinstance(result, Expression)
    assert result.num_variables() == 1
    assert result.get_linear(x) == -1
    assert result.get_offset() == scalar


@pytest.mark.variable
def test_sub_two_variables():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = x - y
    assert isinstance(result, Expression)
    assert result.num_variables() == 2
    assert result.get_offset() == 0
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == -1


@pytest.mark.variable
def test_sub_two_variables_unordered():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = y - x
    assert isinstance(result, Expression)
    assert result.num_variables() == 2
    assert result.get_offset() == 0
    assert result.get_linear(x) == -1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_sub_last_two_variables():
    with Environment():
        _ = Variable("x_ignore")
        _ = Variable("y_ignore")
        x = Variable("x")
        y = Variable("y")

    result = y - x
    assert isinstance(result, Expression)
    assert result.num_variables() == 2
    assert result.get_linear(x) == -1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_sub_any_and_last_variables():
    with Environment():
        _ = Variable("x_ignore")
        x = Variable("x")
        _ = Variable("y_ignore")
        y = Variable("y")

    result = y - x
    assert isinstance(result, Expression)
    assert result.num_variables() == 2
    assert result.get_offset() == 0
    assert result.get_linear(x) == -1
    assert result.get_linear(y) == 1


# @pytest.mark.variable
# def test_variable_sub_expression():
#     with Environment():
#         x, y, z = Variable("x"), Variable("y"), Variable("z")
#     expr = x - y
#     assert isinstance(expr, Expression)
#     assert expr.num_variables() == 2
#     assert expr.get_offset() == 0
#     assert expr.get_linear(x) == 1
#     assert expr.get_linear(y) == -1
#
#     result = z - expr
#     assert isinstance(result, Expression)
#     assert result.num_variables() == 3
#     assert result.get_offset() == 0
#     assert result.get_linear(x) == 1
#     assert result.get_linear(y) == 1
#     assert result.get_linear(z) == 1


# @pytest.mark.variable
# def test_variable_rsub_expression():
#     with Environment():
#         x, y, z = Variable("x"), Variable("y"), Variable("z")
#     expr = x - y
#     assert isinstance(expr, Expression)
#     assert expr.num_variables() == 2
#     assert expr.get_offset() == 0
#     assert expr.get_linear(x) == 1
#     assert expr.get_linear(y) == 1
#
#     result = expr - z
#     assert isinstance(result, Expression)
#     assert result.num_variables() == 3
#     assert result.get_offset() == 0
#     assert result.get_linear(x) == 1
#     assert result.get_linear(y) == 1
#     assert result.get_linear(z) == 1
