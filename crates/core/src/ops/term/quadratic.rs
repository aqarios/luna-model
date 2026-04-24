//! Multiplication helpers for quadratic sparse term storage.

use lunamodel_types::Bias;

use crate::{
    ArcEnv,
    ops::{traits::internal::PrvMul, utils::VarMulRes},
    prelude::{HigherOrder, Linear, Quadratic, VarRef},
};

impl PrvMul<&VarRef> for &Quadratic {
    type Output = Vec<VarMulRes>;

    /// Multiplies each quadratic contribution by a single variable reference.
    fn m(self, rhs: &VarRef) -> Self::Output {
        self.iter_flat().map(|v| rhs.m(v)).collect()
    }
}

impl PrvMul<&VarRef> for &Option<Quadratic> {
    type Output = Option<Vec<VarMulRes>>;

    /// Multiplies optional quadratic storage by a variable reference.
    fn m(self, rhs: &VarRef) -> Self::Output {
        self.as_ref().map(|q| q.m(rhs))
    }
}

impl PrvMul<Bias> for &Quadratic {
    type Output = Vec<VarMulRes>;

    /// Scales every quadratic contribution by a scalar bias.
    fn m(self, rhs: Bias) -> Self::Output {
        self.iter_flat()
            .map(|(u, v, b)| VarMulRes::Quad((u, v, b * rhs)))
            .collect()
    }
}

impl PrvMul<Bias> for &Option<Quadratic> {
    type Output = Vec<VarMulRes>;

    /// Scales optional quadratic storage by a scalar bias.
    fn m(self, rhs: Bias) -> Self::Output {
        self.as_ref().map(|q| q.m(rhs)).unwrap_or_default()
    }
}

impl PrvMul<(&Linear, &ArcEnv)> for &Option<Quadratic> {
    type Output = Vec<VarMulRes>;

    /// Multiplies optional quadratic storage with linear storage.
    fn m(self, rhs: (&Linear, &ArcEnv)) -> Self::Output {
        let (lin, env) = rhs;
        lin.m((self, env))
    }
}

impl PrvMul<(&Quadratic, &ArcEnv)> for &Quadratic {
    type Output = Vec<VarMulRes>;

    /// Multiplies two quadratic storages, yielding higher-degree fragments.
    fn m(self, rhs: (&Quadratic, &ArcEnv)) -> Self::Output {
        let (q, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + q.len());
        for (u, v, b) in self.iter_flat() {
            let vref = &VarRef::new(u, env.clone());
            for (u2, v2, b2) in q.iter_flat() {
                let vs = vec![v, u2, v2];
                res.push(vref.m((vs, b2 * b)));
            }
        }
        res
    }
}

impl PrvMul<(&Option<Quadratic>, &ArcEnv)> for &Option<Quadratic> {
    type Output = Vec<VarMulRes>;

    /// Multiplies two optional quadratic storages.
    fn m(self, rhs: (&Option<Quadratic>, &ArcEnv)) -> Self::Output {
        let (q2, env) = rhs;
        self.as_ref()
            .map(|q| {
                q2.as_ref()
                    .map(|q2| q.m((q2, env)))
                    .unwrap_or_else(Vec::default)
            })
            .unwrap_or_default()
    }
}

impl PrvMul<(&HigherOrder, &ArcEnv)> for &Quadratic {
    type Output = Vec<VarMulRes>;

    /// Multiplies quadratic storage with higher-order storage.
    fn m(self, rhs: (&HigherOrder, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + ho.len());
        for (u, v, b) in self.iter_flat() {
            let vref = &VarRef::new(u, env.clone());
            for (mut vs, b2) in ho.iter_contrib() {
                vs.push(v);
                res.push(vref.m((vs, b * b2)));
            }
        }
        res
    }
}

impl PrvMul<(&Option<HigherOrder>, &ArcEnv)> for &Option<Quadratic> {
    type Output = Vec<VarMulRes>;

    /// Multiplies optional quadratic storage with optional higher-order storage.
    fn m(self, rhs: (&Option<HigherOrder>, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        self.as_ref()
            .map(|q| {
                ho.as_ref()
                    .map(|h2| q.m((h2, env)))
                    .unwrap_or_else(Vec::default)
            })
            .unwrap_or_default()
    }
}
