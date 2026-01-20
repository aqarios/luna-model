use hashbrown::HashMap;
use indexmap::IndexMap;
use lunamodel_core::solution::{Assignment, Column};
use lunamodel_core::{ArcEnv, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Sense, Vtype};
use numpy::ndarray::Axis;
use numpy::{PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::{PyResult, pymethods};

use crate::timer::PyTiming;
use crate::utils::retrieve_environment;
use crate::{PyEnvironment, PyModel};

use super::PySolution;
use super::utils::VarKey;

enum BitOrder {
    LTR,
    RTL,
}

#[pymethods]
impl PySolution {
    #[new]
    fn pynew(
        samples: Vec<IndexMap<VarKey, f64>>,
        counts: Option<Vec<usize>>,
        raw_energies: Option<Vec<f64>>,
        obj_values: Option<Vec<f64>>,
        feasible: Option<Vec<bool>>,
        constraints: Option<Vec<IndexMap<String, bool>>>,
        variables_bounds: Option<IndexMap<String, Vec<bool>>>,
        timing: Option<PyTiming>,
        sense: Option<Sense>,
        env: Option<PyEnvironment>,
        vtypes: Option<Vec<Vtype>>,
    ) -> PyResult<Self> {
        let mut sol = Solution::with_sense(sense.unwrap_or_default());
        sol.timing = timing.map(|t| t.into());

        if samples.is_empty() {
            return Ok(sol.into());
        }

        let sample_len = samples[0].len();
        if let Some(vs) = &vtypes
            && sample_len != vs.len()
        {
            return Err(PyValueError::new_err(
                "The number of variables does not match the number of variable types.",
            ));
        }
        match counts {
            Some(counts) => {
                if counts.len() != samples.len() {
                    return Err(PyValueError::new_err(
                        "counts length does not match number of samples.",
                    ));
                } else {
                    sol.counts = counts;
                }
            }
            None => sol.counts = vec![1; samples.len()],
        }
        match raw_energies {
            Some(es) => {
                if es.len() != samples.len() {
                    return Err(PyValueError::new_err(
                        "energies length does not match number of samples.",
                    ));
                } else {
                    sol.raw_energies = Some(es);
                }
            }
            None => sol.raw_energies = None,
        }
        match obj_values {
            Some(es) => {
                if es.len() != samples.len() {
                    return Err(PyValueError::new_err(
                        "obj_values length does not match number of samples.",
                    ));
                } else {
                    sol.obj_values = Some(es);
                }
            }
            None => sol.obj_values = None,
        }

        let sample_vars: Vec<_> = samples[0].keys().collect();
        let mut s: IndexMap<String, Vec<f64>> = sample_vars
            .iter()
            .map(|k| match k.name() {
                Ok(name) => Ok((name, Vec::new())),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<_>>()?;

        for sample in samples.iter() {
            if sample.len() != sample_len {
                return Err(PyValueError::new_err("samples have different lengths."));
            }
            for (var, value) in sample.iter() {
                let varname = var.name()?;
                s.get_mut(&varname)
                    .ok_or_else(|| LunaModelError::VariableNotExisting(varname.into()))?
                    .push(*value);
            }
        }

        let variable_types: HashMap<String, Vtype> = match vtypes {
            Some(vs) => sample_vars
                .into_iter()
                .zip(vs)
                .map(|(v, t)| match v.name() {
                    Ok(n) => Ok((n, t)),
                    Err(e) => Err(e),
                })
                .collect::<LunaModelResult<_>>()?,
            None => {
                let pyenv: PyEnvironment = env.try_into()?;
                let mut vs = HashMap::new();
                for v in sample_vars.into_iter() {
                    let vname = v.name()?;
                    let vt = pyenv.env.vtype_of(&vname)?;
                    vs.insert(vname, vt);
                }
                vs
            }
        };
        for (varname, values) in s {
            match variable_types[&varname] {
                Vtype::Binary => sol.add_binary(varname, values),
                Vtype::Spin => sol.add_spin(varname, values),
                Vtype::Integer => sol.add_integer(varname, values),
                Vtype::Real => sol.add_real(varname, values),
                Vtype::InvertedBinary => (),
            }
        }

        sol.feasible = feasible;

        if let Some(constraints) = constraints {
            if constraints.len() != samples.len() {
                return Err(PyValueError::new_err(
                    "constraints length does not match number of samples.",
                ));
            }
            for c in constraints {
                for (cname, val) in c {
                    sol.constraints
                        .entry(cname)
                        .or_insert(Vec::default())
                        .push(val);
                }
            }
        }

        if let Some(varbounds) = variables_bounds {
            let mut vbs: HashMap<String, Vec<bool>> = HashMap::with_capacity(varbounds.len());
            for (i, (key, values)) in varbounds.into_iter().enumerate() {
                if values.len() != samples.len() {
                    return Err(PyValueError::new_err(format!(
                        "variables_bounds at index '{i}' length does not match number of samples."
                    )));
                }
                vbs.insert(key, values);
            }
            sol.variable_bounds = vbs;
        }
        sol.combine_to_single()?;

        Ok(sol.into())
    }

    #[staticmethod]
    fn from_dict(
        data: IndexMap<VarKey, f64>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        counts: Option<usize>,
        sense: Option<Sense>,
        energy: Option<f64>,
    ) -> PyResult<Self> {
        check_env_or_model(&env, &model)?;
        check_sense_or_model(&sense, &model)?;
        let environment = retrieve_environment(env, &model)?.env;

        fn inner(
            data: IndexMap<VarKey, f64>,
            environment: ArcEnv,
            model: Option<PyModel>,
            timing: Option<PyTiming>,
            counts: Option<usize>,
            sense: Option<Sense>,
            energy: Option<f64>,
        ) -> LunaModelResult<Solution> {
            let mut sol = Solution::with_sense(sense.unwrap_or_else(|| {
                model
                    .as_ref()
                    .map(|m| m.m.read_arc().sense)
                    .unwrap_or_default()
            }));
            sol.timing = timing.map(|t| t.into());
            sol.counts.push(counts.unwrap_or_else(|| 1));
            if let Some(e) = energy {
                sol.raw_energies.as_mut().map(|ens| ens.push(e));
            }

            environment.vars().iter().for_each(|v| {
                let vname = v.name().unwrap();
                let vtype = v.vtype().unwrap();
                match vtype {
                    Vtype::Binary => sol.add_empty_binary(vname),
                    Vtype::Spin => sol.add_empty_spin(vname),
                    Vtype::Integer => sol.add_empty_integer(vname),
                    Vtype::Real => sol.add_empty_real(vname),
                    // TODO: this should never happen. If it does it will return an error later.
                    Vtype::InvertedBinary => (),
                }
            });

            for (var, value) in data.iter() {
                let varname = var.name()?;
                let solcol = sol
                    .samples
                    .get_mut(&varname)
                    .ok_or_else(|| LunaModelError::SampleUnexpectedVariable(varname.into()))?;
                solcol.try_push(*value)?;
            }

            sol.combine_to_single()?;

            if let Some(m) = model {
                sol = m.m.read_arc().evaluate_solution(&sol)?;
            }

            Ok(sol)
        }

        let sol =
            inner(data, environment, model, timing, counts, sense, energy).map_err(
                |e| match e {
                    LunaModelError::VariableNotExisting(e) => {
                        LunaModelError::SampleUnexpectedVariable(e)
                    }
                    e => e,
                },
            )?;

        Ok(sol.into())
    }

    #[staticmethod]
    fn from_dicts(
        data: Vec<IndexMap<VarKey, f64>>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        counts: Option<Vec<usize>>,
        sense: Option<Sense>,
        energies: Option<Vec<f64>>,
    ) -> PyResult<Self> {
        check_env_or_model(&env, &model)?;
        check_sense_or_model(&sense, &model)?;
        let environment = retrieve_environment(env, &model)?.env;

        fn inner(
            data: Vec<IndexMap<VarKey, f64>>,
            environment: ArcEnv,
            model: Option<PyModel>,
            timing: Option<PyTiming>,
            counts: Option<Vec<usize>>,
            sense: Option<Sense>,
            energies: Option<Vec<f64>>,
        ) -> LunaModelResult<Solution> {
            let mut sol = Solution::with_sense(sense.unwrap_or_else(|| {
                model
                    .as_ref()
                    .map(|m| m.m.read_arc().sense)
                    .unwrap_or_default()
            }));
            if data.is_empty() {
                return Ok(sol.into());
            }

            sol.timing = timing.map(|t| t.into());
            sol.counts
                .append(&mut counts.unwrap_or_else(|| vec![1; data.len()]));
            if let Some(es) = energies {
                sol.raw_energies = Some(es)
            }

            environment.vars().iter().for_each(|v| {
                let vname = v.name().unwrap();
                let vtype = v.vtype().unwrap();
                match vtype {
                    Vtype::Binary => sol.add_empty_binary(vname),
                    Vtype::Spin => sol.add_empty_spin(vname),
                    Vtype::Integer => sol.add_empty_integer(vname),
                    Vtype::Real => sol.add_empty_real(vname),
                    // TODO: this should never happen. If it does it will return an error later.
                    Vtype::InvertedBinary => (),
                }
            });

            let sample_len = data[0].len();
            for sample in data.iter() {
                if sample.len() != sample_len {
                    return Err(LunaModelError::SampleIncorrectLength(
                        format!("expected {}, is {}", sample_len, sample.len()).into(),
                    ));
                }
                for (var, value) in sample.iter() {
                    let varname = var.name()?;
                    sol.samples
                        .get_mut(&varname)
                        .ok_or_else(|| LunaModelError::SampleUnexpectedVariable(varname.into()))?
                        .try_push(*value)?;
                }
            }

            sol.combine_to_single()?;

            if let Some(m) = model {
                sol = m.m.read_arc().evaluate_solution(&sol)?;
            }

            Ok(sol)
        }

        let sol = inner(data, environment, model, timing, counts, sense, energies).map_err(
            |e| match e {
                LunaModelError::VariableNotExisting(e) => {
                    LunaModelError::SampleUnexpectedVariable(e)
                }
                e => e,
            },
        )?;

        Ok(sol.into())
    }

    #[staticmethod]
    fn from_arrays(
        data: PyReadonlyArray2<f64>,
        variables: Option<Vec<VarKey>>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        counts: Option<Vec<usize>>,
        sense: Option<Sense>,
        energies: Option<Vec<f64>>,
    ) -> PyResult<Self> {
        check_env_or_model(&env, &model)?;
        check_sense_or_model(&sense, &model)?;
        let environment = retrieve_environment(env, &model)?.env;

        fn inner(
            data: PyReadonlyArray2<f64>,
            variables: Option<Vec<VarKey>>,
            environment: ArcEnv,
            model: Option<PyModel>,
            timing: Option<PyTiming>,
            counts: Option<Vec<usize>>,
            sense: Option<Sense>,
            energies: Option<Vec<f64>>,
        ) -> LunaModelResult<Solution> {
            let mut sol = Solution::with_sense(sense.unwrap_or_else(|| {
                model
                    .as_ref()
                    .map(|m| m.m.read_arc().sense)
                    .unwrap_or_default()
            }));
            sol.timing = timing.map(|t| t.into());
            sol.counts
                .append(&mut counts.unwrap_or_else(|| vec![1; data.shape()[0]]));
            if let Some(es) = energies {
                sol.raw_energies = Some(es)
            }

            let variables: Vec<String> = match variables {
                Some(vars) => vars
                    .iter()
                    .map(|v| v.name())
                    .collect::<LunaModelResult<_>>()?,
                None => environment
                    .vars()
                    .iter()
                    .map(|v| v.name())
                    .collect::<LunaModelResult<_>>()?,
            };

            environment.vars().iter().for_each(|v| {
                let vname = v.name().unwrap();
                let vtype = v.vtype().unwrap();
                match vtype {
                    Vtype::Binary => sol.add_empty_binary(vname),
                    Vtype::Spin => sol.add_empty_spin(vname),
                    Vtype::Integer => sol.add_empty_integer(vname),
                    Vtype::Real => sol.add_empty_real(vname),
                    // TODO: this should never happen. If it does it will return an error later.
                    Vtype::InvertedBinary => (),
                }
            });

            for (col, varname) in data.as_array().axis_iter(Axis(1)).zip(variables) {
                let solcol = sol
                    .samples
                    .get_mut(&varname)
                    .ok_or_else(|| LunaModelError::SampleUnexpectedVariable(varname.into()))?;
                for &v in col.iter() {
                    solcol.try_push(v)?;
                }
            }

            sol.combine_to_single()?;

            Ok(sol)
        }

        let sol = inner(
            data,
            variables,
            environment,
            model,
            timing,
            counts,
            sense,
            energies,
        )
        .map_err(|e| match e {
            LunaModelError::VariableNotExisting(e) => LunaModelError::SampleUnexpectedVariable(e),
            e => e,
        })?;

        Ok(sol.into())
    }

    #[staticmethod]
    fn from_counts(
        data: IndexMap<String, usize>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
        timing: Option<PyTiming>,
        sense: Option<Sense>,
        bit_order: String,
        energies: Option<Vec<f64>>,
        var_order: Option<Vec<String>>,
    ) -> PyResult<PySolution> {
        check_env_or_model(&env, &model)?;
        check_sense_or_model(&sense, &model)?;
        let environment = retrieve_environment(env, &model)?.env;

        fn inner(
            data: IndexMap<String, usize>,
            environment: ArcEnv,
            model: Option<PyModel>,
            timing: Option<PyTiming>,
            sense: Option<Sense>,
            bit_order: String,
            energies: Option<Vec<f64>>,
            var_order: Option<Vec<String>>,
        ) -> LunaModelResult<Solution> {
            let mut sol = Solution::with_sense(sense.unwrap_or_else(|| {
                model
                    .as_ref()
                    .map(|m| m.m.read_arc().sense)
                    .unwrap_or_default()
            }));
            sol.timing = timing.map(|t| t.into());
            if let Some(es) = energies {
                sol.raw_energies = Some(es)
            }

            let vars = match var_order {
                Some(vs) => vs,
                None => environment
                    .vars()
                    .iter()
                    .map(|v| v.name().unwrap())
                    .collect(),
            };
            let nvars = vars.len();
            for v in environment.sort(vars.clone()) {
                match environment.vtype_of(&v)? {
                    Vtype::Binary => sol.add_empty_binary(v),
                    Vtype::Spin => sol.add_empty_spin(v),
                    Vtype::Integer | Vtype::Real | Vtype::InvertedBinary => {
                        return Err(LunaModelError::Translation(
                            "solution contains reference to non-binary or non-spin variables."
                                .into(),
                        ));
                    }
                }
            }

            let order = match bit_order.as_str() {
                "LTR" => BitOrder::LTR,
                "RTL" => BitOrder::RTL,
                _ => {
                    return Err(LunaModelError::Translation(
                        "`bit_order` must be 'RTL' or 'LTR'.".into(),
                    ));
                }
            };

            for (idx, (bitstr, &count)) in data.iter().enumerate() {
                if bitstr.len() != nvars {
                    return Err(LunaModelError::SampleIncorrectLength(
                        format!("sample at index '{idx}' has an unexpected length").into(),
                    ));
                }
                let it = match order {
                    BitOrder::LTR => itertools::Either::Left(bitstr.chars()),
                    BitOrder::RTL => itertools::Either::Right(bitstr.chars().rev()),
                };
                for (c, varname) in it.into_iter().zip(&vars) {
                    match (c, sol.samples.get_mut(varname).unwrap()) {
                        ('0', Column::Binary(vec)) => vec.push(Assignment::Binary(0))?,
                        ('1', Column::Binary(vec)) => vec.push(Assignment::Binary(1))?,
                        ('0', Column::Spin(vec)) => vec.push(Assignment::Spin(1))?,
                        ('1', Column::Spin(vec)) => vec.push(Assignment::Spin(-1))?,
                        _ => {
                            return Err(LunaModelError::Translation(
                                "unexpected char in bitstring.".into(),
                            ));
                        }
                    }
                }
                sol.counts.push(count);
            }

            sol.combine_to_single()?;

            if let Some(m) = model {
                sol = m.m.read_arc().evaluate_solution(&sol)?;
            }
            Ok(sol)
        }

        let sol = inner(
            data,
            environment,
            model,
            timing,
            sense,
            bit_order,
            energies,
            var_order,
        )
        .map_err(|e| match e {
            LunaModelError::VariableNotExisting(e) => LunaModelError::SampleUnexpectedVariable(e),
            e => e,
        })?;

        Ok(sol.into())
    }
}

fn check_env_or_model(env: &Option<PyEnvironment>, model: &Option<PyModel>) -> PyResult<()> {
    if env.is_some() && model.is_some() {
        Err(PyValueError::new_err(
            "either `env` or `model` has to be `None`",
        ))
    } else {
        Ok(())
    }
}

fn check_sense_or_model(sense: &Option<Sense>, model: &Option<PyModel>) -> PyResult<()> {
    if sense.is_some() && model.is_some() {
        Err(PyValueError::new_err(
            "either `sense` or `model` has to be `None`",
        ))
    } else {
        Ok(())
    }
}
