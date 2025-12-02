pub use lunamodel_python::*;
use pyo3::prelude::*;

#[pymodule]
fn _lm(m: &Bound<PyModule>) -> PyResult<()> {
    // Enums
    m.add_class::<Vtype>()?;
    // Classes
    m.add_class::<PyExpression>()?;
    m.add_class::<PyVariable>()?;
    m.add_class::<PyEnvironment>()?;
    Ok(())
}
