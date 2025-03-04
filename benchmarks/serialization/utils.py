import sys
import json

from aq_models import Model
import dimod


def serialize_aqm(model: Model) -> bytes:
    return model.serialize()


def serialize_bqm(model: dimod.BinaryQuadraticModel) -> str:
    o = model.to_serializable()
    o_json = json.dumps(o)
    return o_json


def get_serialized_size_mb(data: bytes | str) -> float:  # type: ignore
    size_bytes = sys.getsizeof(data)
    size_mb = size_bytes / (1024**2)
    return size_mb
