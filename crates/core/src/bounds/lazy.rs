use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Vtype};

use crate::bounds::Bounds;

#[derive(Clone)]
pub struct LazyBounds {
    pub(super) lower: Option<Bound>,
    pub(super) upper: Option<Bound>,
}

pub trait Concretize {
    fn concretize(self, vtype: &Vtype) -> LunaModelResult<Bounds>;
}

impl LazyBounds {
}

impl Concretize for Option<LazyBounds> {
    fn concretize(self, vtype: &Vtype) -> LunaModelResult<Bounds> {
        match (&vtype, &self) {
            (Vtype::Binary | Vtype::Spin, Some(_)) => Err(LunaModelError::InvalidBounds(
                format!("cannot set bounds for vtype {vtype}").into(),
            )),
            _ => Ok(()),
        }?;
        let default_bounds = Bounds::default_for(&vtype);
        let bounds = self.map_or(default_bounds, |b| match (b.lower, b.upper) {
            (Some(l), Some(u)) => Bounds::new(l, u),
            (Some(l), None) => Bounds::new(l, default_bounds.upper),
            (None, Some(u)) => Bounds::new(default_bounds.lower, u),
            (None, None) => default_bounds,
        });
        Ok(bounds)
    }
}
