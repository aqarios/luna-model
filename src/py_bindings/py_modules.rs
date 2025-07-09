use pyo3::{prelude::*, PyTypeCheck};

use crate::core::{Comparator, Sense, Vtype};

use super::{
    py_bounds, py_constr, py_env, py_exceptions as pyexc, py_expr, py_model, py_model_metadata,
    py_res, py_sample, py_sol, py_timing, py_translator, py_utils, py_var,
};

// #[pymodule]
pub fn register_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add core components not as wrappers, required for, e.g., enums
    m.add_class::<Vtype>()?;
    m.add_class::<Comparator>()?;
    m.add_class::<Sense>()?;
    // Add core components as wrappers.
    m.add_class::<py_env::PyEnvironment>()?;
    m.add_class::<py_expr::PyExpression>()?;
    m.add_class::<py_expr::PyExpressionIterator>()?;
    m.add_class::<py_expr::PyConstant>()?;
    m.add_class::<py_expr::PyLinear>()?;
    m.add_class::<py_expr::PyQuadratic>()?;
    m.add_class::<py_expr::PyHigherOrder>()?;
    m.add_class::<py_model::PyModel>()?;
    m.add_class::<py_model_metadata::PyModelMetadata>()?;
    m.add_class::<py_var::PyVariable>()?;
    m.add_class::<py_bounds::PyBounds>()?;
    m.add_class::<py_constr::PyConstraint>()?;
    m.add_class::<py_constr::PyConstraints>()?;
    m.add_class::<py_res::PyResultView>()?;
    m.add_class::<py_res::PyOwnedResult>()?;
    m.add_class::<py_res::PyResultIterator>()?;
    m.add_class::<py_sample::PySamplesIterator>()?;
    m.add_class::<py_sample::PySampleIterator>()?;
    m.add_class::<py_sample::PySamples>()?;
    m.add_class::<py_sample::PySample>()?;
    m.add_class::<py_sol::PySolution>()?;
    m.add_class::<py_timing::PyTiming>()?;
    m.add_class::<py_timing::PyTimer>()?;
    m.add_class::<py_bounds::PyUnbounded>()?;
    Ok(())
}

pub fn register_utils(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "utils")?;
    m.add_function(wrap_pyfunction!(py_utils::quicksum, &m)?)?;
    pm.add_submodule(&m)?;
    #[cfg(not(feature = "lq"))]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels._core.utils", m)?;
    #[cfg(feature = "lq")]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("luna_quantum._core.utils", m)?;
    Ok(())
}

pub fn register_translator(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "translator")?;
    m.add_class::<py_translator::PyQubo>()?;
    m.add_class::<py_translator::PyQuboTranslator>()?;
    m.add_class::<py_translator::PyBqmTranslator>()?;
    m.add_class::<py_translator::PyCqmTranslator>()?;
    m.add_class::<py_translator::PyDwaveTranslator>()?;
    m.add_class::<py_translator::PyQctrlTranslator>()?;
    m.add_class::<py_translator::PyLpTranslator>()?;
    m.add_class::<py_translator::PyIbmTranslator>()?;
    m.add_class::<py_translator::PyZibTranslator>()?;
    m.add_class::<py_translator::PyAwsTranslator>()?;
    m.add_class::<py_translator::PyNumpyTranslator>()?;
    pm.add_submodule(&m)?;
    #[cfg(not(feature = "lq"))]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels._core.translator", m)?;
    #[cfg(feature = "lq")]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("luna_quantum._core.translator", m)?;
    Ok(())
}

pub fn register_errors(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "errors")?;
    m.add(
        pyexc::DecodeError::NAME,
        m.py().get_type::<pyexc::DecodeError>(),
    )?;
    m.add(
        pyexc::DifferentEnvsError::NAME,
        m.py().get_type::<pyexc::DifferentEnvsError>(),
    )?;
    m.add(
        pyexc::ModelNotQuadraticError::NAME,
        m.py().get_type::<pyexc::ModelNotQuadraticError>(),
    )?;
    m.add(
        pyexc::ModelNotUnconstrainedError::NAME,
        m.py().get_type::<pyexc::ModelNotUnconstrainedError>(),
    )?;
    m.add(
        pyexc::ModelSenseNotMinimizeError::NAME,
        m.py().get_type::<pyexc::ModelSenseNotMinimizeError>(),
    )?;
    m.add(
        pyexc::ModelVtypeError::NAME,
        m.py().get_type::<pyexc::ModelVtypeError>(),
    )?;
    m.add(
        pyexc::VariableNamesError::NAME,
        m.py().get_type::<pyexc::VariableNamesError>(),
    )?;
    m.add(
        pyexc::MultipleActiveEnvironmentsError::NAME,
        m.py().get_type::<pyexc::MultipleActiveEnvironmentsError>(),
    )?;
    m.add(
        pyexc::NoActiveEnvironmentFoundError::NAME,
        m.py().get_type::<pyexc::NoActiveEnvironmentFoundError>(),
    )?;
    m.add(
        pyexc::VariableExistsError::NAME,
        m.py().get_type::<pyexc::VariableExistsError>(),
    )?;
    m.add(
        pyexc::VariableNotExistingError::NAME,
        m.py().get_type::<pyexc::VariableNotExistingError>(),
    )?;
    m.add(
        pyexc::VariableOutOfRangeError::NAME,
        m.py().get_type::<pyexc::VariableOutOfRangeError>(),
    )?;
    m.add(
        pyexc::VariableCreationError::NAME,
        m.py().get_type::<pyexc::VariableCreationError>(),
    )?;
    m.add(
        pyexc::VariablesFromDifferentEnvsError::NAME,
        m.py().get_type::<pyexc::VariablesFromDifferentEnvsError>(),
    )?;
    m.add(
        pyexc::SolutionTranslationError::NAME,
        m.py().get_type::<pyexc::SolutionTranslationError>(),
    )?;
    m.add(
        pyexc::SampleIncorrectLengthError::NAME,
        m.py().get_type::<pyexc::SampleIncorrectLengthError>(),
    )?;
    m.add(
        pyexc::SampleUnexpectedVariableError::NAME,
        m.py().get_type::<pyexc::SampleUnexpectedVariableError>(),
    )?;
    m.add(
        pyexc::SampleIncompatibleVtypeError::NAME,
        m.py().get_type::<pyexc::SampleIncompatibleVtypeError>(),
    )?;
    m.add(
        pyexc::IllegalConstraintNameError::NAME,
        m.py().get_type::<pyexc::IllegalConstraintNameError>(),
    )?;
    m.add(
        pyexc::TranslationError::NAME,
        m.py().get_type::<pyexc::TranslationError>(),
    )?;
    m.add(
        pyexc::ComputationError::NAME,
        m.py().get_type::<pyexc::ComputationError>(),
    )?;
    m.add(
        pyexc::EvaluationError::NAME,
        m.py().get_type::<pyexc::EvaluationError>(),
    )?;
    m.add(
        pyexc::DuplicateConstraintNameError::NAME,
        m.py().get_type::<pyexc::DuplicateConstraintNameError>(),
    )?;
    m.add(
        pyexc::CompilationError::NAME,
        m.py().get_type::<pyexc::CompilationError>(),
    )?;
    m.add(
        pyexc::StartCannotBeInferredError::NAME,
        m.py().get_type::<pyexc::StartCannotBeInferredError>(),
    )?;
    m.add(
        pyexc::NoConstraintForKeyError::NAME,
        m.py().get_type::<pyexc::NoConstraintForKeyError>(),
    )?;
    pm.add_submodule(&m)?;
    #[cfg(not(feature = "lq"))]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels._core.errors", m)?;
    #[cfg(feature = "lq")]
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("luna_quantum._core.errors", m)?;
    Ok(())
}
