import numpy as np
import pytest

from luna_model import Model, Solution
from luna_model.errors import SampleIncorrectLengthError


@pytest.fixture
def model():
    m = Model()
    x = m.add_variable("x")
    y = m.add_variable("y")
    m.objective = x
    return m, x, y


# --- from_arrays -------------------------------------------------------------


def test_from_arrays_too_many_columns_raises(model):
    m, _, _ = model
    with pytest.raises(SampleIncorrectLengthError):
        Solution.from_arrays(np.array([[1.0, 0, 1]]), model=m)


def test_from_arrays_too_few_columns_raises(model):
    m, _, _ = model
    with pytest.raises(SampleIncorrectLengthError):
        Solution.from_arrays(np.array([[1.0]]), model=m)


def test_from_arrays_columns_must_match_variables(model):
    m, x, _ = model
    with pytest.raises(SampleIncorrectLengthError):
        Solution.from_arrays(np.array([[1.0, 0]]), variables=[x], model=m)


def test_from_arrays_partial_with_variables(model):
    m, x, _ = model
    sol = Solution.from_arrays(np.array([[1.0], [0.0]]), variables=[x], model=m)
    assert sol.variable_names == ["x"]
    assert sol.obj_values.tolist() == [1.0, 0.0]


def test_from_arrays_partial_matches_from_dicts(model):
    m, x, _ = model
    a = Solution.from_arrays(np.array([[1.0]]), variables=[x], env=m.environment)
    d = Solution.from_dicts([{"x": 1}], env=m.environment)
    assert a.variable_names == d.variable_names
    assert a.samples.tolist() == d.samples.tolist()


@pytest.mark.parametrize("kw", ["counts", "energies"])
@pytest.mark.parametrize("n", [1, 3])
def test_from_arrays_per_sample_lengths(model, kw, n):
    m, _, _ = model
    values = [1] * n if kw == "counts" else [1.0] * n
    with pytest.raises(SampleIncorrectLengthError, match=kw):
        Solution.from_arrays(np.array([[1.0, 0], [0, 0]]), model=m, **{kw: values})


# --- from_dicts / from_dict --------------------------------------------------


@pytest.mark.parametrize("kw", ["counts", "energies"])
@pytest.mark.parametrize("n", [1, 3])
def test_from_dicts_per_sample_lengths(model, kw, n):
    m, _, _ = model
    values = [1] * n if kw == "counts" else [1.0] * n
    with pytest.raises(SampleIncorrectLengthError, match=kw):
        Solution.from_dicts([{"x": 1, "y": 0}, {"x": 0, "y": 0}], model=m, **{kw: values})


def test_from_dict_keeps_energy(model):
    m, _, _ = model
    sol = Solution.from_dict({"x": 1, "y": 0}, env=m.environment, energy=3.0)
    assert sol.raw_energies.tolist() == [3.0]


# --- from_counts -------------------------------------------------------------


@pytest.mark.parametrize("n", [1, 3])
def test_from_counts_energies_length(model, n):
    m, _, _ = model
    with pytest.raises(SampleIncorrectLengthError, match="energies"):
        Solution.from_counts({"10": 2, "01": 1}, env=m.environment, energies=[1.0] * n)


# --- Solution(...) -----------------------------------------------------------


def test_constructor_partial_with_env(model):
    m, _, _ = model
    sol = Solution([{"x": 1}], env=m.environment)
    assert sol.variable_names == ["x"]
    assert sol.samples.tolist() == [[1]]


def test_constructor_feasible_length(model):
    m, _, _ = model
    with pytest.raises(ValueError, match="feasible"):
        Solution([{"x": 1, "y": 0}, {"x": 0, "y": 0}], env=m.environment, feasible=[True])
