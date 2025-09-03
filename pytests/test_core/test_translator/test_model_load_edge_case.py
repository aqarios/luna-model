import base64
from pathlib import Path
from aqmodels import Model
from aqmodels.translator import LpTranslator

def test_model_edge_case():
    model_bytes_b64: bytes = (Path(__file__).parent / "model_bytes.model").read_bytes()
    model_bytes: bytes = base64.b64decode(model_bytes_b64)
    model_lp_str: str = (Path(__file__).parent / "model_edge.lp").read_text()
    model = Model.decode(model_bytes)
    model_lp = LpTranslator.to_aq(model_lp_str)
    model_lp_2 = LpTranslator.to_aq(LpTranslator.from_aq(model))

    assert hash(model) == hash(model_lp)
    assert hash(model) == hash(model_lp_2)

