// maybe change to `use crate::prelude::*` later
use crate::core::{environment::Environment, expression::Expression, varref::VarRef};
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information to the python module
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // Adding the functions
    m.add_class::<Environment>()?;
    m.add_class::<Expression>()?;
    m.add_class::<VarRef>()?;
    // older stuff
    // m.add_class::<Variable>()?;
    // m.add_class::<Model>()?;
    // m.add_class::<Vtype>()?;
    // m.add_class::<Expression>()?;
    //
    // todo: remove
    // m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    Ok(())
}
