from luna_model import Model, Vtype


def _build_model(var_order):
    model = Model("ordering")
    handles = {}
    for name in var_order:
        handles[name] = model.add_variable(name, vtype=Vtype.BINARY)
    x, y, z = handles["x"], handles["y"], handles["z"]

    model.objective = 2 * x + 3 * y - z
    model.add_constraint(x + y + z <= 2, name="cap")
    model.add_constraint(x - z >= 0, name="dom")
    return model


def test_equal_contents_ignores_variable_insertion_order():
    canonical = _build_model(["x", "y", "z"])
    permuted = _build_model(["y", "x", "z"])

    assert canonical.equal_contents(permuted)
    assert permuted.equal_contents(canonical)


def test_equal_contents_ignores_constraint_insertion_order():
    a = Model("constr-order-a")
    xa = a.add_variable("x", vtype=Vtype.BINARY)
    ya = a.add_variable("y", vtype=Vtype.BINARY)
    a.objective = xa + ya
    a.add_constraint(xa + ya <= 1, name="cap")
    a.add_constraint(xa - ya >= 0, name="dom")

    b = Model("constr-order-b")
    xb = b.add_variable("x", vtype=Vtype.BINARY)
    yb = b.add_variable("y", vtype=Vtype.BINARY)
    b.objective = xb + yb
    b.add_constraint(xb - yb >= 0, name="dom")
    b.add_constraint(xb + yb <= 1, name="cap")

    assert a.constraints.equal_contents(b.constraints)

    assert a.equal_contents(b)
    assert b.equal_contents(a)


def test_equal_contents_detects_real_difference_under_permuted_variables():
    canonical = _build_model(["x", "y", "z"])

    different = Model("ordering-diff")
    y = different.add_variable("y", vtype=Vtype.BINARY)
    x = different.add_variable("x", vtype=Vtype.BINARY)
    z = different.add_variable("z", vtype=Vtype.BINARY)
    different.objective = 2 * x + 3 * y - z
    different.add_constraint(x + y + z <= 3, name="cap")
    different.add_constraint(x - z >= 0, name="dom")

    assert not canonical.equal_contents(different)
    assert not different.equal_contents(canonical)
