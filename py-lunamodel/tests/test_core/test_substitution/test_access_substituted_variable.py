import pytest

from luna_model import Model, Variable, Vtype
from luna_model.errors import VariableNotExistingError


@pytest.fixture()
def base() -> tuple[Model, Variable, Variable]:
    model = Model()
    x = model.add_variable("x")
    s = model.add_variable("s", vtype=Vtype.SPIN)
    model.objective = x - s
    rep = model.add_variable("b")
    model.substitute(s, rep)
    return model, x, s


def test_print_s(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        print(s)


def test_print_s_name(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        print(s.name)


def test_print_s_vtype(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        print(s.vtype)


def test_print_s_bounds(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        print(s.bounds)


def test_s_add(base: tuple[Model, Variable, Variable]):
    _, x, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s + x


def test_s_radd(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = 2 + s


def test_s_sub(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s - 3


def test_s_rsub(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = 5 - s


def test_s_mul(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s * 3


def test_s_mul_var(base: tuple[Model, Variable, Variable]):
    _, x, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s * x


def test_s_rmul(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = 5 * s


def test_s_pow(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s**3


def test_s_eq(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s == 3


def test_s_le(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s <= 3


def test_s_ge(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s >= 3


def test_s_neg(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = -s


def test_s_env(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = s.environment


def test_s_prep(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = repr(s)


def test_s_str(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = str(s)


def test_s_hash(base: tuple[Model, Variable, Variable]):
    _, _, s = base
    with pytest.raises(VariableNotExistingError):
        _ = hash(s)
