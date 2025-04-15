import json
import sys

from aqmodels import Model
import dimod


def serialize_aqm(model: Model) -> bytes:
    return model.serialize(compress=True)


def serialize_bqm(model: dimod.BinaryQuadraticModel, use_bytes: bool = False) -> str:
    o = model.to_serializable(use_bytes=use_bytes)
    o_json = json.dumps(o)
    return o_json


def serialize_cqm(model: dimod.ConstrainedQuadraticModel) -> bytes:
    o = model.to_file().read()
    return o


def get_serialized_size_mb(data: bytes | str) -> float:  # type: ignore
    size_bytes = sys.getsizeof(data)
    size_mb = size_bytes / (1024**2)
    return size_mb


def get_serialized_size_bytes(data: bytes | str) -> float:  # type: ignore
    # size_bytes = sys.getsizeof(data)
    # return size_bytes
    return len(data)
