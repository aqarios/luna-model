use lunamodel_core::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Vtype};

use super::{Qubo, QuboTranslator};

impl QuboTranslator {
    pub fn back_translate(model: &Model) -> LunaModelResult<Qubo> {
        if model.objective.has_higher_order() {
            return Err(LunaModelError::ModelNotQuadratic);
        }
        if !model.constraints.is_empty() {
            return Err(LunaModelError::ModelNotUnconstrained);
        }
        // if !model.sense.is_min() {
        //     return Err(LunaModelError::ModelSenseNotMinimize);
        // }

        let vtypes: Vec<_> = model.vtypes().collect();
        if vtypes.len() > 1 {
            return Err(LunaModelError::Vtype(
                "model's variables must be a single vtype".into(),
            ));
        }
        let vtype = vtypes.get(0).cloned();
        let vars: Vec<_> = model
            .environment
            .vars()
            .iter()
            .map(|v| v.name())
            .collect::<LunaModelResult<Vec<_>>>()?;

        let mut dense = Vec::new();
        dense.resize(vars.len() * vars.len(), Bias::default());
        for (vs, bias) in model.objective.items() {
            match &vs[..] {
                [v] => {
                    dense[v.id() as usize * (vars.len() + 1)] = bias
                }
                [u, v] => {
                    dense[u.id() as usize * vars.len() + v.id() as usize] = bias * 0.5;
                    dense[v.id() as usize * vars.len() + u.id() as usize] = bias * 0.5;
                }
                _ => (),
            }
        }

        Ok(Qubo {
            sense: model.sense,
            name: model.name.clone(),
            vtype: vtype.unwrap_or_else(|| Vtype::Binary),
            matrix_flat: dense,
            num_variables: vars.len(),
            offset: model.objective.offset,
            variable_names: vars,
        })
    }
}
