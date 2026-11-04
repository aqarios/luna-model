use itertools::Either::{self, Left, Right};
use lunamodel_types::Sense;

use crate::{Env, Error, Model, Solution, api::api, traits::LmInto};

impl Solution {
    pub fn with_sense(sense: Sense) -> Self {
        Self::from_raw((api().sol_with_sense)(sense.lm_into()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_raw((api().sol_with_capacity)(capacity))
    }

    pub fn from_env(env: &Env) -> crate::Result<Self> {
        env.try_into()
    }

    pub fn from_random(
        n_samples: usize,
        seed: Option<u64>,
        context: Either<Env, Model>,
        sense: Option<Sense>,
        tol: Option<f64>,
    ) -> crate::Result<Self> {
        let mut ctx_a = std::ptr::null();
        let mut ctx_b = std::ptr::null();
        // Borrow: `context` owns the handle and must outlive the call below.
        match &context {
            Left(env) => ctx_a = env.as_ptr(),
            Right(model) => ctx_b = model.as_ptr(),
        };
        out!(sol_from_random(n_samples, ?seed, ctx_a, ctx_b, ?~sense, ?tol))
    }
}

impl TryFrom<&Env> for Solution {
    type Error = Error;

    fn try_from(value: &Env) -> Result<Self, Self::Error> {
        out!(sol_from_env(@value))
    }
}
