//! Python wrappers for concrete and lazy bounds.
mod access;
mod cmp;
mod creation;
mod io;
mod unbounded;

use std::sync::Arc;

use lunamodel_core::prelude::{Bounds, LazyBounds};
use lunamodel_io::{CustomFormat, FormatOpt};
use parking_lot::RwLock;
use pyo3::pyclass;

pub use unbounded::{BoundValue, PyUnbounded};

/// Shared storage behind the Python bounds wrapper.
#[derive(Clone, Debug)]
pub enum BoundsContent {
    /// Fully concrete bounds.
    Concrete(Bounds),
    /// Bounds that still preserve optional endpoints.
    Lazy(LazyBounds),
}

pub type PyBoundsContent = Arc<RwLock<BoundsContent>>;

/// Python-visible bounds wrapper used across the binding layer.
#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
pub struct PyBounds(pub PyBoundsContent);

impl From<BoundsContent> for PyBounds {
    /// Wraps shared bounds content as `PyBounds`.
    fn from(value: BoundsContent) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl From<Bounds> for PyBounds {
    /// Wraps concrete bounds as `PyBounds`.
    fn from(bounds: Bounds) -> Self {
        let content: BoundsContent = bounds.into();
        content.into()
    }
}

impl From<LazyBounds> for PyBounds {
    /// Wraps lazy bounds as `PyBounds`.
    fn from(bounds: LazyBounds) -> Self {
        let content: BoundsContent = bounds.into();
        content.into()
    }
}

impl From<Bounds> for BoundsContent {
    /// Stores concrete bounds in the shared bounds enum.
    fn from(bounds: Bounds) -> Self {
        Self::Concrete(bounds)
    }
}

impl From<LazyBounds> for BoundsContent {
    /// Stores lazy bounds in the shared bounds enum.
    fn from(bounds: LazyBounds) -> Self {
        Self::Lazy(bounds)
    }
}

impl From<BoundsContent> for LazyBounds {
    /// Normalizes any bounds content into a lazy-bounds representation.
    fn from(bounds: BoundsContent) -> Self {
        match bounds {
            BoundsContent::Lazy(lazy) => lazy,
            BoundsContent::Concrete(conc) => conc.into(),
        }
    }
}

impl CustomFormat<FormatOpt> for BoundsContent {
    /// Delegates display formatting to the active inner bounds representation.
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.fmt(fmt, format_type),
            Self::Lazy(l) => l.fmt(fmt, format_type),
        }
    }

    /// Delegates debug formatting to the active inner bounds representation.
    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.dbg(fmt, format_type),
            Self::Lazy(l) => l.dbg(fmt, format_type),
        }
    }
}
