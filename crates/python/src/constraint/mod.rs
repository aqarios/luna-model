//! Python wrappers for constraints and constraint collections.
mod coll;
mod constr;

pub use coll::{
    PyConstraintCollection, PyConstraintCollectionContent, PyConstraintCollectionIterator, utils,
};
pub use constr::PyConstraint;
