import base64
from pathlib import Path

from luna_model import Model
from luna_model.translator import LpTranslator


def test_model_edge_case():
    model_bytes_b64: bytes = (Path(__file__).parent / "model_bytes.model").read_bytes()
    model_bytes: bytes = base64.b64decode(model_bytes_b64)
    model_lp_str: str = (Path(__file__).parent / "model_edge.lp").read_text()
    model = Model.decode(model_bytes)
    model_lp = LpTranslator.to_lm(model_lp_str)

    # print()
    # print("MODEL FROM BYTES")
    # print(model)
    # print("MODEL FROM LP")
    # print(model_lp)

    string = LpTranslator.from_lm(model)
    assert string is not None
    model_lp_2 = LpTranslator.to_lm(string)

    assert model.equal_contents(model_lp)
    assert model.equal_contents(model_lp_2)
    # assert hash(model) == hash(model_lp)
    # assert hash(model) == hash(model_lp_2)
