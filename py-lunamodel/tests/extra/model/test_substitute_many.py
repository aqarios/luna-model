import pytest

from luna_model import Model, Vtype
from luna_model.errors import UnsupportedOperationError


def build():
    m = Model()
    a, b, c, d = (m.add_variable(n, vtype=Vtype.INTEGER) for n in "abcd")
    m.objective = a + 2 * a * b + 3 * b * b + 4 * a * c * d
    m.add_constraint(a + b + c <= 5, "c0")
    return m


def test_substitute_many_matches_substitute_per_entry():
    one, many = build(), build()
    for target, replacement in [("a", lambda v: 2 * v["c"]), ("b", lambda v: v["d"] + 1)]:
        v = {x.name: x for x in one.environment.variables()}
        one.substitute(v[target], replacement(v))
    v = {x.name: x for x in many.environment.variables()}
    many.substitute_many({v["a"]: 2 * v["c"], v["b"]: v["d"] + 1})

    assert str(many.objective) == str(one.objective)
    assert str(many.constraints) == str(one.constraints)
    assert [x.name for x in many.environment.variables()] == ["c", "d"]


def test_substitute_many_rejects_chains_unchanged():
    m = build()
    v = {x.name: x for x in m.environment.variables()}
    before = str(m)
    with pytest.raises(UnsupportedOperationError):
        m.substitute_many({v["a"]: v["b"], v["b"]: v["c"]})
    assert str(m) == before
