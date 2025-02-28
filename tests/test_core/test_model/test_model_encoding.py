import pytest
import base64

from aq_models import Model
from aq_models import Environment


@pytest.mark.model
def test_encode_decode_empty():
    with Environment():
        model = Model()

    encoded_model = model.encode()
    decoded_model = Model.decode(encoded_model)
    assert decoded_model == model

    encoded_model_b64 = base64.encodebytes(encoded_model)
    decoded_model_bytes = base64.decodebytes(encoded_model_b64)
    decoded_model_b64 = Model.decode(decoded_model_bytes)
    assert decoded_model_b64 == model
