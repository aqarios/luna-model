use pyo3::{exceptions::PyRuntimeError, prelude::*};
use rs_lunamodel::py::prelude::*;

fn lm_error(error: rs_lunamodel::Error) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

#[pyclass]
struct HeldModel {
    model: PyModel,
}

#[pyclass]
struct HeldSolution {
    solution: PySolution,
}

#[pyclass]
struct HeldEnvironment {
    environment: PyEnvironment,
}

#[pyclass]
struct HeldExpression {
    expression: PyExpression,
}

#[pyclass]
struct HeldConstraintCollection {
    constraints: PyConstraintCollection,
}

#[pyclass]
struct HeldConstraint {
    constraint: PyConstraint,
}

#[pymethods]
impl HeldModel {
    #[new]
    fn new(model: PyModel) -> Self {
        Self { model }
    }

    fn name(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.model.name()).map_err(lm_error)
    }

    fn set_name(&mut self, py: Python<'_>, name: String) -> PyResult<()> {
        py.detach(|| self.model.set_name(name)).map_err(lm_error)
    }
}

#[pymethods]
impl HeldSolution {
    #[new]
    fn new(solution: PySolution) -> Self {
        Self { solution }
    }

    fn variable_names(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        py.detach(|| self.solution.variable_names())
            .map_err(lm_error)
    }

    fn add_real(&mut self, py: Python<'_>, name: String, data: Vec<f64>) -> PyResult<()> {
        py.detach(|| self.solution.add_real(&name, data))
            .map_err(lm_error)
    }
}

#[pymethods]
impl HeldEnvironment {
    #[new]
    fn new(environment: PyEnvironment) -> Self {
        Self { environment }
    }

    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        py.detach(|| self.environment.len()).map_err(lm_error)
    }

    fn insert_real(&mut self, py: Python<'_>, name: String) -> PyResult<()> {
        py.detach(|| {
            self.environment
                .insert(&name, rs_lunamodel::Vtype::Real, None)
                .map(|_| ())
        })
        .map_err(lm_error)
    }
}

#[pymethods]
impl HeldExpression {
    #[new]
    fn new(expression: PyExpression) -> Self {
        Self { expression }
    }

    fn offset(&self, py: Python<'_>) -> PyResult<f64> {
        py.detach(|| self.expression.offset()).map_err(lm_error)
    }

    fn add_bias(&mut self, py: Python<'_>, value: f64) -> PyResult<()> {
        py.detach(|| rs_lunamodel::TryAddAssign::try_add_assign(self.expression.inner_mut(), value))
            .map_err(lm_error)
    }
}

#[pymethods]
impl HeldConstraintCollection {
    #[new]
    fn new(constraints: PyConstraintCollection) -> Self {
        Self { constraints }
    }

    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        py.detach(|| self.constraints.len()).map_err(lm_error)
    }

    fn add_constraint(
        &mut self,
        py: Python<'_>,
        constraint: PyConstraint,
        name: Option<String>,
    ) -> PyResult<String> {
        py.detach(|| {
            self.constraints
                .add_constraint(&constraint, name.as_deref())
        })
        .map_err(lm_error)
    }
}

#[pymethods]
impl HeldConstraint {
    #[new]
    fn new(constraint: PyConstraint) -> Self {
        Self { constraint }
    }

    fn name(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.constraint.name()).map_err(lm_error)
    }

    fn set_name(&mut self, py: Python<'_>, name: String) -> PyResult<()> {
        py.detach(|| self.constraint.set_name(name))
            .map_err(lm_error)
    }
}

#[pyfunction]
fn roundtrip_bounds(value: PyBounds) -> PyBounds {
    value
}

#[pyfunction]
fn new_bounds(lower: f64, upper: f64) -> PyResult<PyBounds> {
    rs_lunamodel::LazyBounds::new(
        Some(rs_lunamodel::Bound::Bounded(lower)),
        Some(rs_lunamodel::Bound::Bounded(upper)),
    )
    .map(PyBounds::from)
    .map_err(lm_error)
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
fn new_model(name: String) -> PyResult<PyModel> {
    rs_lunamodel::Model::new(Some(name), None)
        .map(PyModel::from)
        .map_err(lm_error)
}

#[pyfunction]
fn model_environment(model: PyModel) -> PyResult<PyEnvironment> {
    model.env().map(PyEnvironment::from).map_err(lm_error)
}

#[pyfunction]
fn model_variable(model: PyModel, name: String) -> PyResult<PyVariable> {
    model.var(&name).map(PyVariable::from).map_err(lm_error)
}

#[pyfunction]
fn roundtrip_model_specs(value: PyModelSpecs) -> PyModelSpecs {
    value
}

#[pyfunction]
fn new_model_specs(max_degree: usize) -> PyResult<PyModelSpecs> {
    rs_lunamodel::Specs::new(None, None, None, Some(max_degree), None, None)
        .map(PyModelSpecs::from)
        .map_err(lm_error)
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
fn evaluate_model(model: PyModel, solution: PySolution) -> PyResult<PySolution> {
    model
        .evaluate_solution(&solution)
        .map(PySolution::from)
        .map_err(lm_error)
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

/// Reads the higher-order storage in Rust, so the host's data is actually
/// interpreted by the extension (a plain roundtrip only moves the handle).
#[pyfunction]
fn expression_higher_order_items(
    py: Python<'_>,
    value: PyExpression,
) -> PyResult<Vec<(Vec<u32>, f64)>> {
    let mut items = py
        .detach(|| value.raw_higher_order_items())
        .map_err(lm_error)?;
    items.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(items)
}

/// Looks up a higher-order bias by key, exercising the host map's hashing.
#[pyfunction]
fn expression_higher_order_bias(py: Python<'_>, value: PyExpression, vars: Vec<u32>) -> PyResult<f64> {
    py.detach(|| value.higher_order(&vars)).map_err(lm_error)
}

/// Computes a new expression in Rust from the host's expression.
#[pyfunction]
fn expression_scaled_higher_order_items(
    py: Python<'_>,
    value: PyExpression,
    factor: f64,
) -> PyResult<Vec<(Vec<u32>, f64)>> {
    let mut items = py
        .detach(|| (value.inner() * factor)?.raw_higher_order_items())
        .map_err(lm_error)?;
    items.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(items)
}

/// Raises the Python exception `into_pyerr` maps an error of `kind` to.
#[pyfunction]
fn raise_error_kind(kind: String) -> PyResult<()> {
    let error = rs_lunamodel::Error::new(rs_lunamodel::Status::Errored, "boom").with_kind(kind);
    Err(into_pyerr(error))
}

#[pymodule]
fn rs_lunamodel_integration_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // The ABI this extension was built for, readable without initializing rs-lunamodel (which
    // fails on a host of another ABI); the test session compares it with the host's.
    m.add("C_API_ABI_VERSION", rs_lunamodel::CAPI_ABI_VERSION)?;
    m.add_function(wrap_pyfunction!(roundtrip_bounds, m)?)?;
    m.add_function(wrap_pyfunction!(new_bounds, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_comparator, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_constraint, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_constraint_collection, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_ctype, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_environment, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_expression, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_model, m)?)?;
    m.add_function(wrap_pyfunction!(new_model, m)?)?;
    m.add_function(wrap_pyfunction!(model_environment, m)?)?;
    m.add_function(wrap_pyfunction!(model_variable, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_model_specs, m)?)?;
    m.add_function(wrap_pyfunction!(new_model_specs, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_sense, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_solution, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_model, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_translation_target, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_unbounded, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_value_source, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_variable, m)?)?;
    m.add_function(wrap_pyfunction!(roundtrip_vtype, m)?)?;
    m.add_function(wrap_pyfunction!(expression_higher_order_items, m)?)?;
    m.add_function(wrap_pyfunction!(expression_higher_order_bias, m)?)?;
    m.add_function(wrap_pyfunction!(expression_scaled_higher_order_items, m)?)?;
    m.add_function(wrap_pyfunction!(raise_error_kind, m)?)?;

    m.add_class::<HeldModel>()?;
    m.add_class::<HeldSolution>()?;
    m.add_class::<HeldEnvironment>()?;
    m.add_class::<HeldExpression>()?;
    m.add_class::<HeldConstraintCollection>()?;
    m.add_class::<HeldConstraint>()?;

    Ok(())
}
