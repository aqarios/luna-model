use crate::core::solution::sol::{SampleCol, ShowMetadata};
use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, PrintLayout, RcSolution, Samples, Solution,
    VarAssignment, Vtype,
};
use crate::errors::{SampleIncorrectLengthErr, SampleUnexpectedVariableErr};
use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::py_res::{PyResultIterator, PyResultView};
use crate::py_bindings::py_sample::PySamples;
use crate::py_bindings::py_timing::PyTiming;
use crate::py_bindings::py_var::PyVariable;
use crate::serialization::{
    Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};
use pyo3::IntoPyObjectExt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(pub VarAssignment<ConcreteAssignmentTypes>);

#[derive(Debug, Clone)]
pub enum SampleKey {
    Str(String),
    Var(PyVariable),
}

/// The solution object that is obtained by running an algorihtm.
///
/// The `Solution` class represents a summary of all data obtained from solving a
/// model. It contains samples, i.e., assignments of values to each model variable as
/// returned by the algorithm, metadata about the solution quality, e.g., the objective
/// value, and the runtime of the algorithm.
///
/// A `Solution` can be constructed explicitly using `from_dict`, `from_dicts` or by obtaining a solution
/// from an algorithm or by converting a different solution format with one of the available
/// translators. Note that the latter requires the environment the model was created in.
///
/// Examples
/// --------
/// Basic usage, assuming that the algorithm already returns a `Solution`:
///
/// >>> from luna_quantum import Model, Solution
/// >>> model: Model = ...
/// >>> algorithm = ...
/// >>> solution: Solution = algorithm.run(model)
/// >>> solution.samples
/// [[1, 0, 1], [0, 0, 1]]
///
/// When you have a `dimod.Sampleset` as the raw solution format:
///
/// >>> from luna_quantum.translator import BqmTranslator
/// >>> from luna_quantum import Model, Solution, DwaveTranslator
/// >>> from dimod import SimulatedAnnealingSampler
/// >>> model: Model = ...
/// >>> bqm = BqmTranslator.from_aq(model)
/// >>> sampleset = SimulatedAnnealingSampler().sample(bqm)
/// >>> solution = DwaveTranslator.from_dimod_sample_set(sampleset)
/// >>> solution.samples
/// [[1, 0, 1], [0, 0, 1]]
///
/// Serialization:
///
/// >>> blob = solution.encode()
/// >>> restored = Solution.decode(blob)
/// >>> restored.samples
/// [[1, 0, 1], [0, 0, 1]]
///
/// Notes
/// -----
/// - To ensure metadata like objective values or feasibility, use `model.evaluate(solution)`.
/// - Use `encode()` and `decode()` to serialize and recover solutions.
#[pyclass(unsendable, name = "Solution", module = "aqmodels")]
#[derive(Deref, DerefMut, Debug)]
pub struct PySolution(pub RcSolution<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<RcSolution<ConcreteBias, ConcreteAssignmentTypes>> for PySolution {
    fn into(self) -> RcSolution<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

#[pymethods]
impl PySolution {
    /// Build a `Solution` based on the provided input data. The solution is constructed
    /// based on a column layout of the solution. Let's take the following sample-set with three
    /// samples as an example:
    ///
    /// [ 0  1  -1  3  2.2  1 ]
    /// [ 1  0  -1  6  3.8  0 ]
    /// [ 1  1  +1  2  2.4  0 ]
    ///
    /// Each row encodes a single sample. However, the variable types vary, the first, second, and
    /// last columns all represent a Binary variable (index 0, 1, 5). The third column represents a
    /// variable of type Spin (index 2). The fourth column (index 3), a variable of type Integer and
    /// the fifth column (index 4), a real-valued variable.
    ///
    /// Thus, the `component_types` list is:
    ///
    /// >>> component_types = [Vtype.Binary, Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary]
    ///
    /// Now we can extract all columns for a binary-valued variable and append them to a new list:
    ///
    /// >>> binary_cols = [[0, 1, 1], [1, 0, 1], [1, 0, 0]]
    ///
    /// where the first element in the list represents the first column, the second element the\
    /// second column and the third element the fifth column.
    /// We do the same for the remaining variable types:
    ///
    /// >>> spin_cols = [[-1, -1, +1]]
    /// >>> int_cols = [[3, 6, 2]]
    /// >>> real_cols = [[2.2, 3.8, 2.4]]
    ///
    /// If we know the raw energies, we can construct them as well:
    ///
    /// >>> raw_energies = [-200, -100, +300]
    ///
    /// And finally call the `build` function:
    ///
    /// >>> sol = Solution.build(
    /// ...     component_types,
    /// ...     binary_cols,
    /// ...     spin_cols,
    /// ...     int_cols,
    /// ...     real_cols,
    /// ...     raw_energies,
    /// ...     timing,
    /// ...     counts=[1, 1, 1]
    /// ... )
    /// >>> sol
    ///
    /// In this example, we could also neglect the `counts` as it defaults to `1`
    /// for all samples if not set:
    ///
    /// >>> sol = Solution.build(
    /// ...     component_types,
    /// ...     binary_cols,
    /// ...     spin_cols,
    /// ...     int_cols,
    /// ...     real_cols,
    /// ...     raw_energies,
    /// ...     timing
    /// ... )
    /// >>> sol
    ///
    ///
    /// Parameters
    /// ----------
    /// component_types : list[Vtype]
    ///     The variable type each element in a sample encodes.
    /// binary_cols : list[list[int]], optional
    ///     The data of all binary valued columns. Each inner list encodes a single binary-valued
    ///     column. Required if any element in the `component_types` is `Vtype.Binary`.
    /// spin_cols : list[list[int]], optional
    ///     The data of all spin-valued columns. Each inner list encodes a single spin-valued
    ///     column. Required if any element in the `component_types` is `Vtype.Spin`.
    /// int_cols : list[list[int]], optional
    ///     The data of all integer-valued columns. Each inner list encodes a single integer valued
    ///     column. Required if any element in the `component_types` is `Vtype.Integer`.
    /// real_cols : list[list[float]], optional
    ///     The data of all real-valued columns. Each inner list encodes a single real-valued
    ///     column. Required if any element in the `component_types` is `Vtype.Real`.
    /// raw_energies : list[float, optional], optional
    ///     The data of all real valued columns. Each inner list encodes a single real-valued
    ///     column.
    /// timing : Timing, optional
    ///     The timing data.
    /// counts : list[int], optional
    ///     The number how often each sample in the solution has occurred. By default, 1 for all
    ///     samples.
    ///
    /// Returns
    /// -------
    /// Solution
    ///     The constructed solution
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If a sample column has an incorrect number of samples or if `counts` has
    ///     a length different from the number of samples given.
    #[staticmethod]
    #[pyo3(signature=(component_types, variable_names=None, binary_cols=None, spin_cols=None, int_cols=None, real_cols=None, raw_energies=None, timing=None, counts=None)
    )]
    fn build(
        component_types: Vec<Vtype>,
        variable_names: Option<Vec<String>>,
        binary_cols: Option<Vec<Vec<u8>>>,
        spin_cols: Option<Vec<Vec<i8>>>,
        int_cols: Option<Vec<Vec<i64>>>,
        real_cols: Option<Vec<Vec<f64>>>,
        raw_energies: Option<Vec<Option<f64>>>,
        timing: Option<PyTiming>,
        counts: Option<Vec<usize>>,
    ) -> PyResult<Self> {
        let var_names: Vec<Option<String>> = if let Some(vn) = variable_names {
            if vn.len() != component_types.len() {
                return Err(PyRuntimeError::new_err(format!("length of variable names and length of component types do not match, is: '{}', actual: '{}'", vn.len(), component_types.len())));
            }
            vn.iter().map(|e| Some(e.clone())).collect()
        } else {
            vec![None; component_types.len()]
        };
        // todo! change to numpy arrays instead of vecs.
        // todo! move further down in rust code.
        let mut sol = Solution::default();

        let (mut lb, mut ls, mut li, mut lr) = (0, 0, 0, 0);
        let binary_cols = binary_cols.unwrap_or(Vec::new());
        let spin_cols = spin_cols.unwrap_or(Vec::new());
        let int_cols = int_cols.unwrap_or(Vec::new());
        let real_cols = real_cols.unwrap_or(Vec::new());

        let mut num_samples: Option<usize> = None;
        for (i, ct) in component_types.iter().enumerate() {
            let len = match ct {
                Vtype::Binary => {
                    let bc = binary_cols[lb].clone();
                    let bc_len = bc.len();
                    sol.add_column(SampleCol::Binary(bc));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("b{lb}")));
                    lb += 1;
                    bc_len
                }
                Vtype::Spin => {
                    let sc = spin_cols[ls].clone();
                    let sc_len = sc.len();
                    sol.add_column(SampleCol::Spin(sc));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("s{ls}")));
                    ls += 1;
                    sc_len
                }
                Vtype::Integer => {
                    let ic = int_cols[li].clone();
                    let ic_len = ic.len();
                    sol.add_column(SampleCol::Integer(ic));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("i{li}")));
                    li += 1;
                    ic_len
                }
                Vtype::Real => {
                    let rc = real_cols[lr].clone();
                    let rc_len = rc.len();
                    sol.add_column(SampleCol::Real(rc));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("r{lr}")));
                    lr += 1;
                    rc_len
                }
            };
            if let Some(ns) = num_samples {
                if ns != len {
                    return Err(PyRuntimeError::new_err(format!(
                        "The number of samples does not match for column {i}"
                    )));
                }
            } else {
                num_samples = Some(len)
            }
        }
        sol.n_samples = num_samples.unwrap_or(0);
        if let Some(re) = raw_energies {
            sol.raw_energies = re;
        } else {
            sol.raw_energies = vec![None; sol.n_samples];
        }
        if let Some(no) = counts {
            if no.len() != sol.n_samples {
                return Err(PyRuntimeError::new_err(
                    "counts does not match the number of samples given.",
                ));
            }
            sol.counts = no;
        } else {
            sol.counts = vec![1; sol.n_samples];
        }
        sol.obj_values = vec![None; sol.n_samples];
        sol.constraints = vec![None; sol.n_samples];
        sol.variable_bounds = vec![None; sol.n_samples];
        sol.feasible = vec![None; sol.n_samples];
        sol.timing = timing.and_then(|t| Some(t.0));
        Ok(PySolution(RcSolution(Rc::new(sol))))
    }

    /// Create a `Solution` from a dict that maps variables or variable names to their
    /// assigned values.
    ///
    /// If a Model is passed, the solution will be evaluated immediately. Otherwise,
    /// there has to be an environment present to determine the correct variable types.
    ///
    /// Parameters
    /// ----------
    /// data : dict[Variable | str, int | float]
    ///     The sample that shall be part of the solution.
    /// env : Environment, optional
    ///     The environment the variable types shall be determined from.
    /// model : Model, optional
    ///     A model to evaluate the sample with.
    ///
    /// Returns
    /// -------
    /// Solution
    ///     The solution object created from the sample dict.
    ///
    /// Raises
    /// ------
    /// NoActiveEnvironmentFoundError
    ///     If no environment or model is passed to the method or available from the
    ///     context.
    /// ValueError
    ///     If `env` and `model` are both present. When this is the case, the user's
    ///     intention is unclear as the model itself already contains an environment.
    /// SolutionTranslationError
    ///     Generally if the sample translation fails. Might be specified by one of the
    ///     three following errors.
    /// SampleIncorrectLengthErr
    ///     If a sample has a different number of variables than the environment.
    /// SampleUnexpectedVariableError
    ///     If a sample has a variable that is not present in the environment.
    /// ModelVtypeError
    ///     If the result's variable types are incompatible with the model environment's
    ///     variable types.
    #[staticmethod]
    #[pyo3(signature=(data, env=None, model=None, timing=None))]
    fn from_dict(
        data: HashMap<SampleKey, f64>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
    ) -> PyResult<PySolution> {
        if env.is_some() && model.is_some() {
            return Err(PyValueError::new_err(
                "either `env` or `model` has to be `None`",
            ));
        }
        let environment: PyEnvironment = if model.is_some() {
            PyEnvironment(Rc::clone(&model.as_ref().unwrap().borrow().environment))
        } else {
            match env {
                Some(env) => env.clone(),
                None => CURRENT_ENV.with(|current| {
                    current.borrow().clone().ok_or_else(|| {
                        NoActiveEnvironmentFoundError::new_err("no active environment found.")
                    })
                })?,
            }
        };

        let mut sol = Solution::default();
        for v in environment.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(1))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(1))),
                Vtype::Integer => sol.add_column(SampleCol::Integer(Vec::with_capacity(1))),
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(1))),
            }
        }

        let n_vars = environment.borrow().varcount.into();
        let mut sample = vec![f64::default(); n_vars];
        let mut mask = vec![false; n_vars];
        let mut var_names = vec![String::default(); n_vars];

        for (k, &v) in data.iter() {
            let var_name = match k {
                SampleKey::Str(s) => s,
                SampleKey::Var(v) => &v.name(),
            };
            let environ = environment.borrow();
            let maybe_var = environ.variables_lookup.get(var_name);
            if maybe_var.is_none() {
                return Err(SampleUnexpectedVariableErr {
                    var_name: var_name.clone(),
                })?;
            }
            let var = maybe_var.unwrap().0 as usize;
            sample[var] = v;
            mask[var] = true;
            var_names[var] = var_name.clone();
        }

        if !mask.iter().all(|&x| x) {
            return Err(SampleIncorrectLengthErr)?;
        }

        sol.variable_names = var_names;
        sol.timing = timing.map(|t| t.0);
        let energy: Option<f64> = None;
        let _ = sol.extend(&sample, 1, energy)?;
        let mut sol_rc = RcSolution(Rc::new(sol));
        if let Some(m) = model {
            sol_rc = m.borrow().evaluate_solution(sol_rc)?;
        }

        Ok(PySolution(sol_rc))
    }

    /// Create a `Solution` from multiple dicts that map variables or variable names to their
    /// assigned values.
    ///
    /// If a Model is passed, the solution will be evaluated immediately. Otherwise,
    /// there has to be an environment present to determine the correct variable types.
    ///
    /// Parameters
    /// ----------
    /// data : list[dict[Variable | str, int | float]]
    ///     The samples that shall be part of the solution.
    /// env : Environment, optional
    ///     The environment the variable types shall be determined from.
    /// model : Model, optional
    ///     A model to evaluate the sample with.
    ///
    /// Returns
    /// -------
    /// Solution
    ///     The solution object created from the sample dict.
    ///
    /// Raises
    /// ------
    /// NoActiveEnvironmentFoundError
    ///     If no environment or model is passed to the method or available from the
    ///     context.
    /// ValueError
    ///     If `env` and `model` are both present. When this is the case, the user's
    ///     intention is unclear as the model itself already contains an environment.
    /// SolutionTranslationError
    ///     Generally if the sample translation fails. Might be specified by one of the
    ///     three following errors.
    /// SampleIncorrectLengthErr
    ///     If a sample has a different number of variables than the environment.
    /// SampleUnexpectedVariableError
    ///     If a sample has a variable that is not present in the environment.
    /// ModelVtypeError
    ///     If the result's variable types are incompatible with the model environment's
    ///     variable types.
    #[staticmethod]
    #[pyo3(signature=(data, env=None, model=None, timing=None)
    )]
    fn from_dicts(
        data: Vec<HashMap<SampleKey, f64>>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
    ) -> PyResult<PySolution> {
        if env.is_some() && model.is_some() {
            return Err(PyValueError::new_err(
                "either `env` or `model` has to be `None`",
            ));
        }
        let environment: PyEnvironment = if model.is_some() {
            PyEnvironment(Rc::clone(&model.as_ref().unwrap().borrow().environment))
        } else {
            match env {
                Some(env) => env.clone(),
                None => CURRENT_ENV.with(|current| {
                    current.borrow().clone().ok_or_else(|| {
                        NoActiveEnvironmentFoundError::new_err("no active environment found.")
                    })
                })?,
            }
        };

        let mut sol = Solution::default();
        for v in environment.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(data.len()))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(data.len()))),
                Vtype::Integer => {
                    sol.add_column(SampleCol::Integer(Vec::with_capacity(data.len())))
                }
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(data.len()))),
            }
        }

        let n_vars = environment.borrow().varcount.into();

        let mut samples: Vec<Vec<f64>> = Vec::with_capacity(data.len());

        for d in data.iter() {
            let mut sample = vec![f64::default(); n_vars];
            let mut mask = vec![false; n_vars];
            let mut var_names = vec![String::default(); n_vars];

            for (k, &v) in d.iter() {
                let var_name = match k {
                    SampleKey::Str(s) => s,
                    SampleKey::Var(v) => &v.name(),
                };
                let environ = environment.borrow();
                let maybe_var = environ.variables_lookup.get(var_name);
                if maybe_var.is_none() {
                    return Err(SampleUnexpectedVariableErr {
                        var_name: var_name.clone(),
                    })?;
                }
                let var = maybe_var.unwrap().0 as usize;
                sample[var] = v;
                mask[var] = true;
                var_names[var] = var_name.clone();
            }

            if !mask.iter().all(|&x| x) {
                return Err(SampleIncorrectLengthErr)?;
            }

            sol.variable_names = var_names;
            let energy: Option<f64> = None;

            if let Some(pos) = samples.iter().position(|s| s == &sample) {
                sol.counts[pos] += 1;
            } else {
                let _ = sol.extend(&sample, 1, energy)?;
                samples.push(sample);
            }
        }

        sol.timing = timing.map(|t| t.0);

        let mut sol_rc = RcSolution(Rc::new(sol));
        if let Some(m) = model {
            sol_rc = m.borrow().evaluate_solution(sol_rc)?;
        }

        Ok(PySolution(sol_rc))
    }

    /// Get a human-readable string representation of a solution.
    #[pyo3(
        signature=(
            max_line_length=80,
            max_chars_per_var=5,
            max_lines=10,
            layout=PrintLayout::Col,
            show_metadata=ShowMetadata::Right,
        )
    )]
    fn print(
        &self,
        max_line_length: usize,
        max_chars_per_var: usize,
        max_lines: usize,
        layout: PrintLayout,
        show_metadata: ShowMetadata,
    ) -> String {
        self.0.print(
            max_line_length,
            max_chars_per_var,
            max_lines,
            layout,
            show_metadata,
        )
    }

    /// Get an iterator over the single results of the solution.
    #[getter]
    fn get_results<'a>(&self) -> PyResultIterator {
        PyResultIterator(self.iter_results())
    }

    /// Get a view into the samples of the solution.
    #[getter]
    fn get_samples(&self) -> PySamples {
        PySamples(Samples(RcSolution::clone(&self)))
    }

    /// Get the objective values of the single samples as a ndarray. A value will be
    /// None if the sample hasn't yet been evaluated.
    #[getter]
    fn get_obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.obj_values
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Get the raw energy values of the single samples as returned by the solver /
    /// algorithm. Will be None if the solver / algorithm did not provide a value.
    #[getter]
    fn get_raw_energies<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.raw_energies
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Return how often each sample occurred in the solution.
    #[getter]
    fn get_counts<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<usize>> {
        self.counts.to_pyarray(py)
    }

    /// Get the solver / algorithm runtime.
    #[getter]
    fn get_runtime(&self) -> Option<PyTiming> {
        self.timing.map(|t| PyTiming(t))
    }

    /// Get the index of the sample with the best objective value.
    #[getter]
    fn get_best_sample_idx(&self) -> Option<usize> {
        self.best_sample_idx
    }

    /// Get the names of all variables in the solution.
    #[getter]
    fn get_variable_names(&self) -> Vec<String> {
        self.variable_names.clone()
    }

    /// Compute the expectation value.
    fn expectation_value(&self) -> PyResult<f64> {
        Ok(self.0.expectation_value()?)
    }

    /// Get the best result.
    fn best(&self) -> Option<PyResultView> {
        self.0.best().map(|r| PyResultView(r))
    }

    fn __len__(&self) -> usize {
        self.n_samples
    }

    /// Serialize the solution into a compact binary format.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the binary output. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded model representation.
    ///
    /// Raises
    /// ------
    /// IOError
    ///     If serialization fails.
    #[pyo3(signature=(compress=true, level=3))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .0
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
        .into())
    }

    /// Alias for `encode()`.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    /// Reconstruct a solution object from binary data.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Serialized model blob created by `encode()`.
    ///
    /// Returns
    /// -------
    /// Solution
    ///     The reconstructed solution.
    ///
    /// Raises
    /// ------
    /// DecodeError
    ///     If decoding fails due to corruption or incompatibility.
    #[classmethod]
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(PySolution(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }

    fn __str__(&self) -> String {
        self.print(80, 5, 10, PrintLayout::Col, ShowMetadata::Right)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    /// Iterate over the single results of the solution.
    ///
    /// Returns
    /// -------
    /// ResultIterator
    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        PyResultIterator(slf.0.iter_results())
    }

    /// Extract a result view from the `Solution` object.
    ///
    /// Returns
    /// -------
    /// ResultView
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If `item` has the wrong type.
    /// IndexError
    ///     If the row index is out of bounds for the variable environment.
    fn __getitem__(&self, py: Python, item: PyObject) -> PyResult<PyResultView> {
        if let Ok(res_idx) = item.extract::<usize>(py) {
            match self.get_result_view(res_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyResultView(r)),
            }
        } else {
            Err(PyTypeError::new_err("unsupported type for indexing"))
        }
    }

    /// Check whether this solution is equal to `other`.
    ///
    /// Parameters
    /// ----------
    /// other : Model
    ///
    /// Returns
    /// -------
    /// bool
    fn __eq__(&self, other: &PySolution) -> bool {
        &self.0 == &other.0
    }
}

impl<'py> IntoPyObject<'py> for PyVarAssignment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            VarAssignment::Binary(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Spin(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Integer(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Real(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl<'py> IntoPyObject<'py> for SampleKey {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SampleKey::Str(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            SampleKey::Var(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl<'py> FromPyObject<'py> for SampleKey {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(SampleKey::Str(s))
        } else if let Ok(v) = ob.extract::<PyVariable>() {
            Ok(SampleKey::Var(v))
        } else {
            Err(PyTypeError::new_err("keys have to be 'str' or 'Variable'"))
        }
    }
}

impl Hash for SampleKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SampleKey::Str(s) => s.hash(state),
            SampleKey::Var(v) => v.hash(state),
        }
    }
}

impl PartialEq<Self> for SampleKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SampleKey::Str(s1), SampleKey::Str(s2)) => s1 == s2,
            (SampleKey::Var(v1), SampleKey::Var(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for SampleKey {}

// Implement FromPyObject for your enum
impl<'py> FromPyObject<'py> for PrintLayout {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mode: &str = ob.extract()?;
        match mode {
            "row" => Ok(PrintLayout::Row),
            "column" => Ok(PrintLayout::Col),
            _ => Err(PyValueError::new_err(format!(
                "Invalid spec '{mode}'. Expected one of 'row', 'column'."
            ))),
        }
    }
}

impl<'py> FromPyObject<'py> for ShowMetadata {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mode: &str = ob.extract()?;
        match mode {
            "left" => Ok(ShowMetadata::Left),
            "right" => Ok(ShowMetadata::Right),
            "false" => Ok(ShowMetadata::False),
            _ => Err(PyValueError::new_err(format!(
                "Invalid spec '{mode}'. Expected one of 'left', 'right', 'false."
            ))),
        }
    }
}
