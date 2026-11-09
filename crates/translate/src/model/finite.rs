//! Rejects infinite and NaN numbers in expressions, which LP and MPS cannot express.

use lunamodel_core::{Expression, Model};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

pub fn check_finite(model: &Model) -> LunaModelResult<()> {
    if let Some(n) = non_finite(model.objective()) {
        return Err(error(format!("the objective contains {n}")));
    }
    for (name, c) in model.constraints().iter() {
        let rhs = Some(c.rhs).filter(|n| !n.is_finite());
        if let Some(n) = non_finite(&c.lhs).or(rhs) {
            return Err(error(format!("constraint '{name}' contains {n}")));
        }
    }
    Ok(())
}

fn non_finite(expr: &Expression) -> Option<Bias> {
    let linear = expr.linear.iter().map(|(_, c)| c);
    let quadratic = expr.quadratic.iter().flat_map(|q| q.iter_flat());
    let quadratic = quadratic.map(|(_, _, c)| c);
    let mut numbers = std::iter::once(expr.offset).chain(linear).chain(quadratic);
    numbers.find(|n| !n.is_finite())
}

fn error(msg: String) -> LunaModelError {
    LunaModelError::Translation(format!("{msg}, which is not a finite number").into())
}
