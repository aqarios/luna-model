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

#[derive(Clone, Debug)]
pub enum BoundsContent {
    Concrete(Bounds),
    Lazy(LazyBounds),
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyBounds(pub Arc<RwLock<BoundsContent>>);

impl From<BoundsContent> for PyBounds {
    fn from(value: BoundsContent) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl From<Bounds> for PyBounds {
    fn from(bounds: Bounds) -> Self {
        let content: BoundsContent = bounds.into();
        content.into()
    }
}

impl From<LazyBounds> for PyBounds {
    fn from(bounds: LazyBounds) -> Self {
        let content: BoundsContent = bounds.into();
        content.into()
    }
}

impl From<Bounds> for BoundsContent {
    fn from(bounds: Bounds) -> Self {
        Self::Concrete(bounds)
    }
}

impl From<LazyBounds> for BoundsContent {
    fn from(bounds: LazyBounds) -> Self {
        Self::Lazy(bounds)
    }
}

impl From<BoundsContent> for LazyBounds {
    fn from(bounds: BoundsContent) -> Self {
        match bounds {
            BoundsContent::Lazy(lazy) => lazy,
            BoundsContent::Concrete(conc) => conc.into(),
        }
    }
}

impl CustomFormat<FormatOpt> for BoundsContent {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.fmt(fmt, format_type),
            Self::Lazy(l) => l.fmt(fmt, format_type),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.dbg(fmt, format_type),
            Self::Lazy(l) => l.dbg(fmt, format_type),
        }
    }
}
