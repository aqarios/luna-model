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

/// Reads the higher-order storage in Rust, so the host's in-memory layout is
/// actually interpreted by the extension (a plain roundtrip only moves the Arc).
#[pyfunction]
fn expression_higher_order_items(value: PyExpression) -> Vec<(Vec<u32>, f64)> {
    let mut items: Vec<_> = value.0.read_with(|e| {
        e.raw_higher_order_items()
            .map(|(vars, bias)| (vars.to_vec(), bias))
            .collect()
    });
    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
}

/// Looks up a higher-order bias by key, exercising the host map's hashing.
#[pyfunction]
fn expression_higher_order_bias(value: PyExpression, vars: Vec<u32>) -> f64 {
    value.0.read_with(|e| e.higher_order(&vars))
}

/// Computes a new expression in Rust from the host's expression.
#[pyfunction]
fn expression_scaled_higher_order_items(value: PyExpression, factor: f64) -> PyResult<Vec<(Vec<u32>, f64)>> {
    let scaled = value
        .0
        .mul(factor)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    let mut items: Vec<_> = scaled
        .raw_higher_order_items()
        .map(|(vars, bias)| (vars.to_vec(), bias))
        .collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(items)
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
    m.add_function(wrap_pyfunction!(expression_higher_order_items, m)?)?;
    m.add_function(wrap_pyfunction!(expression_higher_order_bias, m)?)?;
    m.add_function(wrap_pyfunction!(expression_scaled_higher_order_items, m)?)?;
    Ok(())
}
