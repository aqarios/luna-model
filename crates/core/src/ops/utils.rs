use std::fmt::Debug;

use crate::{
    Expression,
    prelude::{HigherOrder, Linear, Quadratic, VarRef},
    traits::DefaultEditable,
};
use hashbrown::{HashMap, hash_map::Entry};
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
        dbg!(a, b);
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

pub enum VarMulRes {
    Const(Bias),
    Lin((VarIdx, Bias)),
    Quad((VarIdx, VarIdx, Bias)),
    HiOr((Vec<VarIdx>, Bias)),
}

impl Into<Expression> for VarMulRes {
    fn into(self) -> Expression {
        use VarMulRes::*;
        match self {
            Const(cnst) => cnst.into(),
            Lin(lin) => Linear::with(|l| l[lin.0] = lin.1).into(),
            Quad(quad) => Quadratic::with(|q| q[(quad.0, quad.1)] = quad.2).into(),
            HiOr(ho) => HigherOrder::with(|h| h[ho.0.as_slice()] = ho.1).into(),
        }
    }
}

impl From<(Vec<u32>, Bias)> for VarMulRes {
    fn from(value: (Vec<u32>, Bias)) -> Self {
        let (vars, bias) = value;
        match vars.as_slice() {
            &[] => Self::Const(bias),
            &[a] => Self::Lin((a, bias)),
            &[a, b] => Self::Quad((a, b, bias)),
            _ => Self::HiOr((vars, bias)),
        }
    }
}

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
    let mut ocs: HashMap<VarIdx, (usize, Vtype)> = HashMap::new();
    for &v in vars {
        let entry = ocs.entry(v);
        if let Entry::Occupied(entry, ..) = entry {
            let vt = entry.get().1;
            if (vt == Binary || vt == InvertedBinary)
                && let Some(inverted) = inv(v)
                // && ocs.contains_key(&inverted)
                && let Entry::Occupied(_) = ocs.entry(inverted)
            {
                // if the variable is a binary or an inverted binary and it has
                // an inverted and this inverted is also in the ocs than
                // everything is 0 and we have an empty vec.
                return Vec::default();
            }
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
            _ => todo!(),
        }
    }
    reduced
}
