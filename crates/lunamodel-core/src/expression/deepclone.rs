use crate::ArcEnv;

use super::Expression;

impl Expression {
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        let mut out = self.clone();
        out.env = env;
        out
    }
}
