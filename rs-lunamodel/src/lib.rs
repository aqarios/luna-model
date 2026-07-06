//! Rust wrapper over the LunaModel C API — the Rust twin of `cxx-lunamodel`.
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
//! let y = x.mul_bias(3.0)?;
//! assert_eq!(y.offset()?, 6.0);
//! ```

mod traits;

mod api;
mod env;
mod error;
mod expr;
mod types;

pub use api::{InitError, init, is_initialized};
pub use env::Env;
pub use error::{Error, Result, Status};
pub use expr::Expression;
pub use traits::{ContentEquality, Serialization, TryClone, TryDeepClone};

/// Re-export of the ABI dispatch-table type, for obtaining and passing it to
/// [`init`].
pub use lunamodel_cext_vtable::LunaModelCApi;
