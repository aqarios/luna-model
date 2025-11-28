use pyo3::prelude::*;
pub use lunamodel_python::*;


#[pymodule]
fn _core(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyExpression>()?;
    m.add_class::<PyVariable>()?;
    m.add_class::<PyEnvironment>()?;
    Ok(())
}
