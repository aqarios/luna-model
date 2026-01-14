from luna_model import Model, Vtype


def test_decode():
    model = Model.decode(
        b'\x123\x08\x01\x12/(\xb5/\xfd\x00X1\x01\x00\x1a\x11\x10\x02\x1a\x01\x00"\x01\x01:\x01xB\x01s\x98\x01\x02"\x07unnamed*\x08Minimize'
    )
    # x = model.add_variable("x", vtype=Vtype.REAL)
    # y = model.add_variable("y", vtype=Vtype.REAL)
    # z = model.add_variable("z", vtype=Vtype.SPIN)
    # model.objective = x + y + z + x * y + x * y + y * z + x * y * z
    serialized_object = model.encode()
    decoded = Model.decode(serialized_object)
    assert isinstance(decoded, Model)
    assert decoded.equal_contents(model)


def test_decode_base():
    # model = Model.decode(b'\x123\x08\x01\x12/(\xb5/\xfd\x00X1\x01\x00\x1a\x11\x10\x02\x1a\x01\x00"\x01\x01:\x01xB\x01s\x98\x01\x02"\x07unnamed*\x08Minimize')
    model = Model()
    x = model.add_variable("x", vtype=Vtype.BINARY)
    s = model.add_variable("y", vtype=Vtype.SPIN)
    model.objective = 0.2586255290675049 * x + 0.2586255290675049 * s
    # z = model.add_variable("z", vtype=Vtype.SPIN)
    # model.objective = x + y + z + x * y + x * y + y * z + x * y * z
    serialized_object = model.encode()
    decoded = Model.decode(serialized_object)
    assert isinstance(decoded, Model)
    assert decoded.equal_contents(model)
