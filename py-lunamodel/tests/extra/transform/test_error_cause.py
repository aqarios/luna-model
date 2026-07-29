import pytest

from luna_model import Model
from luna_model.transformation import NothingArtifact, PassContext, PassManager, analyze, transform
from luna_model.errors import LunaModelError


def model() -> Model:
    m = Model()
    x = m.add_variables("x", 2)
    m.objective = (2 * x).sum()
    return m


def test_analysis_pass_python_exception_preserved_as_cause():
    @analyze(name="boom", provides="boom", requires=[])
    def boom_analysis(model: Model, ctx: PassContext) -> int:
        _ = model, ctx
        raise ValueError("boom from python analysis")

    with pytest.raises(LunaModelError) as exc_info:
        PassManager([boom_analysis]).run(model())

    cause = exc_info.value.__cause__
    assert cause is not None
    assert isinstance(cause, ValueError)
    assert "boom from python analysis" in str(cause)


def test_transformation_pass_python_exception_preserved_as_cause():
    @transform(name="boom", requires=[], invalidates=[])
    def boom_transform(model: Model, ctx: PassContext) -> tuple[Model, NothingArtifact]:
        _ = ctx
        raise ValueError("boom from python transform")

    with pytest.raises(LunaModelError) as exc_info:
        PassManager([boom_transform]).run(model())

    cause = exc_info.value.__cause__
    assert cause is not None
    assert isinstance(cause, ValueError)
    assert "boom from python transform" in str(cause)
