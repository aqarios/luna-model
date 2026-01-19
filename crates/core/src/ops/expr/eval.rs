use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Vtype};

use crate::{
    prelude::{Expression, VarRef},
    traits::{TryIndex, Variables},
};

impl Expression {
    pub fn evaluate_sampleset<'s, Sample, S>(&self, sampleset: S) -> LunaModelResult<Vec<Bias>>
    where
        for<'v> Sample: 's + TryIndex<&'v str, Output = Bias, Err = LunaModelError>,
        Sample: Variables,
        S: Iterator<Item = Sample>,
    {
        sampleset.map(|s| self.evaluate_sample(&s)).collect()
    }

    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<Bias>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
        S: Variables,
    {
        check_alignment(
            &self.vars().map(|v| v.name().unwrap()).collect::<Vec<_>>(),
            &sample.vars(),
        )?;
        let mut val = Bias::default();
        for (vs, bias) in self.items() {
            match &vs[..] {
                [] => val += bias,
                [v] => val += adjusted(sample, v)? * bias,
                [u, v] => val += adjusted(sample, u)? * adjusted(sample, v)? * bias,
                vs => {
                    let varval: LunaModelResult<f64> = vs
                        .iter()
                        .try_fold(bias, |b, v| Ok(b * adjusted(sample, v)?));
                    val += varval?;
                }
            }
        }
        Ok(val)
    }
}

fn adjusted<S>(sample: &S, v: &VarRef) -> LunaModelResult<Bias>
where
    for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
{
    if v.vtype()? == Vtype::InvertedBinary {
        Ok(1.0 - sample.try_index(&v.inverted()?.unwrap().name()?)?)
    } else {
        sample
            .try_index(&v.name()?)
            .copied()
            .map_err(|e| LunaModelError::Evaluation(e.to_string().into()))
    }
}

fn check_alignment(expr_vars: &[String], sample_vars: &[String]) -> LunaModelResult<()> {
    if expr_vars.len() != sample_vars.len() {
        return Err(LunaModelError::Evaluation(
            "number of variables does not match".into(),
        ));
    }
    for ev in expr_vars {
        if !sample_vars.contains(ev) {
            return Err(LunaModelError::Evaluation(
                format!("variable '{ev}' is not contained in sample").into(),
            ));
        }
    }
    for sv in sample_vars {
        if !expr_vars.contains(sv) {
            return Err(LunaModelError::Evaluation(
                format!("variable '{sv}' is not contained in expression").into(),
            ));
        }
    }
    Ok(())
}
