pub use lunamodel_python::*;
use pyo3::prelude::*;

#[pymodule]
fn _lm(m: &Bound<PyModule>) -> PyResult<()> {
    // Enums from lunamodel-core
    m.add_class::<ValueSource>()?;
    // Enums from lunamodel-types
    m.add_class::<Vtype>()?;
    m.add_class::<Sense>()?;
    m.add_class::<Ctype>()?;
    m.add_class::<Comparator>()?;
    // Enums from lunamodel-translate
    m.add_class::<TranslationTarget>()?;
    m.add_class::<SolutionSource>()?;
    // extaa
    m.add_class::<PyUnbounded>()?;
    // Classes
    m.add_class::<PyExpression>()?;
    m.add_class::<PyVariable>()?;
    m.add_class::<PyEnvironment>()?;
    m.add_class::<PyBounds>()?;
    m.add_class::<PyModel>()?;
    m.add_class::<PyConstraint>()?;
    m.add_class::<PyModelSpecs>()?;
    m.add_class::<PyConstraintCollection>()?;
    m.add_class::<PySolution>()?;
    m.add_class::<PyTimer>()?;

    // For Expression iteration
    m.add_class::<PyExpressionIterator>()?;
    m.add_class::<PyConstant>()?;
    m.add_class::<PyLinear>()?;
    m.add_class::<PyQuadratic>()?;
    m.add_class::<PyHigherOrder>()?;
    Ok(())
}
