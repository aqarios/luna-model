use std::sync::Arc;

use lunamodel_transpiler::PipelineStep;
use pyo3::{
    FromPyObject, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyTypeError,
    types::{PyAnyMethods, PyTypeMethods},
};

use super::builtin::{
    analysis::{
        PyCheckModelSpecsAnalysis, PyMaxBiasAnalysis, PyMinValueForConstraintAnalysis,
        PySpecsAnalysis,
    },
    transformation::PyIntegerToBinaryPass,
};
use crate::transformv2::{
    PyControlFlowPass, PyControlFlowPassAdapter, PyPipeline, PyTransformationPassAdapter,
    adapter::{PyAnalysisPass, PyAnalysisPassAdapter, PyTransformationPass},
    builtin::{
        control_flow::PyIfElsePass,
        transformation::{
            PyBinarySpinPass, PyChangeSensePass, PyEqualityConstraintsToQuadraticPenaltyPass,
            PyGeToLeConstraintsPass, PyLeToEqConstraintsPass,
        },
    },
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
    // control-flow
    IfElse(Py<PyIfElsePass>),
    // special containers
    Pipeline(PyPipeline),
    // ///////////////////////////
    // /// CUSTOM FROM PYTHON  ///
    // ///////////////////////////
    // custom control-flow from python
    ControlFlow(Py<PyControlFlowPass>),
    // custom transformation from python
    CustomTransformation(Py<PyTransformationPass>),
    // custom analysis from python
    CustomAnalysis(Py<PyAnalysisPass>),
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
            // control-flow
            Self::IfElse(p) => Ok(PipelineStep::ControlFlow(Arc::new(p.borrow(py).to_rs()))),
            // special container
            Self::Pipeline(p) => Ok(PipelineStep::Pipeline {
                name: p.name(),
                passes: p.steps().to_vec(),
            }),
            // ///////////////////////////
            // /// CUSTOM FROM PYTHON  ///
            // ///////////////////////////
            // custom control flow from python.
            Self::ControlFlow(p) => Ok(PipelineStep::ControlFlow(Arc::new(
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
            // default for non-leaking error
            Self::Default(d) => Err(invalid_pass_error(d, py)),
        }
        // self.inner.clone()
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
