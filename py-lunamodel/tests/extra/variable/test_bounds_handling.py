import math

import pytest

from luna_model import Bounds, Environment, Model, Solution, Unbounded, Variable, Vtype
from luna_model.errors import TransformationError, VariableCreationError
from luna_model.transformation import PassManager, passes
from luna_model.translator import LpTranslator, MpsTranslator


@pytest.mark.parametrize("vtype", [Vtype.REAL, Vtype.INTEGER])
@pytest.mark.parametrize("side", ["lower", "upper"])
def test_nan_bound_is_rejected(vtype, side):
    m = Model()
    with pytest.raises(VariableCreationError, match=f"{side} bound cannot be NaN"):
        m.add_variable("x", vtype=vtype, **{side: math.nan})
    assert len(m.environment.variables()) == 0


def test_nan_bound_is_rejected_for_variable_constructor():
    with Environment(), pytest.raises(VariableCreationError, match="NaN"):
        Variable("x", vtype=Vtype.REAL, bounds=Bounds(upper=math.nan))


@pytest.mark.parametrize("vtype", [Vtype.REAL, Vtype.INTEGER])
def test_outward_infinity_is_unbounded(vtype):
    x = Model().add_variable("x", vtype=vtype, lower=-math.inf, upper=math.inf)
    assert x.bounds.lower is Unbounded
    assert x.bounds.upper is Unbounded


def test_inward_infinity_is_kept_as_value():
    x = Model().add_variable("x", vtype=Vtype.REAL, lower=math.inf, upper=-math.inf)
    assert x.bounds.lower == math.inf
    assert x.bounds.upper == -math.inf


def test_lower_above_upper_is_accepted():
    x = Model().add_variable("x", vtype=Vtype.REAL, lower=5, upper=1)
    assert (x.bounds.lower, x.bounds.upper) == (5, 1)


def test_fractional_integer_bounds_are_accepted():
    x = Model().add_variable("x", vtype=Vtype.INTEGER, lower=0.5, upper=0.29 * 100)
    assert (x.bounds.lower, x.bounds.upper) == (0.5, 0.29 * 100)


def test_missing_side_uses_default():
    x = Model().add_variable("x", vtype=Vtype.INTEGER, upper=-3)
    assert (x.bounds.lower, x.bounds.upper) == (0, -3)


def _int_to_bin(lower, upper):
    m = Model()
    x = m.add_variable("x", vtype=Vtype.INTEGER, lower=lower, upper=upper)
    m.objective = x
    return PassManager([passes.IntegerToBinaryPass()]).run(m)


def test_integer_to_binary_rounds_fractional_bounds_inward():
    # 0.5 <= x <= 3.7 allows 1, 2, 3.
    ir = _int_to_bin(0.5, 3.7)
    b0, b1 = ir.model.variables()
    assert (b0 + b1 + 1).equal_contents(ir.model.objective)


def test_integer_to_binary_snaps_float_noise():
    # 0.29 * 100 == 28.999999999999996 is meant as 29.
    assert _int_to_bin(0, 0.29 * 100).model.objective.equal_contents(_int_to_bin(0, 29).model.objective)


def test_integer_to_binary_single_value():
    ir = _int_to_bin(3, 3)
    assert len(ir.model.variables()) == 0
    assert ir.model.objective.get_offset() == 3


def test_integer_to_binary_without_integer_value_raises():
    with pytest.raises(TransformationError, match="no integer value"):
        _int_to_bin(0.2, 0.8)


def test_random_integer_samples_stay_within_fractional_bounds():
    m = Model()
    m.add_variable("x", vtype=Vtype.INTEGER, lower=0.5, upper=3.7)
    sol = Solution.from_random(200, seed=1, model=m)
    assert {v for row in sol.samples.tolist() for v in row} == {1, 2, 3}


def test_lp_reader_accepts_fractional_integer_bounds():
    lp = """Minimize
 obj: x
Subject To
 c0: x >= 0
Bounds
 0.5 <= x <= 3.7
Generals
 x
End
"""
    x = LpTranslator.to_lm(lp).environment.get_variable("x")
    assert (x.bounds.lower, x.bounds.upper) == (0.5, 3.7)


def test_mps_roundtrip_keeps_fractional_integer_bounds(tmp_path):
    m = Model()
    x = m.add_variable("x", vtype=Vtype.INTEGER, lower=0.5, upper=3.7)
    m.objective = x
    path = tmp_path / "model.mps"
    MpsTranslator.from_lm(m, filepath=path)
    y = MpsTranslator.to_lm(path).environment.get_variable("x")
    assert (y.bounds.lower, y.bounds.upper) == (0.5, 3.7)
