use crate::py_bindings::unwind;
use crate::core::Qubo;
use crate::py_bindings::py_model::PyModel;
use crate::{core::Vtype, translator::MatrixTranslator};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods, ToPyArray};
use pyo3::prelude::*;
use unwind_macros::unwindable;

/// A wrapper around qubo matrices that holds all relevant metadata, e.g., the model offset.
#[cfg_attr(not(feature = "lq"), pyclass(name = "Qubo", module = "aqmodels._core.translator"))]
#[cfg_attr(feature = "lq",      pyclass(name = "Qubo", module = "luna_quantum._core.translator"))]
#[derive(Deref, DerefMut)]
pub struct PyQubo(pub Qubo);

impl Into<Qubo> for PyQubo {
    fn into(self) -> Qubo {
        self.0
    }
}

#[unwindable]
#[pymethods]
impl PyQubo {
    /// The actual QUBO matrix.
    ///
    /// Returns
    /// -------
    /// NDArray
    ///     A square NumPy array representing the QUBO matrix derived from
    ///     the model's objective.
    #[getter(matrix)]
    fn get_np_matrix<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self
            .matrix_flat
            .to_pyarray(py)
            .reshape((self.num_variables.into(), self.num_variables.into()))?)
    }

    /// The name of the variables in the same order as in the QUBO matrix.
    ///
    /// Returns
    /// -------
    /// list[Variable]
    ///     The variable names in the order they appear in the QUBO.
    #[getter]
    fn get_variable_names(&self) -> Vec<String> {
        self.variable_names.clone()
    }

    /// The name of the model the QUBO matrix was generated from.
    ///
    /// Returns
    /// -------
    /// str
    ///     The model name.
    #[getter]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// The constant offset of the original model passed to the QuboTranslator.
    ///
    /// Returns
    /// -------
    /// float
    ///     The constant offset of the model.
    #[getter]
    fn get_offset(&self) -> f64 {
        self.offset
    }

    /// The type of the model variables. Can be `Binary` or `Spin`.
    ///
    /// Returns
    /// -------
    /// Vtype
    ///     The variable type.
    #[getter]
    fn get_vtype(&self) -> Vtype {
        self.vtype
    }
}

/// Utility class for converting between dense QUBO matrices and symbolic models.
///
/// `QuboTranslator` provides methods to:
/// - Convert a NumPy-style QUBO matrix into a symbolic `Model`
/// - Convert a `Model` (with quadratic objective) into a dense QUBO matrix
///
/// These conversions are especially useful when interacting with external solvers
/// or libraries that operate on matrix-based problem definitions.
///
/// Examples
/// --------
/// >>> import numpy as np
/// >>> from luna_quantum import QuboTranslator, Vtype
/// >>> q = np.array([[1.0, -1.0], [-1.0, 2.0]])
///
/// Create a model from a matrix:
///
/// >>> model = QuboTranslator.to_aq(q, offset=4.2, name="qubo_model", vtype=Vtype.Binary)
///
/// Convert it back to a dense matrix:
///
/// >>> recovered = QuboTranslator.from_aq(model)
/// >>> assert np.allclose(q, recovered.matrix)
#[cfg_attr(not(feature = "lq"), pyclass(name = "QuboTranslator", module = "aqmodels._core.translator"))]
#[cfg_attr(feature = "lq",      pyclass(name = "QuboTranslator", module = "luna_quantum._core.translator"))]
pub struct PyQuboTranslator {}

#[derive(FromPyObject)]
enum QuboType<'py> {
    F64(PyReadonlyArray2<'py, f64>),
    I64(PyReadonlyArray2<'py, i64>),
}

#[unwindable]
#[pymethods]
impl PyQuboTranslator {
    /// Convert a dense QUBO matrix into a symbolic `Model`.
    ///
    /// Parameters
    /// ----------
    /// qubo : NDArray
    ///     A square 2D NumPy array representing the QUBO matrix.
    ///     Diagonal entries correspond to linear coefficients;
    ///     off-diagonal entries represent pairwise quadratic terms.
    /// name : str, optional
    ///     An optional name to assign to the resulting model.
    /// vtype : Vtype, optional
    ///     The variable type to assign to all variables (e.g. Binary, Spin).
    ///
    /// Returns
    /// -------
    /// Model
    ///     A symbolic model representing the given QUBO structure.
    ///
    /// Raises
    /// ------
    /// TranslationError
    ///     Generally if the translation fails. Might be specified by the following
    ///     error.
    /// VariableNamesError
    ///     If a list of variable names is provided but contains duplicates or has an
    ///     incorrect length.
    #[staticmethod]
    #[pyo3(signature=(qubo, offset=None, variable_names=None, name=None, vtype=None))]
    fn to_aq(
        qubo: QuboType,
        offset: Option<f64>,
        variable_names: Option<Vec<String>>,
        name: Option<String>,
        vtype: Option<Vtype>,
    ) -> PyResult<PyModel> {
        let (dense, var_num): (&[f64], usize) = match qubo {
            QuboType::F64(q) => (
                &q.as_slice()
                    .expect("failed to convert to slice")
                    .iter()
                    .map(|&v| v)
                    .collect::<Vec<f64>>(),
                q.shape()[0],
            ),
            QuboType::I64(q) => (
                &q.as_slice()
                    .expect("failed to convert to slice")
                    .iter()
                    .map(|&v| v as f64)
                    .collect::<Vec<f64>>(),
                q.shape()[0],
            ),
        };
        Ok(PyModel::new(MatrixTranslator::model_from_dense(
            name,
            dense,
            var_num.into(),
            vtype,
            offset,
            variable_names,
        )?))
    }

    /// Convert a symbolic model to a dense QUBO matrix representation.
    ///
    /// Parameters
    /// ----------
    /// model : Model
    ///     The symbolic model to convert. The objective must be quadratic-only
    ///     and unconstrained.
    ///
    /// Returns
    /// -------
    /// Qubo
    ///     An object representing a QUBO with information additional to the square NumPy array
    ///     representing the QUBO matrix derived from the model's objective. This object also
    ///     includes the `variable_ordering` as well as the `offset` of the original model.
    ///
    /// Raises
    /// ------
    /// TranslationError
    ///     Generally if the translation fails. Might be specified by one of the
    ///     four following errors.
    /// ModelNotQuadraticError
    ///     If the objective contains higher-order (non-quadratic) terms.
    /// ModelNotUnconstrainedError
    ///     If the model contains any constraints.
    /// ModelSenseNotMinimizeError
    ///     If the model's optimization sense is 'maximize'.
    /// ModelVtypeError
    ///     If the model contains different vtypes or vtypes other than binary and
    ///     spin.
    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq(model: &PyModel) -> PyResult<PyQubo> {
        let qubo = MatrixTranslator::model_to_dense(&model.access())?;
        Ok(PyQubo(qubo))
    }
}
