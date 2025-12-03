use lunamodel_core::prelude::{Bounds, LazyBounds};
use pyo3::pyclass;

#[derive(Clone)]
pub enum BoundsContent {
    Concrete(Bounds),
    Lazy(LazyBounds),
}

#[pyclass]
#[derive(Clone)]
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
