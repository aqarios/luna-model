use hashbrown::HashMap;
use lunamodel_types::EnvIdx;

use crate::ArcEnv;

use super::Expression;

impl Expression {
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        let mut out = self.clone();
        out.env = env;
        out
    }

    pub fn deep_clone_many(exprs: &[&Expression]) -> Vec<Expression> {
        let mut hm: HashMap<EnvIdx, ArcEnv> = HashMap::default();
        let mut v = Vec::new();
        for &expr in exprs {
            let newnev = hm
                .get(&expr.env.id())
                .and_then(|e| Some(e.clone()))
                .or_else(|| {
                    let out = expr.env.deep_clone();
                    hm.insert(expr.env.id(), out.clone());
                    Some(out)
                })
                .unwrap();
            let new = expr.deep_clone(newnev);
            v.push(new);
        }
        v
    }
}
