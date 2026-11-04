import pytest
from luna_model import errors

# Every kind tag of luna-model-ffi's `ErrorKind` catalog, with the class luna-model raises for it
# natively; `into_pyerr` takes it from the installed luna-model (`errors._error_for_kind`).
KIND_CLASSES = {
    "different_environments": errors.DifferentEnvsError,
    "variable_exists": errors.VariableExistsError,
    "variable_not_existing": errors.VariableNotExistingError,
    "variable_name_invalid": errors.VariableNamesError,
    "constraint_name_invalid": errors.IllegalConstraintNameError,
    "invalid_bounds": errors.VariableCreationError,
    "invalid_inversion": errors.UnsupportedOperationError,
    "inverted_binary_creation": errors.VariableCreationError,
    "compression": errors.CompressionError,
    "decoding": errors.DecodeError,
    "formatter": errors.LunaModelError,
    "unsupported_operation": errors.UnsupportedOperationError,
    "internal": errors.LunaModelError,
    "dtype": errors.SampleColCreationError,
    "computation": errors.ComputationError,
    "no_constraint_for_key": errors.NoConstraintForKeyError,
    "duplicate_constraint_name": errors.DuplicateConstraintNameError,
    "model_not_quadratic": errors.ModelNotQuadraticError,
    "model_not_unconstrained": errors.ModelNotUnconstrainedError,
    "model_sense_not_minimize": errors.ModelSenseNotMinimizeError,
    "vtype": errors.ModelVtypeError,
    "translation": errors.TranslationError,
    "evaluation": errors.EvaluationError,
    "sample_incorrect_length": errors.SampleIncorrectLengthError,
    "sample_unexpected_variable": errors.SampleUnexpectedVariableError,
    "sample_incompatible_vtype": errors.SampleIncompatibleVtypeError,
    "variable_names": errors.VariableNamesError,
    "transformation_pass": errors.TransformationPassError,
    "analysis_pass": errors.AnalysisPassError,
    "if_else_pass": errors.IfElsePassError,
    "meta_analysis_pass": errors.MetaAnalysisPassError,
    "transformation": errors.TransformError,
    "random_sampling": errors.RandomSamplingError,
    "invalid_tolerance": errors.InvalidToleranceError,
    "infeasible": errors.ModelInfeasibleError,
    "external": errors.LunaModelError,
    "myplugin::custom": errors.LunaModelError,
    "errored": errors.LunaModelError,
}


# A luna-model before 0.8.0a2 (e.g. the `pypi` test group) has no kind -> exception mapping;
# `into_pyerr` then raises the base class for every kind.
HAS_KIND_MAPPING = hasattr(errors, "_error_for_kind")


@pytest.mark.parametrize(("kind", "cls"), KIND_CLASSES.items())
def test_error_kind_raises_native_class(ext, kind, cls):
    expected = cls if HAS_KIND_MAPPING else errors.LunaModelError
    with pytest.raises(expected) as info:
        ext.raise_error_kind(kind)
    assert type(info.value) is expected
    assert str(info.value) == "boom"  # the bare message, without a status prefix
    if HAS_KIND_MAPPING:
        assert type(errors._error_for_kind(kind, "boom")) is cls


@pytest.mark.skipif(not HAS_KIND_MAPPING, reason="luna-model without the kind mapping")
def test_index_out_of_bounds_raises_index_error(ext):
    with pytest.raises(IndexError, match="^boom$"):
        ext.raise_error_kind("index_out_of_bounds")
