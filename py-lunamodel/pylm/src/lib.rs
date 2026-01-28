use lunamodel_python::translate::model::*;
pub use lunamodel_python::*;
use pyo3::{PyTypeInfo, prelude::*};

#[pymodule]
fn _lm(m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(quicksum, m)?)?;
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

    // For ConstraintCollection iteration
    m.add_class::<PyConstraintCollectionIterator>()?;

    // For Expression iteration
    m.add_class::<PyExpressionIterator>()?;
    m.add_class::<PyConstant>()?;
    m.add_class::<PyLinear>()?;
    m.add_class::<PyQuadratic>()?;
    m.add_class::<PyHigherOrder>()?;

    // Model Translator
    m.add_class::<PyLpTranslator>()?;
    m.add_class::<PyBqmTranslator>()?;
    m.add_class::<PyQuboTranslator>()?;
    m.add_class::<PyQubo>()?;

    // Transformations
    // Core classes.
    m.add_class::<transform::PyPassManager>()?;
    m.add_class::<transform::PyPipeline>()?;
    m.add_class::<transform::PyIfElsePass>()?;
    m.add_class::<transform::PyIR>()?;
    m.add_class::<transform::PyAnalysisCache>()?;
    m.add_class::<transform::PyLogElement>()?;
    m.add_class::<transform::ActionType>()?;
    m.add_class::<transform::PyStructuredTransformationOutcome>()?;
    // Abstract base classes.
    m.add_class::<transform::PyTransformationPass>()?;
    m.add_class::<transform::PyAnalysisPass>()?;
    m.add_class::<transform::PyMetaAnalysisPass>()?;
    // Predefnied and implemented Transformations
    m.add_class::<transform::PyChangeSensePass>()?;
    m.add_class::<transform::PyMaxBiasAnalysis>()?;
    m.add_class::<transform::PyBinarySpinPass>()?;

    // Errors
    m.add(
        PyLunaModelError::NAME,
        m.py().get_type::<PyLunaModelError>(),
    )?;
    m.add(
        PyUnsupportedOperationError::NAME,
        m.py().get_type::<PyUnsupportedOperationError>(),
    )?;
    m.add(
        PyCompressionError::NAME,
        m.py().get_type::<PyCompressionError>(),
    )?;
    m.add(
        PyInternalPanicError::NAME,
        m.py().get_type::<PyInternalPanicError>(),
    )?;
    m.add(
        PyComputationError::NAME,
        m.py().get_type::<PyComputationError>(),
    )?;
    m.add(
        PyDuplicateConstraintNameError::NAME,
        m.py().get_type::<PyDuplicateConstraintNameError>(),
    )?;
    m.add(
        PyVariableOutOfRangeError::NAME,
        m.py().get_type::<PyVariableOutOfRangeError>(),
    )?;
    m.add(
        PyVariableExistsError::NAME,
        m.py().get_type::<PyVariableExistsError>(),
    )?;
    m.add(
        PyVariableNotExistingError::NAME,
        m.py().get_type::<PyVariableNotExistingError>(),
    )?;
    m.add(
        PyVariableCreationError::NAME,
        m.py().get_type::<PyVariableCreationError>(),
    )?;
    m.add(
        PyVariablesFromDifferentEnvsError::NAME,
        m.py().get_type::<PyVariablesFromDifferentEnvsError>(),
    )?;
    m.add(
        PyDifferentEnvsError::NAME,
        m.py().get_type::<PyDifferentEnvsError>(),
    )?;
    m.add(
        PyNoActiveEnvironmentFoundError::NAME,
        m.py().get_type::<PyNoActiveEnvironmentFoundError>(),
    )?;
    m.add(
        PyMultipleActiveEnvironmentsError::NAME,
        m.py().get_type::<PyMultipleActiveEnvironmentsError>(),
    )?;
    m.add(PyDecodeError::NAME, m.py().get_type::<PyDecodeError>())?;
    m.add(
        PyIllegalConstraintNameError::NAME,
        m.py().get_type::<PyIllegalConstraintNameError>(),
    )?;
    m.add(
        PyTranslationError::NAME,
        m.py().get_type::<PyTranslationError>(),
    )?;
    m.add(
        PyModelNotQuadraticError::NAME,
        m.py().get_type::<PyModelNotQuadraticError>(),
    )?;
    m.add(
        PyModelNotUnconstrainedError::NAME,
        m.py().get_type::<PyModelNotUnconstrainedError>(),
    )?;
    m.add(
        PyModelSenseNotMinimizeError::NAME,
        m.py().get_type::<PyModelSenseNotMinimizeError>(),
    )?;
    m.add(
        PyModelVtypeError::NAME,
        m.py().get_type::<PyModelVtypeError>(),
    )?;
    m.add(
        PyVariableNamesError::NAME,
        m.py().get_type::<PyVariableNamesError>(),
    )?;
    m.add(
        PyEvaluationError::NAME,
        m.py().get_type::<PyEvaluationError>(),
    )?;
    m.add(
        PySolutionTranslationError::NAME,
        m.py().get_type::<PySolutionTranslationError>(),
    )?;
    m.add(
        PySampleIncorrectLengthError::NAME,
        m.py().get_type::<PySampleIncorrectLengthError>(),
    )?;
    m.add(
        PySampleUnexpectedVariableError::NAME,
        m.py().get_type::<PySampleUnexpectedVariableError>(),
    )?;
    m.add(
        PySampleIncompatibleVtypeError::NAME,
        m.py().get_type::<PySampleIncompatibleVtypeError>(),
    )?;
    m.add(
        PyStartCannotBeInferredError::NAME,
        m.py().get_type::<PyStartCannotBeInferredError>(),
    )?;
    m.add(
        PySampleColCreationError::NAME,
        m.py().get_type::<PySampleColCreationError>(),
    )?;
    m.add(
        PyNoConstraintForKeyError::NAME,
        m.py().get_type::<PyNoConstraintForKeyError>(),
    )?;
    m.add(
        PyTransformationError::NAME,
        m.py().get_type::<PyTransformationError>(),
    )?;
    m.add(
        PyTransformationPassError::NAME,
        m.py().get_type::<PyTransformationPassError>(),
    )?;
    m.add(
        PyAnalysisPassError::NAME,
        m.py().get_type::<PyAnalysisPassError>(),
    )?;
    m.add(
        PyIfElsePassError::NAME,
        m.py().get_type::<PyIfElsePassError>(),
    )?;
    m.add(
        PyMetaAnalysisPassError::NAME,
        m.py().get_type::<PyMetaAnalysisPassError>(),
    )?;
    m.add(
        PyCompilationError::NAME,
        m.py().get_type::<PyCompilationError>(),
    )?;
    Ok(())
}
