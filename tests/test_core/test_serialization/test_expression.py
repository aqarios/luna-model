import pytest
from itertools import product

import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray

from aq_models import MatrixTranslator
from aq_models import Expression


@pytest.fixture
def qubo(request) -> NDArray:
    size, density = request.param
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_serialize(qubo: NDArray):
    expr = MatrixTranslator.to_model(qubo).objective

    data_encoded = expr.encode()
    data_serialized = expr.serialize()

    assert (
        data_encoded == data_serialized
    ), ".encode and .serialize do not produce equal results"


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_serialize_compress(qubo: NDArray):
    expr = MatrixTranslator.to_model(qubo).objective

    data_enc_compressed = expr.encode(compress=True)
    data_ser_compressed = expr.serialize(compress=True)

    assert (
        data_enc_compressed == data_ser_compressed
    ), ".encode and .serialize do not produce equal results when using compression"


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_deserialize_from_encode(qubo: NDArray):
    data_encoded = MatrixTranslator.to_model(qubo).objective.encode()

    expr_decoded_a = Expression.decode(data_encoded)
    expr_deserialized_a = Expression.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data"


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_deserialize_from_encode_compressed(qubo: NDArray):
    data_encoded = MatrixTranslator.to_model(qubo).objective.encode(compress=True)

    expr_decoded_a = Expression.decode(data_encoded)
    expr_deserialized_a = Expression.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data when compression was used"


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_deserialized_object_equal(qubo: NDArray):
    expr = MatrixTranslator.to_model(qubo).objective
    data_encoded = expr.encode()

    expr_decoded_a = Expression.decode(data_encoded)
    expr_deserialized_a = Expression.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce the same object"


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800, 1000, 2000], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_expression_deserialized_object_equal_to_initial(qubo: NDArray):
    expr = MatrixTranslator.to_model(qubo).objective
    expr_decoded = Expression.decode(expr.encode())

    assert expr == expr_decoded, "decoded/deserialized object not equal to input"


# def test_serialize_expression():
#     Request = namedtuple("Request", ["param"])
#
#     for size in [100, 200, 400, 600, 800, 1000, 2000]:
#         for d in [0.1, 0.5, 1.0]:
#             q = qubo(Request((size, d)))
#             expr = MatrixTranslator.to_model(q).objective
#
#             data = expr.serialize()
#             data_compressed = expr.serialize(compress=True)
#             data_compressed_1 = expr.serialize(level=1)
#             data_compressed_2 = expr.serialize(level=2)
#             data_compressed_3 = expr.serialize(level=3)
#             data_old = expr.serialize_old()
#
#             expr_decoded = Expression.deserialize(data)
#
#             print(f"Size {size} -- Density {d}")
#             print(f"Uncompressed   = {len(data) * 1024**-2}")
#             print(f"Compressed     = {len(data_compressed) * 1024**-2}")
#             print(f"Compressed (1) = {len(data_compressed_1) * 1024**-2}")
#             print(f"Compressed (2) = {len(data_compressed_2) * 1024**-2}")
#             print(f"Compressed (3) = {len(data_compressed_3) * 1024**-2}")
#             print(f"Old            = {len(data_old) * 1024**-2}")
#
#             # print(repr(expr))
#             # print(repr(expr_decoded))
#
#             assert expr == expr_decoded
