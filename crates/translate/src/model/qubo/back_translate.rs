//! Back-translation from core models into dense QUBO data.

use std::collections::HashMap;

use lunamodel_core::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, VarIdx, Vtype};

use super::{Qubo, QuboTranslator};

impl QuboTranslator {
    pub fn back_translate(model: &Model) -> LunaModelResult<Qubo> {
        if model.objective.has_higher_order() {
            return Err(LunaModelError::ModelNotQuadratic);
        }
        if !model.constraints.is_empty() {
            return Err(LunaModelError::ModelNotUnconstrained);
        }
        // TODO: do we really want to return an error here forever?
        if !model.sense.is_min() {
            return Err(LunaModelError::ModelSenseNotMinimize);
        }

        let vtypes: Vec<_> = model.vtypes().collect();
        if vtypes.len() > 1 {
            return Err(LunaModelError::Vtype(
                "model's variables must be a single vtype".into(),
            ));
        }
        let vtype = vtypes.first().cloned();
        if let Some(vt) = vtype
            && !(vt == Vtype::Binary || vt == Vtype::Spin)
        {
            return Err(LunaModelError::Vtype(
                "model's vtype must be binary or spin".into(),
            ));
        }

        let vars: Vec<_> = model
            .environment
            .vars()
            .iter()
            .map(|v| v.name())
            .collect::<LunaModelResult<Vec<_>>>()?;

        let idx_map: HashMap<VarIdx, usize> = model
            .environment
            .vars()
            .iter()
            .enumerate()
            .map(|(i, v)| (v.id(), i))
            .collect();

        let mut dense = Vec::new();
        dense.resize(vars.len() * vars.len(), Bias::default());
        for (vs, bias) in model.objective.items() {
            match &vs[..] {
                [v] => dense[idx_map[&v.id()] * (vars.len() + 1)] = bias,
                [u, v] => {
                    dense[idx_map[&u.id()] * vars.len() + idx_map[&v.id()]] = bias * 0.5;
                    dense[idx_map[&v.id()] * vars.len() + idx_map[&u.id()]] = bias * 0.5;
                }
                _ => (),
            }
        }

        Ok(Qubo {
            sense: model.sense,
            name: model.name.clone(),
            vtype: vtype.unwrap_or(Vtype::Binary),
            matrix_flat: dense,
            num_variables: vars.len(),
            offset: model.objective.offset,
            variable_names: vars,
        })
    }
}
