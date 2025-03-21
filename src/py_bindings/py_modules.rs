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

pub fn register_translator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // let tm = PyModule::new(m.py(), "translator")?;
    m.add_class::<py_matrix_translator::PyMatrixTranslator>()?;
    // m.add_submodule(&tm)?;
    Ok(())
}

pub fn register_exceptions(em: &Bound<'_, PyModule>) -> PyResult<()> {
    // let em = PyModule::new(m.py(), "exceptions")?;
    em.add(
        pyexc::DecodeException::NAME,
        em.py().get_type::<pyexc::DecodeException>(),
    )?;
    em.add(
        pyexc::DifferentEnvsException::NAME,
        em.py().get_type::<pyexc::DifferentEnvsException>(),
    )?;
    em.add(
        pyexc::ModelNotQuadraticException::NAME,
        em.py().get_type::<pyexc::ModelNotQuadraticException>(),
    )?;
    em.add(
        pyexc::ModelNotUnconstrainedException::NAME,
        em.py().get_type::<pyexc::ModelNotUnconstrainedException>(),
    )?;
    em.add(
        pyexc::MultipleActiveEnvironmentsException::NAME,
        em.py()
            .get_type::<pyexc::MultipleActiveEnvironmentsException>(),
    )?;
    em.add(
        pyexc::NoActiveEnvironmentFoundException::NAME,
        em.py()
            .get_type::<pyexc::NoActiveEnvironmentFoundException>(),
    )?;
    em.add(
        pyexc::VariableExistsException::NAME,
        em.py().get_type::<pyexc::VariableExistsException>(),
    )?;
    em.add(
        pyexc::VariableOutOfRangeException::NAME,
        em.py().get_type::<pyexc::VariableOutOfRangeException>(),
    )?;
    em.add(
        pyexc::VariablesFromDifferentEnvsException::NAME,
        em.py()
            .get_type::<pyexc::VariablesFromDifferentEnvsException>(),
    )?;
    Ok(())
}
