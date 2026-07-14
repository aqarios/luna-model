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

/// The name used when a model is created without an explicit name.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

/// Native-compatible paths for code written against `luna_model::core`.
pub mod core {
    pub use crate::{
        ArcEnv, Assignment, Bound, Bounds, ColElement, Column, Constraint, ConstraintCollection,
        Environment, Expression, HigherOrder, LazyBounds, Linear, Model, Quadratic, ResultView,
        SampleView, Solution, Specs, Timer, Timing, VarRef, Variable,
    };

    pub mod prelude {
        pub use crate::{
            ArcEnv, Bounds, Constraint, ConstraintCollection, ContentEquality, DefaultEditable,
            Editable, Environment, Expression, HigherOrder, LazyBounds, Linear, LmAddAssign,
            LmMulAssign, LmPow, LmPowAssign, LmSubAssign, Model, Quadratic, Solution, Timer,
            Timing, VarRef, Variable,
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
        pub use crate::{Assignment, ColElement, Column, ResultView, SampleView, Solution};
        pub mod result {
            pub use crate::ResultView;
        }
        pub mod sample {
            pub use crate::{SampleView, SampleViewIdx};
        }
        pub mod timing {
            pub use crate::{Timer, Timing};
        }
    }
    pub mod variable {
        pub use crate::{VarRef, Variable};
    }
    pub mod ops {
        pub use crate::{LmAddAssign, LmMulAssign, LmPow, LmPowAssign, LmSubAssign};

        /// Reduces repeated variables using their domain-specific product rules.
        pub fn reduce_vars_mul<F, I>(
            vars: &[crate::VarIdx],
            vtype: F,
            inverted: I,
        ) -> Option<Vec<crate::VarIdx>>
        where
            F: Fn(crate::VarIdx) -> crate::Vtype,
            I: Fn(crate::VarIdx) -> Option<crate::VarIdx>,
        {
            use std::collections::BTreeMap;

            let mut occurrences = BTreeMap::<crate::VarIdx, (usize, crate::Vtype)>::new();
            let mut order = Vec::new();
            for &var in vars {
                if inverted(var).is_some_and(|other| occurrences.contains_key(&other)) {
                    return None;
                }
                let entry = occurrences.entry(var).or_insert_with(|| {
                    order.push(var);
                    (0, vtype(var))
                });
                entry.0 += 1;
            }

            let mut reduced = Vec::new();
            for var in order {
                let (count, vtype) = occurrences[&var];
                match vtype {
                    crate::Vtype::Binary => reduced.push(var),
                    crate::Vtype::Spin if count % 2 == 1 => reduced.push(var),
                    crate::Vtype::Spin => {}
                    _ => reduced.extend(std::iter::repeat_n(var, count)),
                }
            }
            Some(reduced)
        }

        pub fn make_lookup<S>(
            env: &crate::Environment,
            vars: impl Iterator<Item = u32>,
            sample: &S,
            lookup: &mut Vec<crate::Bias>,
        ) -> crate::Result<()>
        where
            for<'a> S: crate::TryIndex<&'a str, Output = crate::Bias>,
        {
            let next = env.next_idx()? as usize;
            if lookup.len() < next {
                lookup.resize(next, 0.0);
            }
            for idx in vars {
                let var = env.get(idx)?;
                let value = if var.vtype()? == crate::Vtype::InvertedBinary {
                    let base = var.inverted()?.ok_or_else(|| crate::Error {
                        status: crate::Status::Absent,
                        message: format!(
                            "inverted variable {} has no base companion",
                            var.name().unwrap_or_default()
                        ),
                    })?;
                    1.0 - sample.try_index(&env.get(base)?.name()?)?
                } else {
                    sample.try_index(&var.name()?)?
                };
                lookup[idx as usize] = value;
            }
            Ok(())
        }
    }
}
