use super::py_utilities::repr_solution;
use crate::core::solution::{ColElement, Column, PrintLayout, ShowMetadata, VarKey};
use crate::core::{make_index_map, Sense, SharedEnvironment, Solution, VarAssignment, Vtype};
use crate::errors::{
    ComputationErr, SampleIncorrectLengthErr, SampleUnexpectedVariableErr, VariableNotExistingErr,
};
use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::py_res::{PyResultIterator, PyResultView};
use crate::py_bindings::py_sample::PySamples;
use crate::py_bindings::py_timing::PyTiming;
use crate::py_bindings::py_usize::PyUsize;
use crate::py_bindings::py_var::PyVariable;
use crate::serialization::{Decodable, Decompressable, Encodable, Unversionizable};
use crate::types::VarIndex;
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use indexmap::IndexMap;
use itertools::Itertools;
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};
use pyo3::IntoPyObjectExt;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(pub VarAssignment);

#[derive(Debug, Clone)]
pub enum VariableKey {
    Str(String),
    Var(PyVariable),
}

impl From<String> for VariableKey {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

enum BitOrder {
    LTR,
    RTL,
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
#[cfg_attr(
    not(feature = "lq"),
    pyclass(unsendable, name = "Solution", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Solution", module = "luna_quantum._core")
)]
#[derive(Deref, DerefMut, Debug, Clone)]
pub struct PySolution(pub Rc<RefCell<Solution>>);

impl PySolution {
    pub fn new(sol: Solution) -> Self {
        PySolution(Rc::new(RefCell::new(sol)))
    }
}

// impl Into<SharedSolution> for PySolution {
//     fn into(self) -> SharedSolution {
//         self.0
//     }
// }

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
    /// >>> sol = Solution._build(
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
    /// >>> sol = Solution._build(
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
    #[pyo3(signature=(component_types, variable_names=None, binary_cols=None, spin_cols=None, int_cols=None, real_cols=None, raw_energies=None, timing=None, counts=None, sense=None, obj_values=None, constraints=None, variable_bounds=None, feasible=None)
    )]
    fn _build(
        component_types: Vec<Vtype>,
        variable_names: Option<Vec<String>>,
        binary_cols: Option<Vec<Vec<u8>>>,
        spin_cols: Option<Vec<Vec<i8>>>,
        int_cols: Option<Vec<Vec<i64>>>,
        real_cols: Option<Vec<Vec<f64>>>,
        raw_energies: Option<Vec<Option<f64>>>,
        timing: Option<PyTiming>,
        counts: Option<Vec<PyUsize>>,
        sense: Option<Sense>,
        obj_values: Option<Vec<f64>>,
        constraints: Option<Vec<Vec<bool>>>,
        variable_bounds: Option<Vec<Vec<bool>>>,
        feasible: Option<Vec<bool>>,
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
        let mut sol = Solution::with_sense(sense.unwrap_or_default());

        let (mut lb, mut ls, mut li, mut lr) = (0, 0, 0, 0);
        let binary_cols = binary_cols.unwrap_or(Vec::new());
        let spin_cols = spin_cols.unwrap_or(Vec::new());
        let int_cols = int_cols.unwrap_or(Vec::new());
        let real_cols = real_cols.unwrap_or(Vec::new());

        let mut num_samples: Option<usize> = None;
        for (i, ct) in component_types.iter().enumerate() {
            let len = match ct {
                Vtype::__Ghost => 0,
                Vtype::Binary => {
                    let bc = binary_cols[lb].clone();
                    let bc_len = bc.len();
                    sol.add_column(Column::Binary(ColElement::new(i.into(), bc)));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("b{lb}")));
                    lb += 1;
                    bc_len
                }
                Vtype::Spin => {
                    let sc = spin_cols[ls].clone();
                    let sc_len = sc.len();
                    sol.add_column(Column::Spin(ColElement::new(i.into(), sc)));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("s{ls}")));
                    ls += 1;
                    sc_len
                }
                Vtype::Integer => {
                    let ic = int_cols[li].clone();
                    let ic_len = ic.len();
                    sol.add_column(Column::Integer(ColElement::new(i.into(), ic)));
                    sol.variable_names
                        .push(var_names[i].clone().unwrap_or(format!("i{li}")));
                    li += 1;
                    ic_len
                }
                Vtype::Real => {
                    let rc = real_cols[lr].clone();
                    let rc_len = rc.len();
                    sol.add_column(Column::Real(ColElement::new(i.into(), rc)));
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
            sol.counts = no.into_iter().map(|x| x.into()).collect();
        } else {
            sol.counts = vec![1; sol.n_samples];
        }
        sol.obj_values = obj_values;
        sol.constraints = constraints;
        sol.variable_bounds = variable_bounds;
        sol.feasible = feasible;
        sol.timing = timing.and_then(|t| Some(t.0));
        Ok(PySolution::new(sol))
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
    /// counts : int, optional
    ///     The number of occurrences of this sample.
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
    ///     Or if `sense` and `model` are both present as the sense is then ambiguous.
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
    #[pyo3(signature=(data, env=None, model=None, timing=None, counts=None, sense=None))]
    pub fn from_dict(
        data: IndexMap<VariableKey, f64>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        counts: Option<usize>,
        sense: Option<Sense>,
    ) -> PyResult<PySolution> {
        Self::check_env_or_model(&env, &model)?;
        Self::check_sense_or_model(&sense, &model)?;
        let environment = Self::retrieve_environment(&env, &model)?;

        let mut sol = Solution::with_sense(
            sense.unwrap_or(model.as_ref().map(|m| m.borrow().sense).unwrap_or_default()),
        );
        sol.create_columns(&environment, 1);
        let n_vars = environment.borrow().varcount() as usize;
        let data = Self::build_ordered_raw_sample(data, &environment.variable_names())?;
        sol.variable_names = environment.variable_names();
        let index_map = make_index_map(sol.varname_to_pos(), &environment);
        let sample = Self::build_sample(&data, n_vars, &environment, |idx| index_map[&idx])?;
        sol.timing = timing.map(|t| t.0);

        let energy: Option<f64> = None;
        let _ = sol.extend(&sample, counts.unwrap_or(1), energy)?;
        if let Some(m) = model {
            sol = m.borrow().evaluate_solution(sol)?;
        }

        Ok(PySolution::new(sol))
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
    /// counts : int, optional
    ///     The number of occurrences for each sample.
    /// timing : Timing, optional
    ///     The timing for acquiring the solution.
    /// sense : Sense, optional
    ///     The sense the model the solution belongs to. Default: Sense.Min
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
    ///     Or if `sense` and `model` are both present as the sense is then ambiguous.
    ///     Or if the the number of samples and the number of counts do not match.
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
    #[pyo3(signature=(data, env=None, model=None, timing=None, counts=None, sense=None)
    )]
    fn from_dicts(
        data: Vec<IndexMap<VariableKey, f64>>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        counts: Option<Vec<usize>>,
        sense: Option<Sense>,
    ) -> PyResult<PySolution> {
        Self::check_env_or_model(&env, &model)?;
        Self::check_sense_or_model(&sense, &model)?;
        Self::check_counts_and_samples_len(&counts, data.len())?;
        let environment = Self::retrieve_environment(&env, &model)?;

        let mut sol = Solution::with_sense(
            sense.unwrap_or(model.as_ref().map(|m| m.borrow().sense).unwrap_or_default()),
        );
        sol.create_columns(&environment, data.len());

        let mut samples: Vec<Vec<f64>> = Vec::with_capacity(data.len());

        let n_vars = environment.borrow().varcount() as usize;
        // we need to ensure each sample dict / hashmap / indexmap has the same order.
        let data = Self::build_ordered_raw_samples(data, &environment.variable_names())?;
        sol.variable_names = environment.variable_names();
        let index_map = make_index_map(sol.varname_to_pos(), &environment);

        for (i, d) in data.iter().enumerate() {
            let sample = Self::build_sample(d, n_vars, &environment, |idx| index_map[&idx])?;
            let energy: Option<f64> = None;

            let sc = counts
                .as_ref()
                .and_then(|c| Some(c[i]))
                .or(Some(1))
                .unwrap();
            if let Some(pos) = samples.iter().position(|s| s == &sample) {
                sol.counts[pos] += sc;
            } else {
                let _ = sol.extend(&sample, sc, energy)?;
                samples.push(sample);
            }
        }

        sol.timing = timing.map(|t| t.0);

        if let Some(m) = model {
            sol = m.borrow().evaluate_solution(sol)?;
        }

        Ok(PySolution::new(sol))
    }

    /// Create a `Solution` from a dict that maps measured bitstrings to counts.
    ///
    /// If a Model is passed, the solution will be evaluated immediately. Otherwise,
    /// there has to be an environment present to determine the correct variable types.
    /// Only applicable to binary or spin models.
    ///
    /// Parameters
    /// ----------
    /// data : dict[str, int]
    ///     The counts that shall be part of the solution.
    /// env : Environment, optional
    ///     The environment the variable types shall be determined from.
    /// model : Model, optional
    ///     A model to evaluate the sample with.
    /// timing : Timing, optional
    ///     The timing for acquiring the solution.
    /// sense : Sense, optional
    ///     The sense the model the solution belongs to. Default: Sense.Min
    /// bit_order : Literal["LTR", "RTL"]
    ///     The order of the bits in the bitstring. Default "RTL".
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
    ///     Or if `sense` and `model` are both present as the sense is then ambiguous.
    ///     Or if the the environment contains non-(binary or spin) variables.
    ///     Or if a bitstring contains chars other than '0' and '1'.
    /// SolutionTranslationError
    ///     Generally if the sample translation fails. Might be specified by one of the
    ///     three following errors.
    /// SampleIncorrectLengthErr
    ///     If a sample has a different number of variables than the environment.
    #[staticmethod]
    #[pyo3(signature=(data, env=None, model=None, timing=None, sense=None, bit_order="RTL".to_owned())
    )]
    fn from_counts(
        data: IndexMap<String, usize>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        sense: Option<Sense>,
        bit_order: String,
    ) -> PyResult<PySolution> {
        Self::check_env_or_model(&env, &model)?;
        Self::check_sense_or_model(&sense, &model)?;
        let environment = Self::retrieve_environment(&env, &model)?;

        let mut sol = Solution::with_sense(
            sense.unwrap_or(model.as_ref().map(|m| m.borrow().sense).unwrap_or_default()),
        );
        for (idx, v) in environment.borrow().all_variables().enumerate() {
            match v.vtype {
                Vtype::Binary => sol.add_column(Column::Binary(ColElement::new(
                    idx.into(),
                    Vec::with_capacity(data.len()),
                ))),
                Vtype::Spin => sol.add_column(Column::Spin(ColElement::new(
                    idx.into(),
                    Vec::with_capacity(data.len()),
                ))),
                _ => {
                    return Err(PyValueError::new_err(
                        "environment contains non-binary or non-spin variables.",
                    ))
                }
            }
            sol.variable_names.push(v.name.clone())
        }

        let order = match bit_order.as_str() {
            "LTR" => BitOrder::LTR,
            "RTL" => BitOrder::RTL,
            _ => return Err(PyValueError::new_err("`bit_order` must be 'RTL' or 'LTR'.")),
        };

        let nvars = sol.samples.len();
        sol.n_samples = data.len();
        sol.raw_energies = vec![None; data.len()];
        sol.obj_values = None;
        sol.constraints = None;
        sol.variable_bounds = None;
        sol.feasible = None;

        for (k, v) in data.iter() {
            if k.len() != nvars {
                return Err(SampleIncorrectLengthErr.into());
            }
            let it = match order {
                BitOrder::LTR => itertools::Either::Left(k.chars()),
                BitOrder::RTL => itertools::Either::Right(k.chars().rev()),
            };

            for (c, col) in it.into_iter().zip(sol.samples.iter_mut()) {
                match (c, col) {
                    ('0', Column::Binary(vec)) => vec.push(0),
                    ('1', Column::Binary(vec)) => vec.push(1),
                    ('0', Column::Spin(vec)) => vec.push(1),
                    ('1', Column::Spin(vec)) => vec.push(-1),
                    _ => return Err(PyValueError::new_err("unexpected char in bitstring.")),
                }
            }
            sol.counts.push(*v);
        }

        sol.timing = timing.map(|t| t.0);

        if let Some(m) = model {
            sol = m.borrow().evaluate_solution(sol)?;
        }

        Ok(PySolution::new(sol))
    }

    /// Show a solution object as a human-readable string.
    ///
    /// This method provides various ways to customize the way the solution is
    /// represented as a string.
    ///
    /// Parameters
    /// ----------
    /// layout : Literal["row", "column"]
    ///     With `"row"` layout, all assignments to one variable across different
    ///     samples are shown in the same *row*, and each sample is shown in one
    ///     column.
    ///     With `"column"` layout, all assignments to one variable across different
    ///     samples are shown in the same *column*, and each sample is shown in one row.
    /// max_line_length : int
    ///     The max number of chars shown in one line or, in other words, the max width
    ///     of a row.
    /// max_column_length : int
    ///     The maximal number of chars in one column. For both the row and column
    ///     layout, this controls the max number of chars a single variable assignment
    ///     may be shown with. For the column layout, this also controls the max number
    ///     of chars that a variable name is shown with.
    ///     Note: the max column length cannot always be adhered to. This is
    ///     specifically the case when a variable assignment is so high that the max
    ///     column length is not sufficient to show the number correctly.
    /// max_lines : int
    ///     The max number of lines used for showing the samples. Note that this
    ///      parameter does not influence how metadata are shown, s.t. the total number
    ///      of lines may be higher than `max_lines`.
    /// max_var_name_length : int
    ///     The max number of chars that a variable is shown with in row layout. This
    ///     parameter is ignored in column layout.
    /// show_metadata : Literal["before", "after", "hide"]
    ///     Whether and where to show sample-specific metadata such as feasibility and
    ///     objective value. Note that this parameter only controls how sample-specific
    ///     metadata are shown. Other metadata, like the solution timing will be shown
    ///     after the samples regardless of the value of this parameter.
    ///
    ///     - `"before"`: show metadata before the actual sample, i.e., above the
    ///         sample in row layout, and left of the sample in column layout.
    ///     - `"after"`: show metadata after the actual sample, i.e., below the
    ///         sample in row layout, and right of the sample in column layout.
    ///     - "hide": do not show sample-specific metadata.
    ///
    /// Returns
    /// -------
    /// str
    ///     The solution represented as a string.
    ///
    /// Raises
    ///  ------
    ///  ValueError
    ///      If at least one of the params has an invalid value.
    #[pyo3(
        signature=(
            layout=PrintLayout::Col,
            max_line_length=PyUsize(80),
            max_column_length=PyUsize(5),
            max_lines=PyUsize(10),
            max_var_name_length=PyUsize(10),
            show_metadata=ShowMetadata::After,
        )
    )]
    fn print(
        &self,
        layout: PrintLayout,
        max_line_length: PyUsize,
        max_column_length: PyUsize,
        max_lines: PyUsize,
        max_var_name_length: PyUsize,
        show_metadata: ShowMetadata,
    ) -> PyResult<String> {
        let mll = max_line_length.into();
        let mcl = max_column_length.into();
        let ml = max_lines.into();
        let mvnl = max_var_name_length.into();
        if mll < 5 {
            Err(PyValueError::new_err(format!(
                "`max_line_length needs` to be at least 5; actual value: {mll}"
            )))
        } else if mcl < 1 {
            Err(PyValueError::new_err(format!(
                "`max_column_length` needs to be at least 1; actual value: {mcl}"
            )))
        } else if ml < 1 {
            Err(PyValueError::new_err(format!(
                "`max_lines` needs to be at least 1; actual value: {ml}"
            )))
        } else if mvnl < 1 {
            Err(PyValueError::new_err(format!(
                "`max_var_name_length` needs to be at least 1; actual value: {mvnl}"
            )))
        } else {
            Ok(self
                .borrow()
                .print(mll, mcl, ml, mvnl, layout, show_metadata))
        }
    }

    /// Get an iterator over the single results of the solution.
    #[getter]
    fn get_results(&self) -> PyResultIterator {
        // PyResultIterator(self.borrow().iter_results())
        PyResultIterator::new(self.clone())
    }

    /// Get a view into the samples of the solution.
    #[getter]
    fn get_samples<'a>(&'a self) -> PySamples {
        // PySamples(Samples(&self.0))
        PySamples(self.clone())
    }

    /// Get the objective values of the single samples as a ndarray. A value will be
    /// None if the sample hasn't yet been evaluated.
    #[getter]
    fn get_obj_values<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<PyObject>>> {
        self.borrow().obj_values.as_ref().map(|ovs| {
            ovs.iter()
                .map(|x| x.into_py_any(py).unwrap())
                .collect::<Vec<_>>()
                .to_pyarray(py)
        })
    }

    /// Get the raw energy values of the single samples as returned by the solver /
    /// algorithm. Will be None if the solver / algorithm did not provide a value.
    #[getter]
    fn get_raw_energies<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.borrow()
            .raw_energies
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Return how often each sample occurred in the solution.
    #[getter]
    fn get_counts<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<usize>> {
        self.borrow().counts.to_pyarray(py)
    }

    /// Get the solver / algorithm runtime.
    #[getter]
    fn get_runtime(&self) -> Option<PyTiming> {
        self.borrow().timing.map(|t| PyTiming(t))
    }

    /// Get the optimization sense.
    #[getter]
    fn get_sense(&self) -> Sense {
        self.borrow().sense
    }

    /// Get the index of the sample with the best objective value.
    #[getter]
    fn get_best_sample_idx(&self) -> Option<usize> {
        self.borrow().best_sample_idx
    }

    /// Get the names of all variables in the solution.
    #[getter]
    fn get_variable_names(&self) -> Vec<String> {
        self.borrow().variable_names.clone()
    }

    /// Compute the expectation value of the solution.
    ///
    /// Returns
    /// -------
    /// float
    ///     The expectation value.
    ///
    /// Raises
    /// ------
    /// ComputationError
    ///     If the computation fails for any reason.
    fn expectation_value(&self) -> PyResult<f64> {
        Ok(self.borrow().expectation_value()?)
    }

    /// Compute the expectation value of the solution.
    ///
    /// Returns
    /// -------
    /// float
    ///     The feasibility ratio.
    ///
    /// Raises
    /// ------
    /// ComputationError
    ///     If the computation fails for any reason.
    fn feasibility_ratio(&self) -> PyResult<f64> {
        Ok(self.borrow().feasibility_ratio()?)
    }

    /// Get a new solution with all infeasible samples removed.
    ///
    /// Returns
    /// -------
    ///     The new solution with only feasible samples.
    ///
    /// Raises
    /// ------
    /// ComputationError
    ///     If the computation fails for any reason.
    fn filter_feasible(&self) -> PyResult<PySolution> {
        if let Some(f) = &self.borrow().feasible {
            let sol = self.borrow().filter_samples(&f);
            Ok(PySolution::new(sol))
        } else {
            Err(ComputationErr(
                "no feasible information on solution, evalaute first.".to_string(),
            ))?
        }
    }

    /// Get the index of the constraint with the highest number of violations.
    ///
    /// Returns
    /// -------
    /// int | None
    ///     The index of the constraint with the most violations. None, if the solution
    ///     was created for an unconstrained model.
    ///
    /// Raises
    /// ------
    /// ComputationError
    ///     If the computation fails for any reason.
    fn highest_constraint_violations(&self) -> PyResult<Option<usize>> {
        Ok(self.borrow().highest_constraint_violations()?)
    }

    /// Get the best result.
    fn best(&self) -> Option<PyResultView> {
        self.borrow()
            .best()
            .map(|r| PyResultView::new(self.clone(), r.idx))
    }

    fn __len__(&self) -> usize {
        self.borrow().n_samples
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
        // Ok(PyBytes::new(py, &self.borrow().encode(compress, level)?).into())
        Ok(PyBytes::new(py, &self.borrow().encode(compress, level)?).into())
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
        Ok(PySolution::new(
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
        let s = self.print(
            PrintLayout::Col,
            80.into(),
            5.into(),
            10.into(),
            10.into(),
            ShowMetadata::After,
        );
        s.unwrap()
    }

    fn __repr__(&self) -> String {
        repr_solution(self)
    }

    /// Iterate over the single results of the solution.
    ///
    /// Returns
    /// -------
    /// ResultIterator
    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        PyResultIterator::new(slf.clone())
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
        if let Ok(res_idx) = item.extract::<isize>(py) {
            if res_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {res_idx}"
                )))?;
            }
            match self.borrow().get_result_view(res_idx as usize) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyResultView::new(self.clone(), r.idx)),
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
        &self.borrow().deref() == &other.borrow().deref()
    }

    #[pyo3(signature=(var, data, vtype=None))]
    // todo: change to pyarray
    fn add_var(&self, var: VariableKey, data: Vec<f64>, vtype: Option<Vtype>) -> PyResult<()> {
        let (var, default) = match &var {
            VariableKey::Str(str) => (
                VarKey::Name(str.to_string()),
                vtype.unwrap_or_else(|| Vtype::Binary),
            ),
            VariableKey::Var(elem) => (
                VarKey::Var(elem.0.as_ref()),
                elem.0.as_ref().env.borrow().get_vtype(elem.0.id),
            ),
        };
        Ok(self
            .borrow_mut()
            .add_samplecol(var, data.as_slice(), default)?)
    }

    #[pyo3(signature=(variables, data, vtypes=None))]
    fn add_vars(
        &self,
        variables: Vec<VariableKey>,
        // todo: change to pyarray
        data: Vec<Vec<f64>>,
        vtypes: Option<Vec<Option<Vtype>>>,
    ) -> PyResult<()> {
        let vtypes = match vtypes {
            Some(vs) => vs
                .iter()
                .zip(&variables)
                .map(|(vt, vk)| match vk {
                    VariableKey::Str(_) => vt.unwrap_or_else(|| Vtype::Binary),
                    VariableKey::Var(v) => v.0.as_ref().env.borrow().get_vtype(v.0.id),
                })
                .collect_vec(),
            None => variables
                .iter()
                .map(|vk| match vk {
                    VariableKey::Str(_) => Vtype::Binary,
                    VariableKey::Var(v) => v.0.as_ref().env.borrow().get_vtype(v.0.id),
                })
                .collect_vec(),
        };

        for (i, col) in data.iter().enumerate() {
            let var = match &variables[i] {
                VariableKey::Str(str) => VarKey::Name(str.to_string()),
                VariableKey::Var(elem) => VarKey::Var(elem.0.as_ref()),
            };
            self.borrow_mut().add_samplecol(var, &col, vtypes[i])?;
        }
        Ok(())
    }

    fn remove_var(&self, var: VariableKey) {
        let var = match &var {
            VariableKey::Str(str) => VarKey::Name(str.to_string()),
            VariableKey::Var(elem) => VarKey::Var(elem.0.as_ref()),
        };
        self.borrow_mut().remove_samplecol(var)
    }

    fn remove_vars(&self, variables: Vec<VariableKey>) {
        for var in variables {
            let var = match &var {
                VariableKey::Str(str) => VarKey::Name(str.to_string()),
                VariableKey::Var(elem) => VarKey::Var(elem.0.as_ref()),
            };
            self.borrow_mut().remove_samplecol(var)
        }
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

impl<'py> IntoPyObject<'py> for VariableKey {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            VariableKey::Str(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VariableKey::Var(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl<'py> FromPyObject<'py> for VariableKey {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(VariableKey::Str(s))
        } else if let Ok(v) = ob.extract::<PyVariable>() {
            Ok(VariableKey::Var(v))
        } else {
            Err(PyTypeError::new_err("keys have to be 'str' or 'Variable'"))
        }
    }
}

impl Hash for VariableKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            VariableKey::Str(s) => s.hash(state),
            VariableKey::Var(v) => v.hash(state).expect(&VariableNotExistingErr {}.to_string()),
        }
    }
}

impl PartialEq<Self> for VariableKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VariableKey::Str(s1), VariableKey::Str(s2)) => s1 == s2,
            (VariableKey::Var(v1), VariableKey::Var(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for VariableKey {}

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
            "before" => Ok(ShowMetadata::Before),
            "after" => Ok(ShowMetadata::After),
            "hide" => Ok(ShowMetadata::Hide),
            _ => Err(PyValueError::new_err(format!(
                "Invalid spec '{mode}'. Expected one of 'before', 'after', 'hide'."
            ))),
        }
    }
}

impl PySolution {
    fn check_env_or_model(env: &Option<PyEnvironment>, model: &Option<PyModel>) -> PyResult<()> {
        if env.is_some() && model.is_some() {
            Err(PyValueError::new_err(
                "either `env` or `model` has to be `None`",
            ))
        } else {
            Ok(())
        }
    }

    fn check_sense_or_model(sense: &Option<Sense>, model: &Option<PyModel>) -> PyResult<()> {
        if sense.is_some() && model.is_some() {
            Err(PyValueError::new_err(
                "either `sense` or `model` has to be `None`",
            ))
        } else {
            Ok(())
        }
    }

    fn check_counts_and_samples_len(counts: &Option<Vec<usize>>, data_len: usize) -> PyResult<()> {
        if counts.is_some() && counts.as_ref().unwrap().len() != data_len {
            return Err(PyValueError::new_err(format!(
                "the number of samples and the number of counts do not match: num samples is '{}', num counts is '{}'",
                data_len, counts.as_ref().unwrap().len()))
            );
        } else {
            Ok(())
        }
    }

    fn retrieve_environment(
        env: &Option<PyEnvironment>,
        model: &Option<PyModel>,
    ) -> PyResult<PyEnvironment> {
        let environment = if let Some(model) = &model {
            PyEnvironment(model.borrow().environment.clone())
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
        Ok(environment)
    }

    pub fn build_ordered_raw_samples(
        data: Vec<IndexMap<VariableKey, f64>>,
        variable_names: &Vec<String>,
    ) -> PyResult<Vec<IndexMap<String, f64>>> {
        // todo: this works for now and gives the expected results based on the tests
        // however, there are some bottlenecks in here which probably can be eliminated
        // this needs further investigation in the future. For now this is fine.
        if data.is_empty() {
            return Err(PyRuntimeError::new_err("empty data passed."));
        }
        let data = data
            .into_iter()
            .map(|sample| {
                sample
                    .into_iter()
                    .map(|(k, v)| {
                        let var_name = match k {
                            VariableKey::Str(s) => Ok(s),
                            VariableKey::Var(v) => v.name(),
                        };
                        match var_name {
                            Ok(vn) => Ok((vn, v)),
                            Err(e) => Err(e),
                        }
                    })
                    .collect::<Result<IndexMap<_, _>, _>>()
            })
            .collect::<Result<Vec<IndexMap<_, _>>, _>>()?;
        let rank: IndexMap<&String, usize> = variable_names
            .iter()
            .enumerate()
            .map(|(i, k)| (k, i))
            .collect();
        let data = data
            .iter()
            .map(|sample| {
                let mut s = sample.clone();
                s.sort_unstable_by(|k1, _v1, k2, _v_2| {
                    let r1 = rank.get(k1).copied().unwrap_or(usize::MAX);
                    let r2 = rank.get(k2).copied().unwrap_or(usize::MAX);
                    r1.cmp(&r2)
                });
                s
            })
            .collect::<Vec<IndexMap<_, _>>>();
        Ok(data)
    }

    pub fn build_ordered_raw_sample(
        data: IndexMap<VariableKey, f64>,
        variables: &Vec<String>,
    ) -> PyResult<IndexMap<String, f64>> {
        let mut new_data = IndexMap::with_capacity(variables.len());
        let rank: HashMap<&String, usize> =
            variables.iter().enumerate().map(|(i, k)| (k, i)).collect();

        for (k, v) in data.iter() {
            let var_name = match k {
                VariableKey::Str(s) => s,
                VariableKey::Var(v) => &v.name()?,
            };
            new_data.insert(var_name.to_string(), *v);
        }
        new_data.sort_unstable_by(|k1, _v1, k2, _v2| {
            let r1 = rank.get(k1).copied().unwrap_or(usize::MAX);
            let r2 = rank.get(k2).copied().unwrap_or(usize::MAX);
            r1.cmp(&r2)
        });

        Ok(new_data)
    }

    pub fn build_sample<F>(
        data: &IndexMap<String, f64>,
        n_vars: usize,
        env: &SharedEnvironment,
        map_varidx: F,
    ) -> PyResult<Vec<f64>>
    where
        F: Fn(VarIndex) -> VarIndex,
    {
        let mut sample = vec![f64::default(); n_vars];
        let mut mask = vec![false; n_vars];

        for (k, &v) in data.iter() {
            let environ = env.borrow();
            let maybe_var = environ.get_varidx(k).ok();
            if maybe_var.is_none() {
                return Err(SampleUnexpectedVariableErr {
                    var_name: k.clone(),
                })?;
            }
            let var = maybe_var.unwrap().0 as usize;
            // println!("build_var -> var = {}", &var);
            let sidx: usize = map_varidx(var.into()).into();
            // println!("build_var -> sidx = {}", &sidx);
            sample[sidx] = v;
            mask[sidx] = true;
        }

        if !mask.iter().all(|&x| x) {
            return Err(SampleIncorrectLengthErr)?;
        }

        Ok(sample)
    }
}
