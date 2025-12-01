import pytest

LM_MODULE_NAME: str = "luna_model._core"
TRANSLATOR_MODULE_NAME: str = "luna_model._core.translator"
ERRORS_MODULE_NAME: str = "luna_model._core.errors"


@pytest.mark.imports()
def test_import_luna_model():
    import luna_model as _  # noqa: F401


@pytest.mark.imports()
def test_import_variable():
    import luna_model as lm
    from luna_model import Variable

    assert lm.Variable == Variable
    assert lm.Variable.__module__ == Variable.__module__
    assert lm.Variable.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_vtype():
    import luna_model as lm
    from luna_model import Vtype

    assert lm.Vtype == Vtype
    assert lm.Vtype.__module__ == Vtype.__module__
    assert lm.Vtype.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_comparator():
    import luna_model as lm
    from luna_model import Comparator

    assert lm.Comparator == Comparator
    assert lm.Comparator.__module__ == Comparator.__module__
    assert lm.Comparator.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_environment():
    import luna_model as lm
    from luna_model import Environment

    assert lm.Environment == Environment
    assert lm.Environment.__module__ == Environment.__module__
    assert lm.Environment.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_expression():
    import luna_model as lm
    from luna_model import Expression

    assert lm.Expression == Expression
    assert lm.Expression.__module__ == Expression.__module__
    assert lm.Expression.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_model():
    import luna_model as lm
    from luna_model import Model

    assert lm.Model == Model
    assert lm.Model.__module__ == Model.__module__
    assert lm.Model.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_bounds():
    import luna_model as lm
    from luna_model import Bounds

    assert lm.Bounds == Bounds
    assert lm.Bounds.__module__ == Bounds.__module__
    assert lm.Bounds.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_constraint():
    import luna_model as lm
    from luna_model import Constraint

    assert lm.Constraint == Constraint
    assert lm.Constraint.__module__ == Constraint.__module__
    assert lm.Constraint.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_constraints():
    import luna_model as lm
    from luna_model import ConstraintCollection

    assert lm.ConstraintCollection == ConstraintCollection
    assert lm.ConstraintCollection.__module__ == ConstraintCollection.__module__
    assert lm.ConstraintCollection.__module__ == LM_MODULE_NAME


@pytest.mark.imports()
def test_import_qubo_translator():
    import luna_model
    import luna_model as lm
    from luna_model import translator  # the true way to do it.
    from luna_model.translator import QuboTranslator as MTID

    # The true path is the translator.QuboTranslator
    # The luna_model/lm.translator is hacked into the bindings.
    # So we check again the translator.QuboTranslator with the import from
    # luna_model
    assert luna_model.translator.QuboTranslator == translator.QuboTranslator
    assert lm.translator.QuboTranslator == translator.QuboTranslator
    assert translator.QuboTranslator == MTID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.translator.QuboTranslator.__module__ == TRANSLATOR_MODULE_NAME
    assert luna_model.translator.QuboTranslator == translator.QuboTranslator
    assert MTID.__module__ == TRANSLATOR_MODULE_NAME
    assert translator.QuboTranslator.__module__ == TRANSLATOR_MODULE_NAME


@pytest.mark.imports()
def test_import_variable_out_of_range_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import VariableOutOfRangeError as ID

    # The true path is the errors.VariableOutOfRangeError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.VariableOutOfRangeError with the import from
    # luna_model
    assert luna_model.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert lm.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert errors.VariableOutOfRangeError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME
    assert luna_model.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_variable_exists_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import VariableExistsError as ID

    # The true path is the errors.VariableExistsError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.VariableExistsError with the import from
    # luna_model
    assert luna_model.errors.VariableExistsError == errors.VariableExistsError
    assert lm.errors.VariableExistsError == errors.VariableExistsError
    assert errors.VariableExistsError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.VariableExistsError.__module__ == ERRORS_MODULE_NAME
    assert luna_model.errors.VariableExistsError == errors.VariableExistsError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.VariableExistsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_variable_from_different_envs_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import VariablesFromDifferentEnvsError as ID

    # The true path is the errors.VariablesFromDifferentEnvsError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.VariablesFromDifferentEnvsError with the import from
    # luna_model
    assert (
        luna_model.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert (
        lm.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert errors.VariablesFromDifferentEnvsError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert (
        luna_model.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_different_envs_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import DifferentEnvsError as ID

    # The true path is the errors.DifferentEnvsError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.DifferentEnvsError with the import from
    # luna_model
    assert luna_model.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert lm.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert errors.DifferentEnvsError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert luna_model.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_no_active_environment_found_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import NoActiveEnvironmentFoundError as ID

    # The true path is the errors.NoActiveEnvironmentFoundError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.NoActiveEnvironmentFoundError with the import from
    # luna_model
    assert (
        luna_model.errors.NoActiveEnvironmentFoundError
        == errors.NoActiveEnvironmentFoundError
    )
    assert (
        lm.errors.NoActiveEnvironmentFoundError == errors.NoActiveEnvironmentFoundError
    )
    assert errors.NoActiveEnvironmentFoundError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME
    assert (
        luna_model.errors.NoActiveEnvironmentFoundError
        == errors.NoActiveEnvironmentFoundError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_multiple_active_environments_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import MultipleActiveEnvironmentsError as ID

    # The true path is the errors.MultipleActiveEnvironmentsError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.MultipleActiveEnvironmentsError with the import from
    # luna_model
    assert (
        luna_model.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert (
        lm.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert errors.MultipleActiveEnvironmentsError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME
    assert (
        luna_model.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_decode_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import DecodeError as ID

    # The true path is the errors.DecodeError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.DecodeError with the import from
    # luna_model
    assert luna_model.errors.DecodeError == errors.DecodeError
    assert lm.errors.DecodeError == errors.DecodeError
    assert errors.DecodeError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.DecodeError.__module__ == ERRORS_MODULE_NAME
    assert luna_model.errors.DecodeError == errors.DecodeError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.DecodeError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_model_not_quadratic_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import ModelNotQuadraticError as ID

    # The true path is the errors.ModelNotQuadraticError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.ModelNotQuadraticError with the import from
    # luna_model
    assert luna_model.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert lm.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert errors.ModelNotQuadraticError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME
    assert luna_model.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports()
def test_import_model_not_unconstrained_exception():
    import luna_model
    import luna_model as lm
    from luna_model import errors  # the true way to do it.
    from luna_model.errors import ModelNotUnconstrainedError as ID

    # The true path is the errors.ModelNotUnconstrainedError
    # The luna_model/lm.errors is hacked into the bindings.
    # So we check again the errors.ModelNotUnconstrainedError with the import from
    # luna_model
    assert (
        luna_model.errors.ModelNotUnconstrainedError
        == errors.ModelNotUnconstrainedError
    )
    assert lm.errors.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    assert errors.ModelNotUnconstrainedError == ID

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert lm.errors.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
    assert (
        luna_model.errors.ModelNotUnconstrainedError
        == errors.ModelNotUnconstrainedError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert errors.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
