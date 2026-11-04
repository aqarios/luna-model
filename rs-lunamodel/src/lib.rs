//! Rust wrapper over the LunaModel C API for native Rust extensions.
//!
//! This crate mirrors the native `luna-model` API but routes every call through
//! the stable C ABI. It depends only on [`lunamodel_capi`] for the
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) dispatch table and the
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
pub mod ffi;
pub mod plugin;
pub mod traits;
mod utils;

#[macro_use]
mod api;
mod assignment;
mod bound;
mod bounds;
mod cc;
mod column;
mod constr;
mod env;
pub mod error;
mod error_kind;
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
mod sol_lookup;
mod specs;
mod subtimer;
mod timer;
mod timing;
pub mod types;
mod variable;
mod vref;

#[cfg(feature = "py")]
#[macro_use]
pub mod py;

pub use api::{InitError, abi_version, host_table_size, init, is_initialized, version};
pub use assignment::Assignment;
pub use bound::Bound;
pub use bounds::Bounds;
pub use cc::{ConstraintCollection, ConstraintCollectionEditor};
pub use column::Column;
pub use constr::Constraint;
pub use env::Env;
pub type ArcEnv = Env;
pub use error::{Error, LunaModelError, LunaModelResult, Result, Status};
pub use error_kind::ErrorKind;
pub use expr::Expression;
pub use higher_order::{HigherOrder, HigherOrderEditor};
pub use lazy_bounds::{Concretize, LazyBounds};
pub use linear::{Linear, LinearEditor};
pub use model::Model;
pub use owned_env::{Environment, VariableMut};
pub use quadratic::{Quadratic, QuadraticEditor};
pub use result_view::ResultView;
pub use sample_view::{SampleView, SampleViewIdx};
pub use sol::Solution;
pub use sol_lookup::SolutionLookup;
pub use specs::Specs;
pub use subtimer::SubTimer;
pub use timer::Timer;
pub use timing::Timing;
pub use traits::{
    ContentEquality, DefaultEditable, Editable, LmAddAssign, LmMulAssign, LmPow, LmPowAssign,
    LmSubAssign, Serialization, TryAddAssign, TryClone, TryDeepClone, TryIndex, TryMulAssign,
    TrySubAssign,
};
pub use variable::Variable;
pub use vref::VarRef;

pub use prost;

/// Re-export of the ABI dispatch-table type, for obtaining and passing it to
/// [`init`].
pub use lunamodel_capi::LunaModelCApi;

/// The C API ABI version this SDK was built for; [`init`] accepts only a host table of exactly
/// this version.
pub use lunamodel_capi::CAPI_ABI_VERSION;

/// Size in bytes of the base C API table of [`CAPI_ABI_VERSION`] (as released in luna-model
/// 0.8.0a1); every host provides at least these entry points, see [`host_table_size`].
pub use lunamodel_capi::CAPI_BASE_TABLE_SIZE;

pub use lunamodel_types::{
    Bias, BinaryAssignment, Comparator, Ctype, DEFAULT_BIAS, EnumSet, EnumSetFromVec, EnvIdx,
    IntegerAssignment, Neighborhood, OneVarTerm, RealAssignment, Sense, SpinAssignment, TwoVarTerm,
    ValueSource, VarId, VarIdx, VarName, Vtype,
};

/// The name used when a model is created without an explicit name.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

/// Native-compatible paths for code written against `luna_model::core`.
pub mod core {
    pub use crate::{
        ArcEnv, Assignment, Bound, Bounds, Column, Constraint, ConstraintCollection, Environment,
        Expression, HigherOrder, LazyBounds, Linear, Model, Neighborhood, OneVarTerm, Quadratic,
        ResultView, SampleView, Solution, Specs, SubTimer, Timer, Timing, TwoVarTerm, VarRef,
        Variable,
    };

    pub mod prelude {
        pub use crate::{
            ArcEnv, Bounds, Constraint, ConstraintCollection, ContentEquality, DefaultEditable,
            Editable, Environment, Expression, HigherOrder, LazyBounds, Linear, LmAddAssign,
            LmMulAssign, LmPow, LmPowAssign, LmSubAssign, Model, Quadratic, Solution, SubTimer,
            Timer, Timing, VarRef, Variable,
        };
    }

    pub mod bounds {
        pub use crate::{Bound, Bounds, Concretize, LazyBounds};
    }
    pub mod constraint {
        pub use crate::{Constraint, ConstraintCollection};
    }
    pub mod environment {
        pub use crate::{ArcEnv, Environment};
    }
    pub mod expression {
        pub use crate::Expression;
        pub mod term {
            pub use crate::{HigherOrder, Linear, Quadratic};
            pub mod types {
                pub use crate::{Neighborhood, OneVarTerm, TwoVarTerm};
            }
        }
    }
    pub mod model {
        pub use crate::{DEFAULT_MODEL_NAME, Model, Specs};
    }
    pub mod solution {
        pub use crate::{Assignment, Column, ResultView, SampleView, Solution};
        pub mod result {
            pub use crate::ResultView;
        }
        pub mod sample {
            pub use crate::{SampleView, SampleViewIdx};
        }
        pub mod timing {
            pub use crate::{SubTimer, Timer, Timing};
        }
    }
    pub mod variable {
        pub use crate::{VarRef, Variable};
    }
    pub mod ops {
        pub use crate::{LmAddAssign, LmMulAssign, LmPow, LmPowAssign, LmSubAssign};
    }
}
