mod py_bounds;
mod py_constr;
mod py_env;
mod py_exceptions;
mod py_expr;
mod py_model;
mod py_model_metadata;
pub mod py_modules;
mod py_res;
mod py_sample;
mod py_sol;
mod py_timing;
mod py_translator;
mod py_usize;
mod py_utilities;
mod py_utils;
mod py_var;
pub mod py_unwind;

pub use py_unwind::unwind;

#[cfg(feature = "transformations")]
mod py_transformations;

#[cfg(feature = "transformations")]
pub use py_transformations::register_transformations;

#[cfg(not(feature = "lq"))]
use pyo3::prelude::*;

/// AqModels
/// ========
///
/// Provides
///   1. A model object to define arbitrary (constrained) optimization problems.
///   2. A solution object to define arbitrary solutions to optimization problems.
///   3. Extendable translators to map arbitrary models of other libraries to an aq model.
///   4. Extendable transformers to transform arbitrary (constrained) optimization problems.
///
///
/// How to use the documentation
/// ----------------------------
/// Documentation is available in two forms: docstrings provided with the code, and a
/// reference guide, available from `the Aqarios homepage <https://docs.aqarios.com>`_.
///
/// We recommend exploring the docstrings using
/// `IPython <https://ipython.org>`_, an advanced Python shell with
/// TAB-completion and introspection capabilities.  See below for further
/// instructions.
///
/// The docstring examples assume that `aqmodels` has been imported as ``aqm``::
///
///   >>> import aqmodels as aqm
///
/// Code snippets are indicated by three greater-than signs::
///
///   >>> x = 42
///   >>> x = x + 1
///
/// Use the built-in ``help`` function to view a function's docstring::
///
///   >>> help(aqm.Model)
///   ... # doctest: +SKIP
///
/// Available subpackages
/// ---------------------
/// translators
///     Built-in translators to map a model of a (constrained) optimization problem from
///     another library to an aq-models model.
/// transformers
///     Built-in transformers to map a model of a (constrained) optimization problem to
///     another aq-models model. Such a transformer for example can map a constrained
///     optimization problem to an unconstrained optimization problem or a quadratic model
///     to a linear model.
#[cfg(not(feature = "lq"))]
#[pymodule]
#[pyo3(name = "_core")]
pub fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information to the python module
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    py_modules::register_core(m)?;
    py_modules::register_translator(m)?;
    py_modules::register_errors(m)?;
    py_modules::register_utils(m)?;

    #[cfg(feature = "transformations")]
    py_transformations::register_transformations(m)?;

    Ok(())
}
