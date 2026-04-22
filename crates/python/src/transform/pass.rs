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

macro_rules! define_py_pass {
    (
        analysis: [$(($analysis_py:ident, $analysis_rs:path)),* $(,)?],
        transformation: [$(($transformation_py:ident, $transformation_rs:path)),* $(,)?],
        control_flow: [$(($control_flow_py:ident, $control_flow_rs:path)),* $(,)?],
    ) => {
        #[derive(FromPyObject)]
        pub enum PyPass {
            // ////////////////////////
            // ///      BUILTIN     ///
            // ////////////////////////
            $($analysis_py(Py<$analysis_py>),)*
            $($transformation_py(Py<$transformation_py>),)*
            $($control_flow_py(Py<$control_flow_py>),)*
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
                    $(
                        Self::$analysis_py(p) => {
                            Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).0.clone())))
                        }
                    )*
                    $(
                        Self::$transformation_py(p) => {
                            Ok(PipelineStep::Transform(Arc::new(p.borrow(py).0.clone())))
                        }
                    )*
                    $(
                        Self::$control_flow_py(p) => {
                            Ok(PipelineStep::ControlFlow(Arc::new(p.borrow(py).0.clone())))
                        }
                    )*
                    // known pipelines
                    Self::ToBinaryMin(p) => {
                        let pipe = p.borrow(py).0.clone();
                        Ok(PipelineStep::Pipeline(Arc::new(pipe)))
                    }
                    Self::ToUnconsBin(p) => {
                        let pipe = p.borrow(py).0.clone();
                        Ok(PipelineStep::Pipeline(Arc::new(pipe)))
                    }
                    // special container
                    Self::Pipeline(p) => Ok(PipelineStep::Pipeline(Arc::new(p.0.clone()))),
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
            }

            pub fn from_step(py: Python, step: &PipelineStep) -> LunaModelResult<Py<PyAny>> {
                match step {
                    PipelineStep::Analysis(p) => {
                        $(
                            if let Some(a) = p.as_any().downcast_ref::<$analysis_rs>() {
                                return Py::new(py, $analysis_py(a.clone()))
                                    .map_err(map_pyerr)?
                                    .into_py_any(py)
                                    .map_err(map_pyerr);
                            }
                        )*
                        if let Some(a) = p.as_any().downcast_ref::<PyAnalysisPassAdapter>() {
                            return a.inner(py).into_py_any(py).map_err(map_pyerr);
                        }
                        Err(LunaModelError::Compilation(
                            format!(
                                "cannot convert analysis pass '{}' to a python pass.",
                                p.name()
                            )
                            .into(),
                        ))
                    }
                    PipelineStep::Transform(p) => {
                        $(
                            if let Some(t) = p.as_any().downcast_ref::<$transformation_rs>() {
                                return Py::new(py, $transformation_py(t.clone()))
                                    .map_err(map_pyerr)?
                                    .into_py_any(py)
                                    .map_err(map_pyerr);
                            }
                        )*
                        if let Some(t) = p.as_any().downcast_ref::<PyTransformationPassAdapter>() {
                            return t.inner(py).into_py_any(py).map_err(map_pyerr);
                        }
                        Err(LunaModelError::Compilation(
                            format!(
                                "cannot convert transformation pass '{}' to a python pass.",
                                p.name()
                            )
                            .into(),
                        ))
                    }
                    PipelineStep::ControlFlow(p) => {
                        $(
                            if let Some(c) = p.as_any().downcast_ref::<$control_flow_rs>() {
                                return Py::new(py, $control_flow_py(c.clone()))
                                    .map_err(map_pyerr)?
                                    .into_py_any(py)
                                    .map_err(map_pyerr);
                            }
                        )*
                        if let Some(c) = p.as_any().downcast_ref::<PyControlFlowPassAdapter>() {
                            return c.inner(py).into_py_any(py).map_err(map_pyerr);
                        }
                        Err(LunaModelError::Compilation(
                            format!(
                                "cannot convert control-flow pass '{}' to a python pass.",
                                p.name()
                            )
                            .into(),
                        ))
                    }
                    PipelineStep::Pipeline(p) => PyPipeline(Pipeline::clone(&p))
                        .into_py_any(py)
                        .map_err(map_pyerr),
                    PipelineStep::MetaAnalysis(p) => {
                        if let Some(m) = p.as_any().downcast_ref::<PyMetaAnalysisPassAdapter>() {
                            return m.inner(py).into_py_any(py).map_err(map_pyerr);
                        }
                        Err(LunaModelError::Compilation(
                            format!(
                                "cannot convert meta-analysis pass '{}' to a python pass.",
                                p.name()
                            )
                            .into(),
                        ))
                    }
                    PipelineStep::Composite(p) => {
                        if let Some(c) = p.as_any().downcast_ref::<PyCompositePassAdapter>() {
                            return c.inner(py).into_py_any(py).map_err(map_pyerr);
                        }
                        Err(LunaModelError::Compilation(
                            format!(
                                "cannot convert composite pass '{}' to a python pass.",
                                p.name()
                            )
                            .into(),
                        ))
                    }
                }
            }
        }
    };
}

define_py_pass!(
    analysis: [
        (PyCheckModelSpecsAnalysis, CheckModelSpecsAnalysis),
        (PyMaxBiasAnalysis, MaxBiasAnalysis),
        (PyMinValueForConstraintAnalysis, MinValueForConstraintAnalysis),
        (PySpecsAnalysis, SpecsAnalysis),
    ],
    transformation: [
        (PyBinarySpinPass, BinarySpinPass),
        (PyChangeSensePass, ChangeSensePass),
        (PyEqualityConstraintsToQuadraticPenaltyPass, EqualityConstraintsToQuadraticPenaltyPass),
        (PyGeToLeConstraintsPass, GeToLeConstraintsPass),
        (PyIntegerToBinaryPass, IntegerToBinaryPass),
        (PyLeToEqConstraintsPass, LeToEqConstraintsPass),
        (PyReduceInvertedBinaryPass, ReduceInvertedBinaryPass),
    ],
    control_flow: [(PyIfElsePass, IfElsePass)],
);

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
