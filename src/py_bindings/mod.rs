mod py_bounds;
mod py_constr;
mod py_env;
mod py_exceptions;
mod py_expr;
mod py_model;
mod py_modules;
mod py_sol;
mod py_timing;
mod py_translator;
mod py_var;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_core")]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information to the python module
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    py_modules::register_core(m)?;
    py_modules::register_translator(m)?;
    py_modules::register_errors(m)?;
    Ok(())
}
