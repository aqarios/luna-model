use lunamodel_types::Bias;

use super::Expression;
use super::term::Linear;
use crate::ArcEnv;

impl Expression {
    pub fn empty(env: ArcEnv) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::default(),
            quadratic: None,
            higher_order: None,
            num_vars: 0,
        }
    }
}
