import copy
import pickle

import pytest

from luna_model import Model
from luna_model.errors import VariableNotExistingError


@pytest.fixture()
def substituted():
    """Model after `substitute(a, 1 - b)`, plus expressions built before it."""
    m = Model()
    a = m.add_variable("a")
    b = m.add_variable("b")
    kept = 5 * a + b
    kept_constraint = a + b <= 1
    m.objective = a + b
    m.substitute(a, 1 - b)
    return m, a, b, kept, kept_constraint


def test_substitute_removes_target(substituted):
    m, *_ = substituted
    assert [v.name for v in m.environment.variables()] == ["b"]


def test_expression_substitute_keeps_target():
    m = Model()
    a = m.add_variable("a")
    b = m.add_variable("b")
    (5 * a + b).substitute(a, 1 - b)
    assert [v.name for v in m.environment.variables()] == ["a", "b"]


# Kept expressions no longer panic


def test_str_of_kept_expression(substituted):
    *_, kept, kept_constraint = substituted
    assert str(kept) == "5 <removed variable 0> + b"
    assert str(kept_constraint) == "<removed variable 0> + b <= 1"


def test_kept_expression_is_usable_outside_the_model(substituted):
    _, _, b, kept, _ = substituted
    assert str(kept + b) == "5 <removed variable 0> + 2 b"
    assert len(kept.variables()) == 2
    assert str(copy.deepcopy(kept)) == str(kept)
    assert str(pickle.loads(pickle.dumps(kept))) == str(kept)
    assert kept.equal_contents(kept) is False


# Kept expressions cannot enter the model


def test_assign_kept_objective_raises(substituted):
    m, _, _, kept, _ = substituted
    with pytest.raises(VariableNotExistingError, match="removed from its environment"):
        m.objective = kept


def test_iadd_kept_objective_raises(substituted):
    m, _, _, kept, _ = substituted
    with pytest.raises(VariableNotExistingError):
        m.objective += kept


def test_add_kept_constraint_raises(substituted):
    m, _, _, kept, _ = substituted
    with pytest.raises(VariableNotExistingError):
        m.add_constraint(kept <= 1)


def test_iadd_kept_constraint_raises(substituted):
    m, *_, kept_constraint = substituted
    with pytest.raises(VariableNotExistingError):
        m.constraints += kept_constraint


def test_substitute_removed_target_raises(substituted):
    m, a, b, *_ = substituted
    with pytest.raises(VariableNotExistingError):
        m.substitute(a, 1 * b)


def test_model_unchanged_after_rejection(substituted):
    m, _, _, kept, _ = substituted
    before = str(m)
    with pytest.raises(VariableNotExistingError):
        m.objective = kept
    with pytest.raises(VariableNotExistingError):
        m.add_constraint(kept <= 1)
    assert str(m) == before
