import pytest


AQM_MODULE_NAME: str = "aqmodels"
TRANSLATOR_MODULE_NAME: str = "aqmodels.translator"
EXCEPTIONS_MODULE_NAME: str = "aqmodels.exceptions"


@pytest.mark.imports
def test_import_aqmodels():
    import aqmodels as _


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
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import VariableOutOfRangeException as ID
    from aqmodels import VariableOutOfRangeException as D

    # The true path is the exceptions.VariableOutOfRangeException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.VariableOutOfRangeException with the import from
    # aqmodels
    assert aqm.VariableOutOfRangeException == exceptions.VariableOutOfRangeException
    assert (
        aqmodels.VariableOutOfRangeException == exceptions.VariableOutOfRangeException
    )
    assert (
        aqmodels.exceptions.VariableOutOfRangeException
        == exceptions.VariableOutOfRangeException
    )
    assert (
        aqm.exceptions.VariableOutOfRangeException
        == exceptions.VariableOutOfRangeException
    )
    assert ID == exceptions.VariableOutOfRangeException
    assert D == exceptions.VariableOutOfRangeException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariableOutOfRangeException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.VariableOutOfRangeException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqm.exceptions.VariableOutOfRangeException.__module__ == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.VariableOutOfRangeException
        == exceptions.VariableOutOfRangeException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert exceptions.VariableOutOfRangeException.__module__ == EXCEPTIONS_MODULE_NAME


@pytest.mark.imports
def test_import_variable_exists_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import VariableExistsException as ID
    from aqmodels import VariableExistsException as D

    # The true path is the exceptions.VariableExistsException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.VariableExistsException with the import from
    # aqmodels
    assert aqm.VariableExistsException == exceptions.VariableExistsException
    assert aqmodels.VariableExistsException == exceptions.VariableExistsException
    assert (
        aqmodels.exceptions.VariableExistsException
        == exceptions.VariableExistsException
    )
    assert aqm.exceptions.VariableExistsException == exceptions.VariableExistsException
    assert ID == exceptions.VariableExistsException
    assert D == exceptions.VariableExistsException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariableExistsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.VariableExistsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqm.exceptions.VariableExistsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqmodels.exceptions.VariableExistsException
        == exceptions.VariableExistsException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert exceptions.VariableExistsException.__module__ == EXCEPTIONS_MODULE_NAME


@pytest.mark.imports
def test_import_variable_from_different_envs_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import VariablesFromDifferentEnvsException as ID
    from aqmodels import VariablesFromDifferentEnvsException as D

    # The true path is the exceptions.VariablesFromDifferentEnvsException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.VariablesFromDifferentEnvsException with the import from
    # aqmodels
    assert (
        aqm.VariablesFromDifferentEnvsException
        == exceptions.VariablesFromDifferentEnvsException
    )
    assert (
        aqmodels.VariablesFromDifferentEnvsException
        == exceptions.VariablesFromDifferentEnvsException
    )
    assert (
        aqmodels.exceptions.VariablesFromDifferentEnvsException
        == exceptions.VariablesFromDifferentEnvsException
    )
    assert (
        aqm.exceptions.VariablesFromDifferentEnvsException
        == exceptions.VariablesFromDifferentEnvsException
    )
    assert ID == exceptions.VariablesFromDifferentEnvsException
    assert D == exceptions.VariablesFromDifferentEnvsException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.VariablesFromDifferentEnvsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqmodels.VariablesFromDifferentEnvsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqm.exceptions.VariablesFromDifferentEnvsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.VariablesFromDifferentEnvsException
        == exceptions.VariablesFromDifferentEnvsException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        exceptions.VariablesFromDifferentEnvsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )


@pytest.mark.imports
def test_import_different_envs_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import DifferentEnvsException as ID
    from aqmodels import DifferentEnvsException as D

    # The true path is the exceptions.DifferentEnvsException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.DifferentEnvsException with the import from
    # aqmodels
    assert aqm.DifferentEnvsException == exceptions.DifferentEnvsException
    assert aqmodels.DifferentEnvsException == exceptions.DifferentEnvsException
    assert (
        aqmodels.exceptions.DifferentEnvsException == exceptions.DifferentEnvsException
    )
    assert aqm.exceptions.DifferentEnvsException == exceptions.DifferentEnvsException
    assert ID == exceptions.DifferentEnvsException
    assert D == exceptions.DifferentEnvsException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.DifferentEnvsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.DifferentEnvsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqm.exceptions.DifferentEnvsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqmodels.exceptions.DifferentEnvsException == exceptions.DifferentEnvsException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert exceptions.DifferentEnvsException.__module__ == EXCEPTIONS_MODULE_NAME


@pytest.mark.imports
def test_import_no_active_environment_found_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import NoActiveEnvironmentFoundException as ID
    from aqmodels import NoActiveEnvironmentFoundException as D

    # The true path is the exceptions.NoActiveEnvironmentFoundException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.NoActiveEnvironmentFoundException with the import from
    # aqmodels
    assert (
        aqm.NoActiveEnvironmentFoundException
        == exceptions.NoActiveEnvironmentFoundException
    )
    assert (
        aqmodels.NoActiveEnvironmentFoundException
        == exceptions.NoActiveEnvironmentFoundException
    )
    assert (
        aqmodels.exceptions.NoActiveEnvironmentFoundException
        == exceptions.NoActiveEnvironmentFoundException
    )
    assert (
        aqm.exceptions.NoActiveEnvironmentFoundException
        == exceptions.NoActiveEnvironmentFoundException
    )
    assert ID == exceptions.NoActiveEnvironmentFoundException
    assert D == exceptions.NoActiveEnvironmentFoundException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.NoActiveEnvironmentFoundException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqmodels.NoActiveEnvironmentFoundException.__module__ == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqm.exceptions.NoActiveEnvironmentFoundException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.NoActiveEnvironmentFoundException
        == exceptions.NoActiveEnvironmentFoundException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        exceptions.NoActiveEnvironmentFoundException.__module__
        == EXCEPTIONS_MODULE_NAME
    )


@pytest.mark.imports
def test_import_multiple_active_environments_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import MultipleActiveEnvironmentsException as ID
    from aqmodels import MultipleActiveEnvironmentsException as D

    # The true path is the exceptions.MultipleActiveEnvironmentsException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.MultipleActiveEnvironmentsException with the import from
    # aqmodels
    assert (
        aqm.MultipleActiveEnvironmentsException
        == exceptions.MultipleActiveEnvironmentsException
    )
    assert (
        aqmodels.MultipleActiveEnvironmentsException
        == exceptions.MultipleActiveEnvironmentsException
    )
    assert (
        aqmodels.exceptions.MultipleActiveEnvironmentsException
        == exceptions.MultipleActiveEnvironmentsException
    )
    assert (
        aqm.exceptions.MultipleActiveEnvironmentsException
        == exceptions.MultipleActiveEnvironmentsException
    )
    assert ID == exceptions.MultipleActiveEnvironmentsException
    assert D == exceptions.MultipleActiveEnvironmentsException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.MultipleActiveEnvironmentsException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqmodels.MultipleActiveEnvironmentsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqm.exceptions.MultipleActiveEnvironmentsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.MultipleActiveEnvironmentsException
        == exceptions.MultipleActiveEnvironmentsException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        exceptions.MultipleActiveEnvironmentsException.__module__
        == EXCEPTIONS_MODULE_NAME
    )


@pytest.mark.imports
def test_import_decode_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import DecodeException as ID
    from aqmodels import DecodeException as D

    # The true path is the exceptions.DecodeException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.DecodeException with the import from
    # aqmodels
    assert aqm.DecodeException == exceptions.DecodeException
    assert aqmodels.DecodeException == exceptions.DecodeException
    assert aqmodels.exceptions.DecodeException == exceptions.DecodeException
    assert aqm.exceptions.DecodeException == exceptions.DecodeException
    assert ID == exceptions.DecodeException
    assert D == exceptions.DecodeException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.DecodeException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.DecodeException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqm.exceptions.DecodeException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.exceptions.DecodeException == exceptions.DecodeException
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert exceptions.DecodeException.__module__ == EXCEPTIONS_MODULE_NAME


@pytest.mark.imports
def test_import_model_not_quadratic_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import ModelNotQuadraticException as ID
    from aqmodels import ModelNotQuadraticException as D

    # The true path is the exceptions.ModelNotQuadraticException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.ModelNotQuadraticException with the import from
    # aqmodels
    assert aqm.ModelNotQuadraticException == exceptions.ModelNotQuadraticException
    assert aqmodels.ModelNotQuadraticException == exceptions.ModelNotQuadraticException
    assert (
        aqmodels.exceptions.ModelNotQuadraticException
        == exceptions.ModelNotQuadraticException
    )
    assert (
        aqm.exceptions.ModelNotQuadraticException
        == exceptions.ModelNotQuadraticException
    )
    assert ID == exceptions.ModelNotQuadraticException
    assert D == exceptions.ModelNotQuadraticException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.ModelNotQuadraticException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.ModelNotQuadraticException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqm.exceptions.ModelNotQuadraticException.__module__ == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.ModelNotQuadraticException
        == exceptions.ModelNotQuadraticException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert exceptions.ModelNotQuadraticException.__module__ == EXCEPTIONS_MODULE_NAME


@pytest.mark.imports
def test_import_model_not_unconstrained_exception():
    import aqmodels
    import aqmodels as aqm
    from aqmodels import exceptions  # the true way to do it.
    from aqmodels.exceptions import ModelNotUnconstrainedException as ID
    from aqmodels import ModelNotUnconstrainedException as D

    # The true path is the exceptions.ModelNotUnconstrainedException
    # The aqmodels/aqm.exceptions is hacked into the bindings.
    # So we check again the exceptions.ModelNotUnconstrainedException with the import from
    # aqmodels
    assert (
        aqm.ModelNotUnconstrainedException == exceptions.ModelNotUnconstrainedException
    )
    assert (
        aqmodels.ModelNotUnconstrainedException
        == exceptions.ModelNotUnconstrainedException
    )
    assert (
        aqmodels.exceptions.ModelNotUnconstrainedException
        == exceptions.ModelNotUnconstrainedException
    )
    assert (
        aqm.exceptions.ModelNotUnconstrainedException
        == exceptions.ModelNotUnconstrainedException
    )
    assert ID == exceptions.ModelNotUnconstrainedException
    assert D == exceptions.ModelNotUnconstrainedException

    # the transaltor module is TRANSLATOR_MODULE_NAME
    assert aqm.ModelNotUnconstrainedException.__module__ == EXCEPTIONS_MODULE_NAME
    assert aqmodels.ModelNotUnconstrainedException.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        aqm.exceptions.ModelNotUnconstrainedException.__module__
        == EXCEPTIONS_MODULE_NAME
    )
    assert (
        aqmodels.exceptions.ModelNotUnconstrainedException
        == exceptions.ModelNotUnconstrainedException
    )
    assert ID.__module__ == EXCEPTIONS_MODULE_NAME
    assert D.__module__ == EXCEPTIONS_MODULE_NAME
    assert (
        exceptions.ModelNotUnconstrainedException.__module__ == EXCEPTIONS_MODULE_NAME
    )
