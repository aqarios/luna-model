use lunamodel_types::Bias;

use crate::{
    ArcEnv,
    ops::{traits::internal::PrvMul, utils::VarMulRes},
    prelude::{HigherOrder, Linear, Quadratic, VarRef},
};

impl PrvMul<&VarRef> for &HigherOrder {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: &VarRef) -> Self::Output {
        self.iter_contrib().map(|v| rhs.m(v)).collect()
    }
}

impl PrvMul<&VarRef> for &Option<HigherOrder> {
    type Output = Option<Vec<VarMulRes>>;

    fn m(self, rhs: &VarRef) -> Self::Output {
        self.as_ref().map(|h| h.m(rhs))
    }
}

impl PrvMul<Bias> for &HigherOrder {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: Bias) -> Self::Output {
        self.iter_contrib()
            .map(|(c, b)| VarMulRes::HiOr((c, b * rhs)))
            .collect()
    }
}

impl PrvMul<Bias> for &Option<HigherOrder> {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: Bias) -> Self::Output {
        self.as_ref()
            .map(|h| h.m(rhs))
            .unwrap_or_else(|| Vec::default())
    }
}

impl PrvMul<(&Linear, &ArcEnv)> for &Option<HigherOrder> {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Linear, &ArcEnv)) -> Self::Output {
        let (lin, env) = rhs;
        lin.m((self, env))
    }
}

impl PrvMul<(&Option<Quadratic>, &ArcEnv)> for &Option<HigherOrder> {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Option<Quadratic>, &ArcEnv)) -> Self::Output {
        let (quad, env) = rhs;
        quad.m((self, env))
    }
}

impl PrvMul<(&HigherOrder, &ArcEnv)> for &HigherOrder {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&HigherOrder, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + ho.len());
        for (mut us, ub) in self.iter_contrib() {
            let vref = &VarRef::new(us.pop().unwrap(), env.clone());
            for (mut vs, vb) in ho.iter_contrib() {
                vs.append(&mut vs.clone());
                res.push(vref.m((vs, ub * vb)));

            }
        }
        res
    }
}

impl PrvMul<(&Option<HigherOrder>, &ArcEnv)> for &Option<HigherOrder> {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Option<HigherOrder>, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        self.as_ref()
            .map(|s| {
                ho.as_ref()
                    .map(|h| s.m((h, env)))
                    .unwrap_or_else(|| Vec::default())
            })
            .unwrap_or_else(|| Vec::default())
    }
}
