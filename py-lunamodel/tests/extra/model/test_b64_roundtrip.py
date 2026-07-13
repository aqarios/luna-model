import base64

import pytest

from luna_model import Model, Vtype


@pytest.fixture
def model() -> Model:
    a = Model("constr-order-a")
    xa = a.add_variable("x", vtype=Vtype.BINARY)
    ya = a.add_variable("y", vtype=Vtype.BINARY)
    a.objective = xa + ya
    a.add_constraint(xa + ya <= 1, name="cap")
    a.add_constraint(xa - ya >= 0, name="dom")
    return a


def test_encode_b64_returns_tagged_ascii_str(model: Model) -> None:
    s = model.encode_b64()
    assert isinstance(s, str)
    s.encode("ascii")  # raises if non-ASCII leaked in
    assert Model.is_b64_encoded(s)


def test_roundtrip_is_lossless(model: Model) -> None:
    restored = Model.decode_b64(model.encode_b64())
    # Compare via the canonical binary codec (no reliance on Model.__eq__).
    assert restored.encode() == model.encode()


def test_is_b64_encoded_discriminates(model: Model) -> None:
    assert Model.is_b64_encoded(model.encode_b64()) is True
    # bare base64 without the prefix must not be mistaken for a payload
    assert Model.is_b64_encoded(base64.b64encode(model.encode()).decode()) is False
    # LP/MPS text and non-strings are not payloads
    assert Model.is_b64_encoded("Maximize\n obj: x + y\nEnd") is False
    assert Model.is_b64_encoded(model.encode()) is False  # bytes
    assert Model.is_b64_encoded(None) is False


def test_decode_b64_rejects_untagged_string(model: Model) -> None:
    with pytest.raises(ValueError):
        Model.decode_b64(base64.b64encode(model.encode()).decode())  # no prefix


def test_decode_b64_rejects_malformed_body() -> None:
    with pytest.raises(ValueError):
        Model.decode_b64("lunamodel:b64:v1:not*valid*base64")
