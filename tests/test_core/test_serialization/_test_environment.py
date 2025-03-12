import pytest

from aq_models import Environment
from aq_models import Variable, Vtype


@pytest.fixture
def env(request) -> Environment:
    num_variables = request.param

    env = Environment()

    with env:
        for i in range(num_variables):
            _ = Variable(name=f"b{i}", vtype=Vtype.Binary)
            _ = Variable(name=f"s{i}", vtype=Vtype.Spin)
            _ = Variable(name=f"i{i}", vtype=Vtype.Integer)
            _ = Variable(name=f"r{i}", vtype=Vtype.Real)

    return env


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_serialize(env: Environment):
    data_encoded = env.encode()
    data_serialized = env.serialize()
    assert (
        data_encoded == data_serialized
    ), ".encode and .serialize do not produce equal results"


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_serialize_compress(env: Environment):
    data_enc_compressed = env.encode(compress=True)
    data_ser_compressed = env.serialize(compress=True)

    assert (
        data_enc_compressed == data_ser_compressed
    ), ".encode and .serialize do not produce equal results when using compression"


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_deserialize_from_encode(env: Environment):
    data_encoded = env.encode()
    env_decoded_a = Environment.decode(data_encoded)
    env_deserialized_a = Environment.deserialize(data_encoded)

    assert (
        env_decoded_a == env_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data"


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_deserialize_from_encode_compressed(env: Environment):
    data_encoded = env.encode(compress=True)

    env_decoded_a = Environment.decode(data_encoded)
    env_deserialized_a = Environment.deserialize(data_encoded)

    assert (
        env_decoded_a == env_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data when compression was used"


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_deserialized_object_equal(env: Environment):
    data_encoded = env.encode()

    env_decoded_a = Environment.decode(data_encoded)
    env_deserialized_a = Environment.deserialize(data_encoded)

    assert (
        env_decoded_a == env_deserialized_a
    ), ".decode and .deserialize do not produce the same object"


@pytest.mark.parametrize("env", [1, 2, 4, 8, 16], indirect=True)
def test_enviroment_deserialized_object_equal_to_initial(env: Environment):
    env_decoded = Environment.decode(env.encode())
    assert env == env_decoded, "decoded/deserialized object not equal to input"
