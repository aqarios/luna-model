use lunamodel_core::prelude::Environment;
use pyo3::{PyResult, pymethods};

use crate::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    /// Initialize a new environment for variable construction.
    ///
    /// It is recommended to use this in a `with` statement to ensure proper scoping.
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(Environment::default()))
    }

    // #[getter]
    // fn num_variables(&self) -> usize {
    //     self.env.num_variables()
    // }

    // /// Get a variable by its label (name).
    // ///
    // /// Parameters
    // /// ----------
    // /// label : str
    // ///     The name/label of the variable
    // ///
    // /// Returns
    // /// -------
    // /// Variable
    // ///     The variable with the specified label/name.
    // ///
    // /// Raises
    // /// ------
    // /// VariableNotExistingError
    // ///     If no variable with the specified name is registered.
    // fn get_variable(&self, name: String) -> PyResult<PyVariable> {
    //     Ok(self.env.read_arc().get_vref_by_name(&name)?.into())
    // }

    // fn variables(&self) -> Vec<PyVariable> {
    //     self.vrefs()
    //         .into_iter()
    //         .map(|v| PyVariable::new(v))
    //         .collect()
    // }

    // fn __eq__(&self, other: &PyEnvironment) -> bool {
    //     *self.env.read_arc() == *other.env.read_arc()
    // }

    // fn __str__(&self) -> String {
    //     self.env.read_arc().to_string()
    // }

    // fn __repr__(&self) -> String {
    //     format!("{:#?}", self.env.read_arc())
    // }

    // fn equal_contents(&self, other: &Self) -> bool {
    //     self.env.read_arc().is_equal_contents(&other.env.read_arc())
    // }

    // fn __contains__(&self, varname: String) -> bool {
    //     self.env.read_arc().contains(varname)
    // }
}
