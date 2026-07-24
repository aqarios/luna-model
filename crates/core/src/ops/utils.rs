//! Shared helper types and functions used by operator implementations.

use std::fmt::Debug;
use std::ops::Mul;

use crate::{
    Environment, Expression, Solution, TryIndex, prelude::{HigherOrder, Linear, Quadratic, VarRef}, solution::sample::SampleView, traits::DefaultEditable
};
use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, EnvIdx, VarIdx, Vtype, Vtype::*};

pub(crate) trait EnvIdexable {
    /// Returns the identity of the environment the value belongs to.
    fn env_id(&self) -> EnvIdx;
}

/// Verifies that two operands belong to the same environment.
///
/// Many algebraic operations are only meaningful when both operands reference
/// the same variable universe. This helper centralizes that check so the
/// operator impls stay focused on the algebra itself.
pub(crate) fn check_envs<A, B>(a: &A, b: &B) -> LunaModelResult<()>
where
    A: EnvIdexable + Debug,
    B: EnvIdexable + Debug,
{
    if a.env_id() != b.env_id() {
        Err(LunaModelError::DifferentEnvironments)
    } else {
        Ok(())
    }
}

impl EnvIdexable for Expression {
    fn env_id(&self) -> EnvIdx {
        self.env.id()
    }
}

impl EnvIdexable for VarRef {
    fn env_id(&self) -> EnvIdx {
        self.env.id()
    }
}

#[derive(Debug)]
pub enum VarMulRes {
    /// Multiplication collapsed to a constant contribution.
    Const(Bias),
    /// Multiplication collapsed to a single linear term.
    Lin((VarIdx, Bias)),
    /// Multiplication produced a quadratic term.
    Quad((VarIdx, VarIdx, Bias)),
    /// Multiplication produced a higher-order term with an arbitrary variable list.
    HiOr((Vec<VarIdx>, Bias)),
}

impl From<(VarIdx, VarIdx, Bias)> for VarMulRes {
    fn from(value: (VarIdx, VarIdx, Bias)) -> Self {
        let (u, v, b) = value;
        Self::Quad((u, v, b))
    }
}

impl From<VarMulRes> for Expression {
    /// Promotes a low-level multiplication fragment into a full expression.
    ///
    /// This is the bridge between the storage-oriented multiplication helpers
    /// and the public expression API.
    fn from(val: VarMulRes) -> Self {
        match val {
            VarMulRes::Const(b) => b.into(),
            VarMulRes::Lin((u, b)) => Linear::with(|l| l[u] = b).into(),
            VarMulRes::Quad((u, v, b)) => Quadratic::with(|q| q[(u, v)] = b).into(),
            VarMulRes::HiOr((vs, b)) => HigherOrder::with(|h| h[vs.as_slice()] = b).into(),
        }
    }
}

impl From<(Vec<u32>, Bias)> for VarMulRes {
    /// Chooses the most specific fragment variant for a normalized variable list.
    fn from(value: (Vec<u32>, Bias)) -> Self {
        let (vars, b) = value;
        match *vars.as_slice() {
            [] => Self::Const(b),
            [u] => Self::Lin((u, b)),
            [u, v] => Self::Quad((u, v, b)),
            _ => Self::HiOr((vars, b)),
        }
    }
}

impl Mul<Bias> for VarMulRes {
    type Output = VarMulRes;

    /// Scales the stored coefficient while keeping the fragment shape unchanged.
    fn mul(self, rhs: Bias) -> Self::Output {
        match self {
            Self::Const(b) => Self::Const(b * rhs),
            Self::Lin((u, b)) => Self::Lin((u, b * rhs)),
            Self::Quad((u, v, b)) => Self::Quad((u, v, b * rhs)),
            Self::HiOr((vars, b)) => Self::HiOr((vars, b * rhs)),
        }
    }
}

// pub struct Const {
//     pub b: Bias,
// }
// struct Lin {
//     pub u: VarIdx,
//     pub b: Bias,
// }
// struct Quad {
//     pub u: VarIdx,
//     pub v: VarIdx,
//     pub b: Bias,
// }
// struct HiOr {
//     pub vars: Vec<VarIdx>,
//     pub b: Bias,
// }
//
// pub enum VarMulRes {
//     Const(Const),
//     Lin(Lin),
//     Quad(Quad),
//     HiOr(HiOr),
// }
//
// impl From<(VarIdx, VarIdx, Bias)> for VarMulRes {
//     fn from(value: (VarIdx, VarIdx, Bias)) -> Self {
//         let (u, v, b) = value;
//         Self::Quad(Quad { u, v, b })
//     }
// }
//
// impl Into<Expression> for VarMulRes {
//     fn into(self) -> Expression {
//         match self {
//             Self::Const(Const { b: c }) => c.into(),
//             Self::Lin(Lin { u, b }) => Linear::with(|l| l[u] = b).into(),
//             Self::Quad(Quad { u, v, b }) => Quadratic::with(|q| q[(u, v)] = b).into(),
//             Self::HiOr(HiOr { vars: vs, b }) => HigherOrder::with(|h| h[vs.as_slice()] = b).into(),
//         }
//     }
// }
//
// impl From<(Vec<u32>, Bias)> for VarMulRes {
//     fn from(value: (Vec<u32>, Bias)) -> Self {
//         let (vars, b) = value;
//         match vars.as_slice() {
//             &[] => Self::Const(Const { b }),
//             &[u] => Self::Lin(Lin { u, b }),
//             &[u, v] => Self::Quad(Quad { u, v, b }),
//             _ => Self::HiOr(HiOr { vars, b }),
//         }
//     }
// }
//
// impl Mul<Bias> for VarMulRes {
//     type Output = VarMulRes;
//
//     fn mul(self, rhs: Bias) -> Self::Output {
//         match self {
//             Self::Const(Const { b }) => Self::Const(Const { b: b * rhs }),
//             Self::Lin(Lin { u, b }) => Self::Lin(Lin { u, b: b * rhs }),
//             Self::Quad(Quad { u, v, b }) => Self::Quad(Quad { u, v, b: b * rhs }),
//             Self::HiOr(HiOr { vars, b }) => Self::HiOr(HiOr { vars, b: b * rhs }),
//         }
//     }
// }

// pub struct WithMutRes<'o, T, Rhs, Out>
// where
//     T: Mul<Rhs>,
// {
//     rhs: Rhs,
//     out: &'o mut Out,
//     _p: PhantomData<T>,
// }
//
// pub struct LazyWMR<'o, Out> {
//     out: &'o mut Out,
// }
//
// impl<'o, Out> LazyWMR<'o, Out> {
//     pub fn builder(out: &'o mut Out) -> Self {
//         Self { out }
//     }
//
//     pub fn build<Rhs, T: Mul<Rhs>>(&'o mut self, rhs: Rhs) -> WithMutRes<'o, T, Rhs, Out> {
//         WithMutRes {
//             rhs: rhs,
//             out: &mut self.out,
//             _p: PhantomData::default(),
//         }
//     }
// }

/// Reduces a variable multiset to its canonical multiplicative representation.
///
/// This performs the variable-domain-specific simplifications that LunaModel
/// relies on during multiplication:
///
/// - repeated binary variables collapse to a single factor because `x * x = x`,
/// - repeated spin variables cancel pairwise because `s * s = 1`, and
/// - encountering both a binary variable and its explicit inverted companion
///   collapses the whole product to zero, represented as `None`.
///
/// The returned variable list is not sorted for cosmetic reasons; it is shaped
/// to preserve the information needed by later canonical storage layers.
pub fn reduce_vars_mul<F, I>(vars: &[VarIdx], vtype: F, inv: I) -> Option<Vec<VarIdx>>
where
    F: Fn(VarIdx) -> Vtype,
    I: Fn(VarIdx) -> Option<VarIdx>,
{
    let mut ocs: IndexMap<VarIdx, (usize, Vtype)> = IndexMap::new();
    for &v in vars {
        if let Some(inverted) = inv(v)
            && ocs.contains_key(&inverted)
        {
            return None;
        }
        ocs.entry(v).or_insert((0, vtype(v))).0 += 1;
    }
    let mut reduced: Vec<VarIdx> = Vec::new();
    for (idx, (count, vt)) in ocs {
        match (count, vt) {
            (_, Binary) => reduced.push(idx),
            (c, Spin) => {
                if c % 2 == 1 {
                    reduced.push(idx)
                }
            }
            (c, _) => {
                for _ in 0..c {
                    reduced.push(idx);
                }
            }
        }
    }
    Some(reduced)
}

pub struct Lookup {
    pub lu: Vec<Bias>,
    vids: Vec<u32>,
    vnames: Vec<String>,
    inverted: Vec<bool>,
}

impl Lookup {
    pub fn new(env: &Environment, vars: impl Iterator<Item = u32>) -> LunaModelResult<Self> {
        let lu = vec![0.0; env.next_idx as usize];

        let vids: Vec<u32> = vars.collect();
        let mut vnames: Vec<String> = Vec::with_capacity(vids.len());
        let mut inverted = Vec::with_capacity(vids.len());
        for i in vids.iter() {
            let v = &env.variables[i];
            let (vname, inv) = match v.vtype {
                Vtype::InvertedBinary => {
                    let x = v
                        .inverted
                        .ok_or(LunaModelError::InvalidInversion(v.name.clone().into()))?;
                    (&env.variables[&x].name, true)
                }
                _ => (&v.name, false),
            };
            vnames.push(vname.into());
            inverted.push(inv);
        }
        Ok(Lookup {
            lu,
            vids,
            vnames,
            inverted,
        })
    }

    pub fn update<S>(&mut self, sample: &S) -> LunaModelResult<()>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        for i in 0..self.vids.len() {
            let v = sample.try_index(&self.vnames[i])?;
            self.lu[self.vids[i] as usize] = if self.inverted[i] { 1.0 - v } else { *v };
        }
        Ok(())
    }
}

pub struct SolutionLookup {
    pub lu: Vec<Bias>,
    vids: Vec<u32>,
    inverted: Vec<bool>,
}

impl SolutionLookup {
    pub fn new(env: &Environment, sol: &Solution) -> LunaModelResult<Self> {
        let lu = vec![0.0; env.next_idx as usize];

        let sol_var_names = sol.variable_names();
        let mut vids = Vec::with_capacity(sol_var_names.len());
        let mut inverted = Vec::with_capacity(sol_var_names.len());

        for name in &sol_var_names {
            let vid = env.lookup(name)?;
            let var = &env.variables[&vid];
            let is_inv = matches!(var.vtype, Vtype::InvertedBinary);
            vids.push(vid);
            inverted.push(is_inv);
        }

        Ok(SolutionLookup { lu, vids, inverted })
    }

    pub fn update(&mut self, sample: &SampleView) -> LunaModelResult<()> {
        for (i, v) in sample.iter().enumerate() {
            self.lu[self.vids[i] as usize] = if self.inverted[i] { 1.0 - v } else { v };
        }
        Ok(())
    }
}
