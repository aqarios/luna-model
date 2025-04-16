import pytest


AQM_MODULE_NAME: str = "aqmodels"
TRANSLATOR_MODULE_NAME: str = "aqmodels.translator"
ERRORS_MODULE_NAME: str = "aqmodels.errors"


@pytest.mark.imports
def test_import_aqmodels():
    import aqmodels as _  # noqa: F401


@pytest.mark.imports
def test_import_variable():
    import aqmodels as aqm
    from aqmodels import Variable

    assert aqm.Variable == Variable
    assert aqm.Variable.__module__ == Variable.__module__
    assert aqm.Variable.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_vtype():
    import aqmodels as aqm
    from aqmodels import Vtype

    assert aqm.Vtype == Vtype
    assert aqm.Vtype.__module__ == Vtype.__module__
    assert aqm.Vtype.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_comparator():
    import aqmodels as aqm
    from aqmodels import Comparator

    assert aqm.Comparator == Comparator
    assert aqm.Comparator.__module__ == Comparator.__module__
    assert aqm.Comparator.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_environment():
    import aqmodels as aqm
    from aqmodels import Environment

    assert aqm.Environment == Environment
    assert aqm.Environment.__module__ == Environment.__module__
    assert aqm.Environment.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_expression():
    import aqmodels as aqm
    from aqmodels import Expression

    assert aqm.Expression == Expression
    assert aqm.Expression.__module__ == Expression.__module__
    assert aqm.Expression.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_model():
    import aqmodels as aqm
    from aqmodels import Model

    assert aqm.Model == Model
    assert aqm.Model.__module__ == Model.__module__
    assert aqm.Model.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_bounds():
    import aqmodels as aqm
    from aqmodels import Bounds

    assert aqm.Bounds == Bounds
    assert aqm.Bounds.__module__ == Bounds.__module__
    assert aqm.Bounds.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_constraint():
    import aqmodels as aqm
    from aqmodels import Constraint

    assert aqm.Constraint == Constraint
    assert aqm.Constraint.__module__ == Constraint.__module__
    assert aqm.Constraint.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_constraints():
    import aqmodels as aqm
    from aqmodels import Constraints

    assert aqm.Constraints == Constraints
    assert aqm.Constraints.__module__ == Constraints.__module__
    assert aqm.Constraints.__module__ == AQM_MODULE_NAME


@pytest.mark.imports
def test_import_matrix_translator():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import translator  # the true way to do it.
    from aqmodels.translator import MatrixTranslator as MTID
    from aqmodels import MatrixTranslator as MTD

    # The true path is the translator.MatrixTranslator
    # The aqmodels/aqm.translator is hacked into the bindings.
    # So we check again the translator.MatrixTranslator with the import from
    # aqmodels
    assert aqm.MatrixTranslator == translator.MatrixTranslator
    assert aqmodels.MatrixTranslator == translator.MatrixTranslator
    assert aqmodels.translator.MatrixTranslator == translator.MatrixTranslator
    assert aqm.translator.MatrixTranslator == translator.MatrixTranslator
    assert MTID == translator.MatrixTranslator
    assert MTD == translator.MatrixTranslator

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.MatrixTranslator.__module__ == TRANSLATOR_MODULE_NAME
    assert aqmodels.MatrixTranslator.__module__ == TRANSLATOR_MODULE_NAME
    assert aqm.translator.MatrixTranslator.__module__ == TRANSLATOR_MODULE_NAME
    assert aqmodels.translator.MatrixTranslator == translator.MatrixTranslator
    assert MTID.__module__ == TRANSLATOR_MODULE_NAME
    assert MTD.__module__ == TRANSLATOR_MODULE_NAME
    assert translator.MatrixTranslator.__module__ == TRANSLATOR_MODULE_NAME


@pytest.mark.imports
def test_import_variable_out_of_range_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import VariableOutOfRangeError as ID
    from aqmodels import VariableOutOfRangeError as D

    # The true path is the errors.VariableOutOfRangeError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.VariableOutOfRangeError with the import from
    # aqmodels
    assert aqm.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert aqmodels.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert aqmodels.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert aqm.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert ID == errors.VariableOutOfRangeError
    assert D == errors.VariableOutOfRangeError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.errors.VariableOutOfRangeError == errors.VariableOutOfRangeError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.VariableOutOfRangeError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_variable_exists_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import VariableExistsError as ID
    from aqmodels import VariableExistsError as D

    # The true path is the errors.VariableExistsError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.VariableExistsError with the import from
    # aqmodels
    assert aqm.VariableExistsError == errors.VariableExistsError
    assert aqmodels.VariableExistsError == errors.VariableExistsError
    assert aqmodels.errors.VariableExistsError == errors.VariableExistsError
    assert aqm.errors.VariableExistsError == errors.VariableExistsError
    assert ID == errors.VariableExistsError
    assert D == errors.VariableExistsError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariableExistsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.VariableExistsError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.VariableExistsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.errors.VariableExistsError == errors.VariableExistsError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.VariableExistsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_variable_from_different_envs_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import VariablesFromDifferentEnvsError as ID
    from aqmodels import VariablesFromDifferentEnvsError as D

    # The true path is the errors.VariablesFromDifferentEnvsError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.VariablesFromDifferentEnvsError with the import from
    # aqmodels
    assert aqm.VariablesFromDifferentEnvsError == errors.VariablesFromDifferentEnvsError
    assert (
        aqmodels.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert (
        aqmodels.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert (
        aqm.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert ID == errors.VariablesFromDifferentEnvsError
    assert D == errors.VariablesFromDifferentEnvsError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert (
        aqmodels.errors.VariablesFromDifferentEnvsError
        == errors.VariablesFromDifferentEnvsError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.VariablesFromDifferentEnvsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_different_envs_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import DifferentEnvsError as ID
    from aqmodels import DifferentEnvsError as D

    # The true path is the errors.DifferentEnvsError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.DifferentEnvsError with the import from
    # aqmodels
    assert aqm.DifferentEnvsError == errors.DifferentEnvsError
    assert aqmodels.DifferentEnvsError == errors.DifferentEnvsError
    assert aqmodels.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert aqm.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert ID == errors.DifferentEnvsError
    assert D == errors.DifferentEnvsError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.errors.DifferentEnvsError == errors.DifferentEnvsError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.DifferentEnvsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_no_active_environment_found_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import NoActiveEnvironmentFoundError as ID
    from aqmodels import NoActiveEnvironmentFoundError as D

    # The true path is the errors.NoActiveEnvironmentFoundError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.NoActiveEnvironmentFoundError with the import from
    # aqmodels
    assert aqm.NoActiveEnvironmentFoundError == errors.NoActiveEnvironmentFoundError
    assert (
        aqmodels.NoActiveEnvironmentFoundError == errors.NoActiveEnvironmentFoundError
    )
    assert (
        aqmodels.errors.NoActiveEnvironmentFoundError
        == errors.NoActiveEnvironmentFoundError
    )
    assert (
        aqm.errors.NoActiveEnvironmentFoundError == errors.NoActiveEnvironmentFoundError
    )
    assert ID == errors.NoActiveEnvironmentFoundError
    assert D == errors.NoActiveEnvironmentFoundError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME
    assert (
        aqmodels.errors.NoActiveEnvironmentFoundError
        == errors.NoActiveEnvironmentFoundError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.NoActiveEnvironmentFoundError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_multiple_active_environments_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import MultipleActiveEnvironmentsError as ID
    from aqmodels import MultipleActiveEnvironmentsError as D

    # The true path is the errors.MultipleActiveEnvironmentsError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.MultipleActiveEnvironmentsError with the import from
    # aqmodels
    assert aqm.MultipleActiveEnvironmentsError == errors.MultipleActiveEnvironmentsError
    assert (
        aqmodels.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert (
        aqmodels.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert (
        aqm.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert ID == errors.MultipleActiveEnvironmentsError
    assert D == errors.MultipleActiveEnvironmentsError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME
    assert (
        aqmodels.errors.MultipleActiveEnvironmentsError
        == errors.MultipleActiveEnvironmentsError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.MultipleActiveEnvironmentsError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_decode_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import DecodeError as ID
    from aqmodels import DecodeError as D

    # The true path is the errors.DecodeError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.DecodeError with the import from
    # aqmodels
    assert aqm.DecodeError == errors.DecodeError
    assert aqmodels.DecodeError == errors.DecodeError
    assert aqmodels.errors.DecodeError == errors.DecodeError
    assert aqm.errors.DecodeError == errors.DecodeError
    assert ID == errors.DecodeError
    assert D == errors.DecodeError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.DecodeError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.DecodeError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.DecodeError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.errors.DecodeError == errors.DecodeError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.DecodeError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_model_not_quadratic_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import ModelNotQuadraticError as ID
    from aqmodels import ModelNotQuadraticError as D

    # The true path is the errors.ModelNotQuadraticError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.ModelNotQuadraticError with the import from
    # aqmodels
    assert aqm.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert aqmodels.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert aqmodels.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert aqm.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert ID == errors.ModelNotQuadraticError
    assert D == errors.ModelNotQuadraticError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.errors.ModelNotQuadraticError == errors.ModelNotQuadraticError
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.ModelNotQuadraticError.__module__ == ERRORS_MODULE_NAME


@pytest.mark.imports
def test_import_model_not_unconstrained_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import errors  # the true way to do it.
    from aqmodels.errors import ModelNotUnconstrainedError as ID
    from aqmodels import ModelNotUnconstrainedError as D

    # The true path is the errors.ModelNotUnconstrainedError
    # The aqmodels/aqm.errors is hacked into the bindings.
    # So we check again the errors.ModelNotUnconstrainedError with the import from
    # aqmodels
    assert aqm.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    assert aqmodels.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    assert (
        aqmodels.errors.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    )
    assert aqm.errors.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    assert ID == errors.ModelNotUnconstrainedError
    assert D == errors.ModelNotUnconstrainedError

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
    assert aqmodels.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
    assert aqm.errors.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
    assert (
        aqmodels.errors.ModelNotUnconstrainedError == errors.ModelNotUnconstrainedError
    )
    assert ID.__module__ == ERRORS_MODULE_NAME
    assert D.__module__ == ERRORS_MODULE_NAME
    assert errors.ModelNotUnconstrainedError.__module__ == ERRORS_MODULE_NAME
