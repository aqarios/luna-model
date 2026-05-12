use pyo3::prelude::*;
use pyo3_lunamodel::prelude::*;

#[pyfunction]
fn roundtrip_bounds(value: PyBounds) -> PyBounds {
    value
}

#[pyfunction]
fn roundtrip_comparator(value: PyComparator) -> PyComparator {
    value
}

#[pyfunction]
fn roundtrip_constraint(value: PyConstraint) -> PyConstraint {
    value
}

#[pyfunction]
fn roundtrip_constraint_collection(value: PyConstraintCollection) -> PyConstraintCollection {
    value
}

#[pyfunction]
fn roundtrip_ctype(value: PyCtype) -> PyCtype {
    value
}

#[pyfunction]
fn roundtrip_environment(value: PyEnvironment) -> PyEnvironment {
    value
}

#[pyfunction]
fn roundtrip_expression(value: PyExpression) -> PyExpression {
    value
}

#[pyfunction]
fn roundtrip_model(value: PyModel) -> PyModel {
    value
}

#[pyfunction]
fn roundtrip_model_specs(value: PyModelSpecs) -> PyModelSpecs {
    value
}

#[pyfunction]
fn roundtrip_sense(value: PySense) -> PySense {
    value
}

#[pyfunction]
fn roundtrip_solution(value: PySolution) -> PySolution {
    value
}

#[pyfunction]
fn roundtrip_translation_target(value: PyTranslationTarget) -> PyTranslationTarget {
    value
}

#[pyfunction]
fn roundtrip_unbounded(value: PyUnbounded) -> PyUnbounded {
    value
}

#[pyfunction]
fn roundtrip_value_source(value: PyValueSource) -> PyValueSource {
    value
}

#[pyfunction]
fn roundtrip_variable(value: PyVariable) -> PyVariable {
    value
}

#[pyfunction]
fn roundtrip_vtype(value: PyVtype) -> PyVtype {
    value
}

#[pymodule]
fn pyo3_lunamodel_integration_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(roundtrip_bounds, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_comparator, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_constraint, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_constraint_collection, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_ctype, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_environment, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_expression, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_model, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_model_specs, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_sense, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_solution, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_translation_target, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_unbounded, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_value_source, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_variable, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_vtype, m)?)?;
    Ok(())
}
