use lunamodel_python::translate::model::*;
pub use lunamodel_python::*;
use pyo3::{PyTypeInfo, prelude::*};

#[pymodule]
fn _lm(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(quicksum, m)?)?;
    // Enums from lunamodel-core
    m.add_class::<PyValueSource>()?;
    // Enums from lunamodel-types
    m.add_class::<PyVtype>()?;
    m.add_class::<PySense>()?;
    m.add_class::<PyCtype>()?;
    m.add_class::<PyComparator>()?;
    // Enums from lunamodel-translate
    m.add_class::<PyTranslationTarget>()?;
    // extra
    m.add_class::<PyUnbounded>()?;
    // Classes
    m.add_class::<PyExpression>()?;
    m.add_class::<PyVariable>()?;
    m.add_class::<PyEnvironment>()?;
    m.add_class::<PyBounds>()?;
    m.add_class::<PyModel>()?;
    m.add_class::<PyModelMetadata>()?;
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
    m.add_class::<PyMpsTranslator>()?;
    m.add_class::<PyBqmTranslator>()?;
    m.add_class::<PyQuboTranslator>()?;
    m.add_class::<PyQubo>()?;

    // Transformations
    m.add_class::<transform::PyPassManager>()?;
    m.add_class::<transform::PyTransformationOutput>()?;
    m.add_class::<transform::PyTransformationRecord>()?;
    m.add_class::<transform::PyPassContext>()?;
    m.add_class::<transform::PyTransformationPass>()?;
    m.add_class::<transform::PyAnalysisPass>()?;
    m.add_class::<transform::PyControlFlowPass>()?;
    m.add_class::<transform::PyControlFlowPlan>()?;
    m.add_class::<transform::PyPipeline>()?;
    m.add_class::<transform::PyPassEntry>()?;
    m.add_class::<transform::PyCompositePass>()?;
    m.add_class::<transform::PyMetaAnalysisPass>()?;
    // builtin analysis
    m.add_class::<transform::builtin::analysis::PyCheckModelSpecsAnalysis>()?;
    m.add_class::<transform::builtin::analysis::PyMaxBiasAnalysis>()?;
    m.add_class::<transform::builtin::analysis::PyMaxBias>()?;
    m.add_class::<transform::builtin::analysis::PyMinValueForConstraintAnalysis>()?;
    m.add_class::<transform::builtin::analysis::PyMinConstraintValues>()?;
    m.add_class::<transform::builtin::analysis::PySpecsAnalysis>()?;
    // builtin transformation
    m.add_class::<transform::builtin::transformation::PyIntegerToBinaryPass>()?;
    m.add_class::<transform::builtin::transformation::PyBinarySpinPass>()?;
    m.add_class::<transform::builtin::transformation::PyChangeSensePass>()?;
    m.add_class::<transform::builtin::transformation::PyEqualityConstraintsToQuadraticPenaltyPass>(
    )?;
    m.add_class::<transform::builtin::transformation::PyGeToLeConstraintsPass>()?;
    m.add_class::<transform::builtin::transformation::PyLeToEqConstraintsPass>()?;
    m.add_class::<transform::builtin::transformation::PyReduceInvertedBinaryPass>()?;
    // builtin control flow
    m.add_class::<transform::builtin::control_flow::PyIfElsePass>()?;
    // builtin composite
    // builtin meta-analysis
    // builtin pipelines
    m.add_class::<transform::builtin::pipeline::PyToUnconstrainedBinaryPipeline>()?;
    m.add_class::<transform::builtin::pipeline::PyToBinaryMinimizationPipeline>()?;

    transform::register_backward();

    // Errors
    m.add(
        PyLunaModelError::type_object(py).name()?,
        m.py().get_type::<PyLunaModelError>(),
    )?;
    m.add(
        PyUnsupportedOperationError::type_object(py).name()?,
        m.py().get_type::<PyUnsupportedOperationError>(),
    )?;
    m.add(
        PyCompressionError::type_object(py).name()?,
        m.py().get_type::<PyCompressionError>(),
    )?;
    m.add(
        PyInternalPanicError::type_object(py).name()?,
        m.py().get_type::<PyInternalPanicError>(),
    )?;
    m.add(
        PyComputationError::type_object(py).name()?,
        m.py().get_type::<PyComputationError>(),
    )?;
    m.add(
        PyDuplicateConstraintNameError::type_object(py).name()?,
        m.py().get_type::<PyDuplicateConstraintNameError>(),
    )?;
    m.add(
        PyVariableOutOfRangeError::type_object(py).name()?,
        m.py().get_type::<PyVariableOutOfRangeError>(),
    )?;
    m.add(
        PyVariableExistsError::type_object(py).name()?,
        m.py().get_type::<PyVariableExistsError>(),
    )?;
    m.add(
        PyVariableNotExistingError::type_object(py).name()?,
        m.py().get_type::<PyVariableNotExistingError>(),
    )?;
    m.add(
        PyVariableCreationError::type_object(py).name()?,
        m.py().get_type::<PyVariableCreationError>(),
    )?;
    m.add(
        PyVariablesFromDifferentEnvsError::type_object(py).name()?,
        m.py().get_type::<PyVariablesFromDifferentEnvsError>(),
    )?;
    m.add(
        PyDifferentEnvsError::type_object(py).name()?,
        m.py().get_type::<PyDifferentEnvsError>(),
    )?;
    m.add(
        PyNoActiveEnvironmentFoundError::type_object(py).name()?,
        m.py().get_type::<PyNoActiveEnvironmentFoundError>(),
    )?;
    m.add(
        PyMultipleActiveEnvironmentsError::type_object(py).name()?,
        m.py().get_type::<PyMultipleActiveEnvironmentsError>(),
    )?;
    m.add(
        PyDecodeError::type_object(py).name()?,
        m.py().get_type::<PyDecodeError>(),
    )?;
    m.add(
        PyIllegalConstraintNameError::type_object(py).name()?,
        m.py().get_type::<PyIllegalConstraintNameError>(),
    )?;
    m.add(
        PyTranslationError::type_object(py).name()?,
        m.py().get_type::<PyTranslationError>(),
    )?;
    m.add(
        PyModelNotQuadraticError::type_object(py).name()?,
        m.py().get_type::<PyModelNotQuadraticError>(),
    )?;
    m.add(
        PyModelNotUnconstrainedError::type_object(py).name()?,
        m.py().get_type::<PyModelNotUnconstrainedError>(),
    )?;
    m.add(
        PyModelSenseNotMinimizeError::type_object(py).name()?,
        m.py().get_type::<PyModelSenseNotMinimizeError>(),
    )?;
    m.add(
        PyModelVtypeError::type_object(py).name()?,
        m.py().get_type::<PyModelVtypeError>(),
    )?;
    m.add(
        PyVariableNamesError::type_object(py).name()?,
        m.py().get_type::<PyVariableNamesError>(),
    )?;
    m.add(
        PyEvaluationError::type_object(py).name()?,
        m.py().get_type::<PyEvaluationError>(),
    )?;
    m.add(
        PySolutionTranslationError::type_object(py).name()?,
        m.py().get_type::<PySolutionTranslationError>(),
    )?;
    m.add(
        PySampleIncorrectLengthError::type_object(py).name()?,
        m.py().get_type::<PySampleIncorrectLengthError>(),
    )?;
    m.add(
        PySampleUnexpectedVariableError::type_object(py).name()?,
        m.py().get_type::<PySampleUnexpectedVariableError>(),
    )?;
    m.add(
        PySampleIncompatibleVtypeError::type_object(py).name()?,
        m.py().get_type::<PySampleIncompatibleVtypeError>(),
    )?;
    m.add(
        PyStartCannotBeInferredError::type_object(py).name()?,
        m.py().get_type::<PyStartCannotBeInferredError>(),
    )?;
    m.add(
        PySampleColCreationError::type_object(py).name()?,
        m.py().get_type::<PySampleColCreationError>(),
    )?;
    m.add(
        PyNoConstraintForKeyError::type_object(py).name()?,
        m.py().get_type::<PyNoConstraintForKeyError>(),
    )?;
    m.add(
        PyTransformationError::type_object(py).name()?,
        m.py().get_type::<PyTransformationError>(),
    )?;
    m.add(
        PyTransformationPassError::type_object(py).name()?,
        m.py().get_type::<PyTransformationPassError>(),
    )?;
    m.add(
        PyAnalysisPassError::type_object(py).name()?,
        m.py().get_type::<PyAnalysisPassError>(),
    )?;
    m.add(
        PyIfElsePassError::type_object(py).name()?,
        m.py().get_type::<PyIfElsePassError>(),
    )?;
    m.add(
        PyMetaAnalysisPassError::type_object(py).name()?,
        m.py().get_type::<PyMetaAnalysisPassError>(),
    )?;
    m.add(
        PyCompilationError::type_object(py).name()?,
        m.py().get_type::<PyCompilationError>(),
    )?;
    m.add(
        PyRandomSamplingError::type_object(py).name()?,
        m.py().get_type::<PyRandomSamplingError>(),
    )?;
    m.add(
        PyInvalidToleranceError::type_object(py).name()?,
        m.py().get_type::<PyInvalidToleranceError>(),
    )?;
    Ok(())
}
