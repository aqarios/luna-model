mod py_bounds;
mod py_constr;
mod py_env;
mod py_exceptions;
mod py_expr;
mod py_matrix_translator;
mod py_model;
mod py_var;

use pyo3::prelude::*;

use py_exceptions::{
    DecodeException, DifferentEnvsException, ModelNotQuadraticException,
    ModelNotUnconstrainedException, MultipleActiveEnvironmentsException,
    NoActiveEnvironmentFoundException, VariableExistsException, VariableOutOfRangeException,
    VariablesFromDifferentEnvsException,
};

use crate::core::{Comparator, Vtype};

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information to the python module
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // Add core components not as wrappers, required for e.g. enums
    m.add_class::<Vtype>()?;
    m.add_class::<Comparator>()?;
    // Add core components as wrappers.
    m.add_class::<py_env::PyEnvironment>()?;
    m.add_class::<py_expr::PyExpression>()?;
    m.add_class::<py_matrix_translator::PyMatrixTranslator>()?;
    m.add_class::<py_model::PyModel>()?;
    m.add_class::<py_var::PyVariable>()?;
    m.add_class::<py_bounds::PyBounds>()?;
    m.add_class::<py_constr::PyConstraint>()?;
    m.add_class::<py_constr::PyConstraints>()?;
    // Adding the exceptions
    m.add("DecodeException", m.py().get_type::<DecodeException>())?;
    m.add(
        "DifferentEnvsException",
        m.py().get_type::<DifferentEnvsException>(),
    )?;
    m.add(
        "ModelNotQuadraticException",
        m.py().get_type::<ModelNotQuadraticException>(),
    )?;
    m.add(
        "ModelNotUnconstrainedException",
        m.py().get_type::<ModelNotUnconstrainedException>(),
    )?;
    m.add(
        "MultipleActiveEnvironmentsException",
        m.py().get_type::<MultipleActiveEnvironmentsException>(),
    )?;
    m.add(
        "NoActiveEnvironmentFoundException",
        m.py().get_type::<NoActiveEnvironmentFoundException>(),
    )?;
    m.add(
        "VariableExistsException",
        m.py().get_type::<VariableExistsException>(),
    )?;
    m.add(
        "VariableOutOfRangeException",
        m.py().get_type::<VariableOutOfRangeException>(),
    )?;
    m.add(
        "VariablesFromDifferentEnvsException",
        m.py().get_type::<VariablesFromDifferentEnvsException>(),
    )?;

    Ok(())
}
