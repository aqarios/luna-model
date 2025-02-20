import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3])
def test_mul_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = x * scalar
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_linear(x) == scalar
