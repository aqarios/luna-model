import luna_model as lm
from luna_model.transformation import PassManager
from luna_model.transformation.passes import BinarySpinPass

VARS = [
    ("b1", lm.Vtype.BINARY),
    ("b2", lm.Vtype.BINARY),
    ("b3", lm.Vtype.BINARY),
    ("s1", lm.Vtype.SPIN),
    ("r1", lm.Vtype.REAL),
    ("i1", lm.Vtype.INTEGER),
    ("b4", lm.Vtype.BINARY),
    ("s2", lm.Vtype.SPIN),
    ("b5", lm.Vtype.BINARY),
]


def _model():
    model = lm.Model()
    xs = [model.add_variable(name, vtype=vtype, upper=3 if vtype == lm.Vtype.INTEGER else None) for name, vtype in VARS]
    model.objective = sum((i + 1) * x for i, x in enumerate(xs)) + xs[0] * xs[1]
    return model


def _spin(model):
    return PassManager([BinarySpinPass(lm.Vtype.SPIN)]).run(model)


def test_binary_spin_keeps_variable_positions():
    expected = ["s_b1", "s_b2", "s_b3", "s1", "r1", "i1", "s_b4", "s2", "s_b5"]
    model = _spin(_model()).model
    assert [v.name for v in model.environment.variables()] == expected


def test_binary_spin_order_is_stable_across_runs():
    model = _model()
    env_orders = set()
    model_orders = set()
    for _ in range(20):
        spin = _spin(model).model
        env_orders.add(tuple(v.name for v in spin.environment.variables()))
        model_orders.add(tuple(v.name for v in spin.variables()))
    assert len(env_orders) == 1
    assert len(model_orders) == 1


def test_binary_spin_backward_keeps_column_positions():
    ir = _spin(_model())
    sample = {v.name: (1 if v.vtype == lm.Vtype.SPIN else 0) for v in ir.model.environment.variables()}
    sol = lm.Solution.from_dicts([sample], env=ir.model.environment)
    back = ir.record.backward(sol)
    assert back.variable_names == [name for name, _ in VARS]
