use lunamodel_core::prelude::{Bounds, VarRef};
use lunamodel_transpiler::TranspileKindResult;
use lunamodel_types::{Bias, Bound};

pub(super) fn compute_minvalue(
    items: impl Iterator<Item = (VarRef, Bias)>,
) -> TranspileKindResult<Bound> {
    let mut sum = 0f64;
    for (v, bias) in items {
        let Bounds { lower, upper } = v.bounds()?;
        let Some(val) = extract(lower, upper, bias) else {
            return Ok(Bound::Unbounded);
        };
        sum += val;
    }
    Ok(Bound::Bounded(sum))
}

pub(super) fn compute_maxvalue(
    items: impl Iterator<Item = (VarRef, Bias)>,
) -> TranspileKindResult<Bound> {
    let mut sum = 0f64;
    for (v, bias) in items {
        let Bounds { lower, upper } = v.bounds()?;
        let Some(val) = extract(upper, lower, bias) else {
            return Ok(Bound::Unbounded);
        };
        sum += val;
    }
    Ok(Bound::Bounded(sum))
}

fn extract(ge: Bound, le: Bound, bias: f64) -> Option<f64> {
    match if bias >= 0.0 { ge } else { le } {
        Bound::Bounded(value) => Some(bias * value),
        Bound::Unbounded => None,
    }
}
