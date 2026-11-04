import luna_model as lm
from luna_model.transformation import PassManager
from luna_model.transformation.passes import BinarySpinPass


def _names(model: lm.Model) -> list[str]:
    return [v.name for v in model.environment.variables()]


def test_decode_keeps_environment_order() -> None:
    model = lm.Model()
    a, b, c, d = (model.add_variable(name) for name in "abcd")
    model.objective = a + b + d + c
    data = model.encode()
    for _ in range(20):
        assert _names(lm.Model.decode(data)) == ["a", "b", "c", "d"]


def test_decode_keeps_reordered_environment_order() -> None:
    model = lm.Model()
    xs = [
        model.add_variable(name, vtype=vtype)
        for name, vtype in [("b1", lm.Vtype.BINARY), ("s1", lm.Vtype.SPIN), ("b2", lm.Vtype.BINARY)]
    ]
    model.objective = sum(xs)
    spin = PassManager([BinarySpinPass(lm.Vtype.SPIN)]).run(model).model
    expected = _names(spin)
    data = spin.encode()
    for _ in range(20):
        assert _names(lm.Model.decode(data)) == expected
