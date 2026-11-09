//! What the LP and MPS writers share: the variables in model order and the output.

use std::{fs, path::PathBuf};

use lunamodel_core::{Expression, Model};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, VarIdx, Vtype};

use super::finite::check_finite;

pub struct Var {
    pub idx: VarIdx,
    pub name: String,
    pub vtype: Vtype,
    pub lower: Bound,
    pub upper: Bound,
}

/// The model's variables in model order, found by index via `position`.
pub struct Vars {
    pub list: Vec<Var>,
    position: Vec<usize>, // by VarIdx
}

impl Vars {
    pub fn position(&self, idx: VarIdx) -> usize {
        self.position[idx as usize]
    }

    pub fn name(&self, idx: VarIdx) -> &str {
        &self.list[self.position(idx)].name
    }
}

/// Collects the variables, rejecting what neither LP nor MPS files can express.
pub fn prepare(model: &Model, format: &str) -> LunaModelResult<Vars> {
    let higher_order = |e: &Expression| {
        e.higher_order
            .as_ref()
            .is_some_and(|h| h.iter().any(|(_, b)| b != 0.0))
    };
    if higher_order(model.objective())
        || model
            .constraints()
            .iter()
            .any(|(_, c)| higher_order(&c.lhs))
    {
        return Err(LunaModelError::Translation(
            format!("{format} files cannot contain higher-order terms").into(),
        ));
    }
    check_finite(model)?;

    let mut list = Vec::new();
    for v in model.environment.vars() {
        let vtype = v.vtype()?;
        if matches!(vtype, Vtype::Spin | Vtype::InvertedBinary) {
            let msg = format!("{format} files cannot contain variables of type {vtype:?}");
            return Err(LunaModelError::Translation(msg.into()));
        }
        let bounds = v.bounds()?;
        list.push(Var {
            idx: v.id(),
            name: v.name()?,
            vtype,
            lower: bounds.lower,
            upper: bounds.upper,
        });
    }

    let size = list.iter().map(|v| v.idx as usize + 1).max().unwrap_or(0);
    let mut position = vec![usize::MAX; size];
    for (pos, var) in list.iter().enumerate() {
        position[var.idx as usize] = pos;
    }
    Ok(Vars { list, position })
}

/// Rejects the first variable or constraint name that `invalid` gives a reason for,
/// as the file would not read back.
pub fn check_names(
    model: &Model,
    vars: &Vars,
    format: &str,
    invalid: impl Fn(&str) -> Option<&'static str>,
) -> LunaModelResult<()> {
    let vars = vars.list.iter().map(|v| ("variable", v.name.as_str()));
    let constraints = model.constraints().iter();
    let constraints = constraints.map(|(name, _)| ("constraint", name.as_str()));
    for (kind, name) in vars.chain(constraints) {
        if let Some(reason) = invalid(name) {
            let msg = format!("{format} files cannot contain the {kind} name '{name}': {reason}");
            return Err(LunaModelError::Translation(msg.into()));
        }
    }
    Ok(())
}

/// The expression's non-zero quadratic terms.
pub fn quadratic(expr: &Expression) -> impl Iterator<Item = (VarIdx, VarIdx, Bias)> + '_ {
    let terms = expr.quadratic.iter().flat_map(|q| q.iter_flat());
    terms.filter(|&(_, _, b)| b != 0.0)
}

/// Writes `text` to `filepath` if given, and returns it otherwise.
pub fn output(text: String, filepath: Option<PathBuf>) -> LunaModelResult<Option<String>> {
    match filepath {
        Some(path) => {
            fs::write(path, text).map_err(|e| LunaModelError::Translation(e.to_string().into()))?;
            Ok(None)
        }
        None => Ok(Some(text)),
    }
}
