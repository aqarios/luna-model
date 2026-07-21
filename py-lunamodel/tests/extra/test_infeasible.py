import pytest

from luna_model import Model
from luna_model.transformation import PassManager, TransformationRecord
from luna_model.transformation.passes import CheckInfeasibleConstraintsAnalysis
from luna_model.errors import ModelInfeasibleError

def _run(model: Model):
    pm = PassManager([CheckInfeasibleConstraintsAnalysis()])
    pm.run(model)

def test_le_infeasible():
    m = Model()
    vs = m.add_variables("x", 2)
    m.add_constraint(vs.sum() <= -1.0) # range [0,2] cannot be <= -1
    with pytest.raises(ModelInfeasibleError) as excinfo:
        _run(m)

    err = excinfo.value
    assert err.record is not None
    assert isinstance(err.record, TransformationRecord), "upgraded to surface record"

def test_ge_infeasible():
    m = Model()
    vs = m.add_variables("x", 2)
    m.add_constraint(vs.sum() >= 3.0) # range [0,2] cannot be >= 3
    with pytest.raises(ModelInfeasibleError) as excinfo:
        _run(m)

    err = excinfo.value
    assert err.record is not None
    assert isinstance(err.record, TransformationRecord), "upgraded to surface record"

def test_eq_infeasible():
    m = Model()
    vs = m.add_variables("x", 2)
    m.add_constraint(vs.sum() == 2.2) # range [0,2] does not contain 2.2
    with pytest.raises(ModelInfeasibleError) as excinfo:
        _run(m)

    err = excinfo.value
    assert err.record is not None
    assert isinstance(err.record, TransformationRecord), "upgraded to surface record"
