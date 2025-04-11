use pyo3::{prelude::*, PyTypeCheck};

use crate::core::{Comparator, Vtype};

use super::{
    py_bounds, py_constr, py_env, py_exceptions as pyexc, py_expr, py_model, py_res, py_sample,
    py_sol, py_timing, py_translator, py_var,
};

// #[pymodule]
pub fn register_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add core components not as wrappers, required for e.g. enums
    m.add_class::<Vtype>()?;
    m.add_class::<Comparator>()?;
    // Add core components as wrappers.
    m.add_class::<py_env::PyEnvironment>()?;
    m.add_class::<py_expr::PyExpression>()?;
    m.add_class::<py_model::PyModel>()?;
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
    Ok(())
}

pub fn register_translator(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "translator")?;
    m.add_class::<py_translator::PyMatrixTranslator>()?;
    m.add_class::<py_translator::PySampleSetTranslator>()?;
    m.add_class::<py_translator::PyLpTranslator>()?;
    pm.add_submodule(&m)?;
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels.translator", m)?;
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
        pyexc::VariableOutOfRangeError::NAME,
        m.py().get_type::<pyexc::VariableOutOfRangeError>(),
    )?;
    m.add(
        pyexc::VariablesFromDifferentEnvsError::NAME,
        m.py().get_type::<pyexc::VariablesFromDifferentEnvsError>(),
    )?;
    m.add(
        pyexc::SolutionCreationError::NAME,
        m.py().get_type::<pyexc::SolutionCreationError>(),
    )?;
    pm.add_submodule(&m)?;
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels.exceptions", m)?;
    Ok(())
}
