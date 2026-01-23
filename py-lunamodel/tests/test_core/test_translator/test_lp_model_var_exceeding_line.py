import base64
from pathlib import Path


from luna_model import Model
from luna_model.translator import LpTranslator


def test_model_many_vars_comma():
    m = Model()
    for i in range(10):
        for j in range(10):
            m.add_variable(f"x{i},{j}")
    _ = LpTranslator.from_lm(m)


def test_model_vars_exceeding_length():
    model_bytes_b64: str = (
        Path(__file__).parent / "variables_exceeding_line.model"
    ).read_text()
    model_bytes: bytes = base64.b64decode(model_bytes_b64)
    model = Model.decode(model_bytes)
    _ = LpTranslator.from_lm(model)
