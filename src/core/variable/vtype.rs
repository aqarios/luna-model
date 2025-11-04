use enumset::EnumSetType;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use strum_macros::{Display, EnumIter};

/// Enumeration of variable types supported by the optimization system.
///
/// This enum defines the type of a variable used in a model. The type influences
/// the domain and behavior of the variable during optimization. It is often passed
/// when defining variables to specify how they should behave.
///
/// Attributes
/// ----------
/// Real : Vtype
///     Continuous real-valued variable. Can take any value within given bounds.
/// Integer : Vtype
///     Discrete integer-valued variable. Takes integer values within bounds.
/// Binary : Vtype
///     Binary variable. Can only take values 0 or 1.
/// Spin : Vtype
///     Spin variable. Can only take values -1 or +1.
///
/// Examples
/// --------
/// >>> from luna_model import Vtype
/// >>> Vtype.Real
/// Real
///
/// >>> str(Vtype.Binary)
/// 'Binary'
// we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "Vtype", module = "luna_model._core")
)]
#[derive(EnumSetType, Debug, EnumIter, Display, Hash)]
pub enum Vtype {
    /// Binary variable. Can only take values 0 or 1.
    Binary,
    /// Inverted binary variable. ¬b == 1 - b. Can only take values 0 or 1.
    InvertedBinary,
    /// Spin variable. Can only take values -1 or +1.
    Spin,
    /// Discrete integer-valued variable. Takes integer values within bounds.
    Integer,
    /// Continuous real-valued variable. Can take any value within given bounds.
    Real,
    /// Special Ghost Variable type. Cannot be exluded from python enum specifically.
    /// todo: either keep as is or make pyenum wrapper...
    // TODO: remove this => breaking changes in encoding/decoding (serialization/deserialization)
    __Ghost,
}

impl Vtype {
    /// The default variable type.
    pub fn default() -> Self {
        Vtype::Binary
    }
}
