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
        }
    }

    pub fn constant(env: ArcEnv, val: Bias) -> Self {
        let mut slf = Self::empty(env);
        slf.offset += val;
        slf
    }
}
