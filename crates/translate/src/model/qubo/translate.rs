use lunamodel_core::{Expression, Model};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Sense, Vtype};

use super::QuboTranslator;

impl QuboTranslator {
    pub fn translate(
        dense: &[Bias],
        num_vars: usize,
        vtype: Option<Vtype>,
        offset: Option<Bias>,
        variable_names: Option<Vec<String>>,
        name: Option<String>,
    ) -> LunaModelResult<Model> {
        let mut model = Model::new(name, Some(Sense::Min));
        if let Some(vnames) = variable_names.as_ref()
            && vnames.len() != num_vars
        {
            return Err(LunaModelError::VariableNames(
                format!("number of variable names does not match number of variables: is {}, expected {}", vnames.len(), num_vars).into(),
            ));
        }
        for i in 0..num_vars {
            let vname = match &variable_names {
                None => &format!("x_{i}"),
                Some(names) => &names[i],
            };
            model
                .environment
                .insert(vname, vtype.unwrap_or_else(|| Vtype::Binary), None)?;
        }
        model.objective =
            Expression::from_dense_quadratic(dense, num_vars, offset, model.environment.clone())?;
        Ok(model)
    }
}
