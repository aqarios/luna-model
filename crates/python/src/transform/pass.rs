use std::sync::Arc;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transform::{
    analysis::{
        CheckModelSpecsAnalysis, MaxBiasAnalysis, MinValueForConstraintAnalysis, SpecsAnalysis,
    },
    control_flow::IfElsePass,
    transformation::{
        BinarySpinPass, ChangeSensePass, EqualityConstraintsToQuadraticPenaltyPass,
        GeToLeConstraintsPass, IntegerToBinaryPass, LeToEqConstraintsPass,
        ReduceInvertedBinaryPass,
    },
};
use lunamodel_transpiler::{Pipeline, PipelineStep};
use pyo3::{
    FromPyObject, IntoPyObjectExt, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyTypeError,
    types::{PyAnyMethods, PyTypeMethods},
};

use crate::transform::{
    PyControlFlowPass, PyControlFlowPassAdapter, PyMetaAnalysisPassAdapter, PyPipeline,
    PyTransformationPassAdapter,
    adapter::{
        PyAnalysisPass, PyAnalysisPassAdapter, PyCompositePass, PyCompositePassAdapter,
        PyMetaAnalysisPass, PyTransformationPass,
    },
    builtin::{
        analysis::{
            PyCheckModelSpecsAnalysis, PyMaxBiasAnalysis, PyMinValueForConstraintAnalysis,
            PySpecsAnalysis,
        },
        control_flow::PyIfElsePass,
        pipeline::{PyToBinaryMinimizationPipeline, PyToUnconstrainedBinaryPipeline},
        transformation::{
            PyBinarySpinPass, PyChangeSensePass, PyEqualityConstraintsToQuadraticPenaltyPass,
            PyGeToLeConstraintsPass, PyIntegerToBinaryPass, PyLeToEqConstraintsPass,
            PyReduceInvertedBinaryPass,
        },
    },
    utils::map_pyerr,
};

#[derive(FromPyObject)]
pub enum PyPass {
    // ////////////////////////
    // ///      BUILTIN     ///
    // ////////////////////////
    // analysis
    CheckSpecs(Py<PyCheckModelSpecsAnalysis>),
    MaxBias(Py<PyMaxBiasAnalysis>),
    MinValInConstr(Py<PyMinValueForConstraintAnalysis>),
    Specs(Py<PySpecsAnalysis>),
    // transformation
    BinSpin(Py<PyBinarySpinPass>),
    ChangeSense(Py<PyChangeSensePass>),
    EqConstrToQuadPen(Py<PyEqualityConstraintsToQuadraticPenaltyPass>),
    GeToLe(Py<PyGeToLeConstraintsPass>),
    IntToBin(Py<PyIntegerToBinaryPass>),
    LeToEq(Py<PyLeToEqConstraintsPass>),
    RedInvBin(Py<PyReduceInvertedBinaryPass>),
    // control-flow
    IfElse(Py<PyIfElsePass>),
    // known pipelines
    ToBinaryMin(Py<PyToBinaryMinimizationPipeline>),
    ToUnconsBin(Py<PyToUnconstrainedBinaryPipeline>),
    // special containers
    Pipeline(PyPipeline),
    // ///////////////////////////
    // /// CUSTOM FROM PYTHON  ///
    // ///////////////////////////
    // custom control-flow from python
    CustomControlFlow(Py<PyControlFlowPass>),
    // custom transformation from python
    CustomTransformation(Py<PyTransformationPass>),
    // custom analysis from python
    CustomAnalysis(Py<PyAnalysisPass>),
    // custom meta-analysis from python
    CustomMetaAnalysis(Py<PyMetaAnalysisPass>),
    // custom composite from python
    CustomComposite(Py<PyCompositePass>),
    // fallback for non-leaking error.
    Default(Py<PyAny>),
}

impl PyPass {
    pub fn to_step(&self, py: Python) -> PyResult<PipelineStep> {
        match self {
            // ////////////////////////
            // ///      BUILTIN     ///
            // ////////////////////////
            // analysis
            Self::CheckSpecs(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::MaxBias(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::MinValInConstr(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::Specs(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            // transformation
            Self::BinSpin(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::ChangeSense(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::EqConstrToQuadPen(p) => {
                Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs())))
            }
            Self::GeToLe(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::IntToBin(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::LeToEq(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::RedInvBin(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            // control-flow
            Self::IfElse(p) => Ok(PipelineStep::ControlFlow(Arc::new(p.borrow(py).to_rs()))),
            // known pipelines
            Self::ToBinaryMin(p) => {
                let pipe = p.borrow(py).clone();
                Ok(PipelineStep::Pipeline(Arc::new(pipe)))
            }
            Self::ToUnconsBin(p) => {
                let pipe = p.borrow(py).0.clone();
                Ok(PipelineStep::Pipeline(Arc::new(pipe)))
            }
            // special container
            Self::Pipeline(p) => Ok(PipelineStep::Pipeline(Arc::new(p.0.clone()))),
            // ///////////////////////////
            // /// CUSTOM FROM PYTHON  ///
            // ///////////////////////////
            // custom control flow from python.
            Self::CustomControlFlow(p) => Ok(PipelineStep::ControlFlow(Arc::new(
                PyControlFlowPassAdapter::new(py, p.clone_ref(py))?,
            ))),
            // custom transformation from python.
            Self::CustomTransformation(p) => Ok(PipelineStep::Transform(Arc::new(
                PyTransformationPassAdapter::new(py, p.clone_ref(py))?,
            ))),
            // custom analysis from python
            Self::CustomAnalysis(p) => Ok(PipelineStep::Analysis(Arc::new(
                PyAnalysisPassAdapter::new(py, p.clone_ref(py))?,
            ))),
            // custom meta-analysis from python
            Self::CustomMetaAnalysis(p) => Ok(PipelineStep::MetaAnalysis(Arc::new(
                PyMetaAnalysisPassAdapter::new(py, p.clone_ref(py))?,
            ))),
            // custom composite from python
            Self::CustomComposite(p) => Ok(PipelineStep::Composite(Arc::new(
                PyCompositePassAdapter::new(py, p.clone_ref(py))?,
            ))),
            // default for non-leaking error
            Self::Default(d) => Err(invalid_pass_error(d, py)),
        }
        // self.inner.clone()
    }

    pub fn from_step(py: Python, step: &PipelineStep) -> LunaModelResult<Py<PyAny>> {
        match step {
            PipelineStep::Analysis(p) => {
                if let Some(a) = p.as_any().downcast_ref::<CheckModelSpecsAnalysis>() {
                    return Py::new(py, PyCheckModelSpecsAnalysis(a.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(a) = p.as_any().downcast_ref::<MaxBiasAnalysis>() {
                    return Py::new(py, PyMaxBiasAnalysis(a.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(a) = p.as_any().downcast_ref::<MinValueForConstraintAnalysis>() {
                    return Py::new(py, PyMinValueForConstraintAnalysis(a.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(a) = p.as_any().downcast_ref::<SpecsAnalysis>() {
                    return Py::new(py, PySpecsAnalysis(a.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(a) = p.as_any().downcast_ref::<PyAnalysisPassAdapter>() {
                    return a.inner(py).into_py_any(py).map_err(map_pyerr);
                }
                return Err(LunaModelError::Compilation(
                    format!(
                        "cannot convert analysis pass '{}' to a python pass.",
                        p.name()
                    )
                    .into(),
                ));
            }
            PipelineStep::Transform(p) => {
                if let Some(t) = p.as_any().downcast_ref::<BinarySpinPass>() {
                    return Py::new(py, PyBinarySpinPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<ChangeSensePass>() {
                    return Py::new(py, PyChangeSensePass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p
                    .as_any()
                    .downcast_ref::<EqualityConstraintsToQuadraticPenaltyPass>()
                {
                    return Py::new(py, PyEqualityConstraintsToQuadraticPenaltyPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<GeToLeConstraintsPass>() {
                    return Py::new(py, PyGeToLeConstraintsPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<IntegerToBinaryPass>() {
                    return Py::new(py, PyIntegerToBinaryPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<LeToEqConstraintsPass>() {
                    return Py::new(py, PyLeToEqConstraintsPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<ReduceInvertedBinaryPass>() {
                    return Py::new(py, PyReduceInvertedBinaryPass(t.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(t) = p.as_any().downcast_ref::<PyTransformationPassAdapter>() {
                    return t.inner(py).into_py_any(py).map_err(map_pyerr);
                }
                return Err(LunaModelError::Compilation(
                    format!(
                        "cannot convert transformation pass '{}' to a python pass.",
                        p.name()
                    )
                    .into(),
                ));
            }
            PipelineStep::ControlFlow(p) => {
                if let Some(c) = p.as_any().downcast_ref::<IfElsePass>() {
                    return Py::new(py, PyIfElsePass(c.clone()))
                        .map_err(map_pyerr)?
                        .into_py_any(py)
                        .map_err(map_pyerr);
                }
                if let Some(c) = p.as_any().downcast_ref::<PyControlFlowPassAdapter>() {
                    return c.inner(py).into_py_any(py).map_err(map_pyerr);
                }
                return Err(LunaModelError::Compilation(
                    format!(
                        "cannot convert control-flow pass '{}' to a python pass.",
                        p.name()
                    )
                    .into(),
                ));
            }
            PipelineStep::Pipeline(p) => {
                return PyPipeline(Pipeline::clone(&p))
                    .into_py_any(py)
                    .map_err(map_pyerr);
            }
            PipelineStep::MetaAnalysis(p) => {
                if let Some(m) = p.as_any().downcast_ref::<PyMetaAnalysisPassAdapter>() {
                    return m.inner(py).into_py_any(py).map_err(map_pyerr);
                }
                return Err(LunaModelError::Compilation(
                    format!(
                        "cannot convert meta-analysis pass '{}' to a python pass.",
                        p.name()
                    )
                    .into(),
                ));
            }
            PipelineStep::Composite(p) => {
                if let Some(c) = p.as_any().downcast_ref::<PyCompositePassAdapter>() {
                    return c.inner(py).into_py_any(py).map_err(map_pyerr);
                }
                return Err(LunaModelError::Compilation(
                    format!(
                        "cannot convert composite pass '{}' to a python pass.",
                        p.name()
                    )
                    .into(),
                ));
            }
        }
    }
}

fn invalid_pass_error(obj: &Py<PyAny>, py: Python<'_>) -> PyErr {
    let bound = obj.bind(py);

    let type_name = bound
        .get_type()
        .name()
        .map(|n| n.to_string())
        .unwrap_or_else(|_| "<unknown>".to_string());

    let module = bound
        .getattr("__class__")
        .and_then(|c| c.getattr("__module__"))
        .and_then(|m| m.extract::<String>())
        .unwrap_or_else(|_| "<unknown>".to_string());

    PyTypeError::new_err(format!(
        "Invalid pass object of type '{module}.{type_name}'. Expected 
Pass/AnalysisPass/TransformationPass/ControlFlowPass."
    ))
}
