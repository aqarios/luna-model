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
    let m = PyModule::new(pm.py(), "aqmodels.translator")?;
    m.add_class::<py_matrix_translator::PyMatrixTranslator>()?;
    pm.add_submodule(&m)?;
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels.translator", m)?;
    Ok(())
}

pub fn register_exceptions(pm: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(pm.py(), "aqmodels.exceptions")?;
    m.add(
        pyexc::DecodeException::NAME,
        m.py().get_type::<pyexc::DecodeException>(),
    )?;
    m.add(
        pyexc::DifferentEnvsException::NAME,
        m.py().get_type::<pyexc::DifferentEnvsException>(),
    )?;
    m.add(
        pyexc::ModelNotQuadraticException::NAME,
        m.py().get_type::<pyexc::ModelNotQuadraticException>(),
    )?;
    m.add(
        pyexc::ModelNotUnconstrainedException::NAME,
        m.py().get_type::<pyexc::ModelNotUnconstrainedException>(),
    )?;
    m.add(
        pyexc::MultipleActiveEnvironmentsException::NAME,
        m.py()
            .get_type::<pyexc::MultipleActiveEnvironmentsException>(),
    )?;
    m.add(
        pyexc::NoActiveEnvironmentFoundException::NAME,
        m.py()
            .get_type::<pyexc::NoActiveEnvironmentFoundException>(),
    )?;
    m.add(
        pyexc::VariableExistsException::NAME,
        m.py().get_type::<pyexc::VariableExistsException>(),
    )?;
    m.add(
        pyexc::VariableOutOfRangeException::NAME,
        m.py().get_type::<pyexc::VariableOutOfRangeException>(),
    )?;
    m.add(
        pyexc::VariablesFromDifferentEnvsException::NAME,
        m.py()
            .get_type::<pyexc::VariablesFromDifferentEnvsException>(),
    )?;
    pm.add_submodule(&m)?;
    pm.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("aqmodels.exceptions", m)?;
    Ok(())
}
