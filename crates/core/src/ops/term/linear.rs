use lunamodel_types::Bias;

use crate::{
    ArcEnv,
    ops::{traits::internal::PrvMul, utils::VarMulRes},
    prelude::{HigherOrder, Linear, Quadratic, VarRef},
};

impl PrvMul<&VarRef> for &Linear {
    /// I'd like to change the return type to this at some point in the
    /// future but that's currently unstable. So we have to wait a bit.
    /// Or we allow it explicitly...
    ///
    ///   type Output = LunaModelResult<impl Iterator<Item = VarMulRes>>;
    ///   `impl Trait` in associated types is unstable
    ///   see issue #63063 <https://github.com/rust-lang/rust/issues/63063>
    ///   for more information [E0658]
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: &VarRef) -> Self::Output {
        self.iter().map(|v| rhs.m(v)).collect()
    }
}

impl PrvMul<Bias> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: Bias) -> Self::Output {
        self.iter()
            .map(|(v, b)| VarMulRes::Lin((v, b * rhs)))
            .collect()
    }
}

// trait L<A> {
//     fn len(&self) -> usize;
//     fn iter(&self) -> dyn Iterator<Item = A>;
// }
//
// impl<T, A> PrvMul<(&T, &ArcEnv)> for &Linear
// where
//     for<'i> &'i T: L<A>,
//     for<'v> &'v VarRef: PrvMul<A, Output = VarMulRes>,
// {
//     type Output = Vec<VarMulRes>;
//
//     fn m(self, rhs: (&T, &ArcEnv)) -> Self::Output {
//         let (r, env) = rhs;
//         let mut res = Vec::with_capacity(self.len() + r.len());
//         for (u, ub) in self.iter() {
//             let vref = &VarRef::new(u, env.clone());
//             for o in &mut r.iter() {
//                 res.push(vref.m(o) * ub);
//             }
//         }
//         res
//     }
// }

impl PrvMul<(&Linear, &ArcEnv)> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Linear, &ArcEnv)) -> Self::Output {
        let (lin, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + lin.len());
        for (u, ub) in self.iter() {
            let vref = &VarRef::new(u, env.clone());
            for other in lin.iter() {
                res.push(vref.m(other) * ub);
            }
        }
        res
    }
}

impl PrvMul<(&Quadratic, &ArcEnv)> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Quadratic, &ArcEnv)) -> Self::Output {
        let (quad, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + quad.len());
        for (u, ub) in self.iter() {
            let vref = &VarRef::new(u, env.clone());
            for other in quad.iter_flat() {
                res.push(vref.m(other) * ub);
            }
        }
        res
    }
}

impl PrvMul<(&Option<Quadratic>, &ArcEnv)> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Option<Quadratic>, &ArcEnv)) -> Self::Output {
        let (quad, env) = rhs;
        quad.as_ref()
            .map(|q| self.m((q, env)))
            .unwrap_or_else(|| Vec::default())
    }
}

impl PrvMul<(&HigherOrder, &ArcEnv)> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&HigherOrder, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        let mut res = Vec::with_capacity(self.len() + ho.len());
        for (u, ub) in self.iter() {
            let vref = &VarRef::new(u, env.clone());
            for other in ho.iter_contrib() {
                res.push(vref.m(other) * ub);
            }
        }
        res
    }
}

impl PrvMul<(&Option<HigherOrder>, &ArcEnv)> for &Linear {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: (&Option<HigherOrder>, &ArcEnv)) -> Self::Output {
        let (ho, env) = rhs;
        ho.as_ref()
            .map(|h| self.m((h, env)))
            .unwrap_or_else(|| Vec::default())
    }
}
