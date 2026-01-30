import base64

from luna_model import Environment, Model, Sense, Variable


def assert_encode_decode(model: Model):
    encoded_model = model.encode()
    decoded_model = Model.decode(encoded_model)
    assert decoded_model.equal_contents(model), f"\n{decoded_model=}\n{model=}"

    encoded_model_b64 = base64.encodebytes(encoded_model)
    decoded_model_bytes = base64.decodebytes(encoded_model_b64)
    decoded_model_b64 = Model.decode(decoded_model_bytes)
    assert decoded_model_b64.equal_contents(model)


def test_encode_decode_empty():
    with Environment():
        model = Model()

    assert_encode_decode(model)


def test_encode_decode_empty_max():
    with Environment():
        model = Model(sense=Sense.MAX)

    assert_encode_decode(model)


def test_encode_decode_with_objective():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        model = Model(name="objective")

    model.objective += 1
    model.objective += x
    model.objective += x * y
    model.objective += x * y * z

    assert_encode_decode(model)


def test_encode_decode_with_objective_max():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        model = Model(name="objective", sense=Sense.MAX)

    model.objective += 1
    model.objective += x
    model.objective += x * y
    model.objective += x * y * z

    assert_encode_decode(model)
