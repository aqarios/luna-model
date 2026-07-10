import pytest

from luna_model import Model, Sense
from luna_model.errors import TransformError
from luna_model.transformation import PassManager, TransformationRecord, analyze, PassContext
from luna_model.transformation.context import PassContext
from luna_model.transformation.passes import ChangeSensePass


def test_transformation_record_recovered_on_error():
    model = Model()
    xs = model.add_variables("x", 2)
    model.objective = xs.sum()

    @analyze(name="failure")
    def failure(model: Model, ctx: PassContext) -> None:
        _ = model, ctx
        raise ValueError("failure")

    pm = PassManager([ChangeSensePass(Sense.MIN), failure])

    with pytest.raises(TransformError) as excinfo:
        pm.run(model)

    err = excinfo.value
    assert err.record is not None
    assert isinstance(err.record, TransformationRecord), "upgraded to surface record"
    assert len(err.record.entries) >= 1, "all pre-failure passes captured"

def test_compilation_error_alias_deprecated_but_working():
    import luna_model.errors as errors

    with pytest.warns(FutureWarning):
        compilation_error = errors.CompilationError

    assert compilation_error is errors.TransformError
