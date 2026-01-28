use hashbrown::HashMap;
use indexmap::IndexMap;
use itertools::Either;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Sense, Vtype};
use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

use crate::{ArcEnv, Model, Timer, bounds::Bounds, solution::Column};

use super::Solution;

fn get_bounds<T: num_traits::FromPrimitive + num_traits::Bounded>(
    bounds: &Bounds,
) -> LunaModelResult<(T, T)> {
    let ub = match bounds.upper {
        Bound::Bounded(b) => {
            T::from_f64(b).ok_or(LunaModelError::RandomSampling("unexpected bounds.".into()))?
        }
        Bound::Unbounded => T::max_value(),
    };
    let lb = match bounds.lower {
        Bound::Bounded(b) => {
            T::from_f64(b).ok_or(LunaModelError::RandomSampling("unexpected bounds.".into()))?
        }
        Bound::Unbounded => T::min_value(),
    };
    Ok((lb, ub))
}

impl Solution {
    pub fn from_random(
        n_samples: usize,
        seed: Option<u64>,
        context: Either<ArcEnv, Model>,
        sense: Option<Sense>,
    ) -> LunaModelResult<Self> {
        let timer = Timer::start();

        let env = match &context {
            Either::Right(model) => model.environment.clone(),
            Either::Left(env) => env.clone(),
        };
        let mut rng = match seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_os_rng(),
        };

        let mut samples: IndexMap<String, Column> = IndexMap::with_capacity(env.len());

        let binary_distr = Uniform::new_inclusive(0, 1)?;

        for v in env.vars().iter() {
            let vname = v.name()?;
            match v.vtype()? {
                Vtype::Binary => {
                    let data: Vec<u8> =
                        binary_distr.sample_iter(&mut rng).take(n_samples).collect();
                    samples.insert(
                        vname,
                        Column::binary(data.iter().map(|&v| v as f64).collect()),
                    );
                }
                Vtype::Spin => {
                    let data: Vec<i8> = binary_distr
                        .sample_iter(&mut rng)
                        .map(|x| (2 * x as i8) - 1)
                        .take(n_samples)
                        .collect();
                    samples.insert(
                        vname,
                        Column::spin(data.iter().map(|&v| v as f64).collect()),
                    );
                }
                Vtype::Integer => {
                    let (lb, ub) = get_bounds::<i64>(&v.bounds()?)?;
                    let integer_distr = Uniform::new_inclusive(lb, ub)?;
                    let data: Vec<i64> = integer_distr
                        .sample_iter(&mut rng)
                        .take(n_samples)
                        .collect();
                    samples.insert(
                        vname,
                        Column::integer(data.iter().map(|&v| v as f64).collect()),
                    );
                }
                Vtype::Real => {
                    let (lb, ub) = get_bounds::<f64>(&v.bounds()?)?;
                    let real_distr = Uniform::new_inclusive(lb, ub)?;
                    let data: Vec<f64> = real_distr.sample_iter(&mut rng).take(n_samples).collect();
                    samples.insert(
                        vname,
                        Column::real(data.iter().map(|&v| v as f64).collect()),
                    );
                }
                // Ingnore inverted binaries
                Vtype::InvertedBinary => (),
            }
        }
        let counts = vec![1; n_samples];

        let timing = Some(timer.stop());

        let mut sol = Solution {
            samples,
            counts,
            raw_energies: None,
            obj_values: None,
            constraints: HashMap::new(),
            variable_bounds: HashMap::new(),
            feasible: None,
            timing,
            sense: match &context {
                Either::Right(m) => m.sense.clone(),
                Either::Left(_) => sense.unwrap_or(Sense::Min),
            },
        };

        sol.aggregate()?;

        match context {
            Either::Right(m) => Ok(m.evaluate_solution(&sol)?),
            Either::Left(_) => Ok(sol),
        }
    }
}
