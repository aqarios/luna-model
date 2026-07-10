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
mod traits;
mod utils;

mod api;
mod assignment;
mod bounds;
mod cc;
mod column;
mod env;
mod error;
mod expr;
mod higher_order;
mod lazy_bounds;
mod linear;
mod model;
mod quadratic;
mod result_view;
mod sample_view;
mod sol;
mod specs;
mod timing;
mod types;
mod variable;
mod vref;

pub use api::{InitError, init, is_initialized};
pub use assignment::Assignment;
pub use bounds::Bounds;
pub use cc::ConstraintCollection;
pub use column::Column;
pub use env::Env;
pub use error::{Error, Result, Status};
pub use expr::Expression;
pub use higher_order::HigherOrder;
pub use lazy_bounds::LazyBounds;
pub use linear::Linear;
pub use model::Model;
pub use quadratic::Quadratic;
pub use result_view::ResultView;
pub use sample_view::SampleView;
pub use sol::Solution;
pub use specs::Specs;
pub use timing::Timing;
pub use traits::{ContentEquality, Serialization, TryClone, TryDeepClone};
pub use vref::VarRef;

/// Re-export of the ABI dispatch-table type, for obtaining and passing it to
/// [`init`].
pub use lunamodel_cext_vtable::LunaModelCApi;
