mod access;
mod cmp;
mod creation;
mod io;
mod unbounded;

use lunamodel_core::prelude::{Bounds, LazyBounds};
use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pyclass;

pub use unbounded::PyUnbounded;

#[derive(Clone, Debug)]
pub enum BoundsContent {
    Concrete(Bounds),
    Lazy(LazyBounds),
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyBounds(pub BoundsContent);

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
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.fmt(fmt, format_type),
            Self::Lazy(l) => l.fmt(fmt, format_type),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: FormatOpt) -> std::fmt::Result {
        match self {
            Self::Concrete(c) => c.dbg(fmt, format_type),
            Self::Lazy(l) => l.dbg(fmt, format_type),
        }
    }
}
