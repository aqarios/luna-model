use pyo3::{prelude::*, PyTypeCheck};

use crate::core::{Comparator, Vtype};

use super::{
    py_bounds, py_constr, py_env, py_exceptions as pyexc, py_expr, py_matrix_translator, py_model,
    py_var,
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

    Ok(())
}

pub fn register_translator(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "translator")?;
    m.add_class::<py_matrix_translator::PyMatrixTranslator>()?;
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
    pm.add_submodule(&m)?;
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels.exceptions", m)?;
    Ok(())
}
