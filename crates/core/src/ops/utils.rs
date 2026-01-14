use std::fmt::Debug;
use std::ops::Mul;

use crate::{
    Expression,
    prelude::{HigherOrder, Linear, Quadratic, VarRef},
    traits::DefaultEditable,
};
use hashbrown::HashMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, EnvIdx, VarIdx, Vtype, Vtype::*};

pub(crate) trait EnvIdexable {
    fn env_id(&self) -> EnvIdx;
}

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
    Const(Bias),
    Lin((VarIdx, Bias)),
    Quad((VarIdx, VarIdx, Bias)),
    HiOr((Vec<VarIdx>, Bias)),
}

impl From<(VarIdx, VarIdx, Bias)> for VarMulRes {
    fn from(value: (VarIdx, VarIdx, Bias)) -> Self {
        let (u, v, b) = value;
        Self::Quad((u, v, b))
    }
}

impl Into<Expression> for VarMulRes {
    fn into(self) -> Expression {
        match self {
            Self::Const(b) => b.into(),
            Self::Lin((u, b)) => Linear::with(|l| l[u] = b).into(),
            Self::Quad((u, v, b)) => Quadratic::with(|q| q[(u, v)] = b).into(),
            Self::HiOr((vs, b)) => HigherOrder::with(|h| h[vs.as_slice()] = b).into(),
        }
    }
}

impl From<(Vec<u32>, Bias)> for VarMulRes {
    fn from(value: (Vec<u32>, Bias)) -> Self {
        let (vars, b) = value;
        match vars.as_slice() {
            &[] => Self::Const(b),
            &[u] => Self::Lin((u, b)),
            &[u, v] => Self::Quad((u, v, b)),
            _ => Self::HiOr((vars, b)),
        }
    }
}

impl Mul<Bias> for VarMulRes {
    type Output = VarMulRes;

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

/// Reduce the given variables to the minimal set representing
/// the same logical operation for multilication.
pub fn reduce_vars_mul<F, I>(vars: &[VarIdx], vtype: F, inv: I) -> Vec<VarIdx>
where
    F: Fn(VarIdx) -> Vtype,
    I: Fn(VarIdx) -> Option<VarIdx>,
{
    // TODO: I don't like this very much. Very unclear. Include extra flag for binary *
    // inverted binary occured.
    let mut ocs: HashMap<VarIdx, (usize, Vtype)> = HashMap::new();
    for &v in vars {
        if let Some(inverted) = inv(v)
            && ocs.contains_key(&inverted)
        {
            return Vec::default();
        }
        // if let Some(entry) = entry {
        //     let vt = entry.1;
        //     if (vt == Binary || vt == InvertedBinary)
        //         && let Some(inverted) = inv(v)
        //         && ocs.contains_key(&inverted)
        //     {
        //         // if the variable is a binary or an inverted binary and it has
        //         // an inverted and this inverted is also in the ocs than
        //         // everything is 0 and we have an empty vec.
        //         return Vec::default();
        //     }
        // }
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
    reduced
}
