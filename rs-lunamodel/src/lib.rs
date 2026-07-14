//! Rust wrapper over the LunaModel C API for native Rust extensions.
//!
//! This crate mirrors the native `luna-model` API but routes every call through
//! the stable C ABI. It depends only on [`lunamodel_cext_vtable`] for the
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) dispatch table and the
//! `Lm*` ABI structs — never on `luna-model` itself — so a consumer never links a
//! second copy of the core.
//!
//! # Wiring up the table
//!
//! The function pointers come from the host's single in-process instance, exposed
//! through the `luna_model._lm._C_API` PyCapsule. Install the table once during
//! startup, then use the wrappers just like the native API:
//!
//! ```ignore
//! // In a pyo3 extension: read the capsule and hand the pointer to `init`.
//! let capsule = py.import("luna_model._lm")?.getattr("_C_API")?;
//! let table = capsule.downcast::<PyCapsule>()?.pointer() as *const LunaModelCApi;
//! unsafe { rs_lunamodel::init(table)? };
//!
//! let env = rs_lunamodel::Env::new();
//! let x = rs_lunamodel::Expression::constant(&env, 2.0)?;
//! let y = (x * 3.0)?;
//! assert_eq!(y.offset()?, 6.0);
//! ```

#[macro_use]
mod macros;

mod buffer;
pub mod traits;
pub mod utils;

mod api;
mod assignment;
mod bound;
mod bounds;
mod cc;
mod column;
mod constr;
mod env;
pub mod error;
mod expr;
mod higher_order;
mod lazy_bounds;
mod linear;
mod model;
mod owned_env;
mod quadratic;
mod result_view;
mod sample_view;
mod sol;
mod specs;
mod term_types;
mod timer;
mod timing;
pub mod types;
mod variable;
mod vref;

pub use api::{InitError, abi_version, init, is_initialized, version};
pub use assignment::Assignment;
pub use bound::Bound;
pub use bounds::Bounds;
pub use cc::ConstraintCollection;
pub use column::{ColElement, Column};
pub use constr::Constraint;
pub use env::Env;
pub type ArcEnv = Env;
pub use error::{Error, Result, Status};
pub use expr::Expression;
pub use higher_order::HigherOrder;
pub use lazy_bounds::{Concretize, LazyBounds};
pub use linear::Linear;
pub use model::Model;
pub use owned_env::{Environment, VariableMut};
pub use quadratic::Quadratic;
pub use result_view::ResultView;
pub use sample_view::{SampleView, SampleViewIdx};
pub use sol::Solution;
pub use specs::Specs;
pub use term_types::{Neighborhood, OneVarTerm, TwoVarTerm};
pub use timer::Timer;
pub use timing::Timing;
pub use traits::{
    ContentEquality, DefaultEditable, Editable, FilterByMask, LmAddAssign, LmMulAssign, LmPow,
    LmPowAssign, LmSubAssign, Serialization, TryAddAssign, TryClone, TryDeepClone, TryIndex,
    TryMulAssign, TrySubAssign,
};
pub use variable::Variable;
pub use vref::VarRef;

/// Re-export of the ABI dispatch-table type, for obtaining and passing it to
/// [`init`].
pub use lunamodel_cext_vtable::LunaModelCApi;

pub use lunamodel_types::{
    Bias, BinaryAssignment, Comparator, Ctype, DEFAULT_BIAS, EnumSet, EnumSetFromVec, EnvIdx,
    IntegerAssignment, RealAssignment, Sense, SpinAssignment, ValueSource, VarId, VarIdx, VarName,
    Vtype,
};

/// Native-compatible paths for code written against `luna_model::core`.
pub mod core;
