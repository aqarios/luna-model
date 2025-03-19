mod py_bounds;
mod py_constr;
mod py_env;
mod py_exceptions;
mod py_expr;
mod py_matrix_translator;
mod py_model;
mod py_sol;
mod py_timing;
mod py_var;
mod solution_translator;
mod types;

use pyo3::prelude::*;

use crate::core::{
    MultipleActiveEnvironmentsException, NoActiveEnvironmentFoundException,
    VariableExistsException, Vtype,
};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information to the python module
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // Add core components not as wrappers, required for e.g. enums
    m.add_class::<Vtype>()?;
    // Add core components as wrappers.
    m.add_class::<py_env::PyEnvironment>()?;
    m.add_class::<py_expr::PyExpression>()?;
    m.add_class::<py_matrix_translator::PyMatrixTranslator>()?;
    m.add_class::<py_model::PyModel>()?;
    m.add_class::<py_var::PyVariable>()?;
    m.add_class::<py_bounds::PyBounds>()?;
    m.add_class::<py_constr::PyConstraint>()?;
    m.add_class::<py_constr::PyConstraints>()?;
    m.add_class::<py_timing::PyTiming>()?;
    m.add_class::<py_timing::PyTimer>()?;
    m.add_class::<py_sol::PyRes>()?;
    m.add_class::<py_sol::PyResults>()?;
    m.add_class::<py_sol::PySolution>()?;
    m.add_class::<solution_translator::PySampleSetTranslator>()?;
    // Adding the exceptions
    m.add(
        "VariableExistsException",
        m.py().get_type::<VariableExistsException>(),
    )?;
    m.add(
        "NoActiveEnvironmentFoundException",
        m.py().get_type::<NoActiveEnvironmentFoundException>(),
    )?;
    m.add(
        "MultipleActiveEnvironmentsException",
        m.py().get_type::<MultipleActiveEnvironmentsException>(),
    )?;
    Ok(())
}
