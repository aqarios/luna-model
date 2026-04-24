//! Python wrapper for Binary Quadratic Model interoperability helpers.

use lunamodel_core::{Model, ops::LmAddAssign};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Sense, Vtype};
use lunamodel_unwind::*;
use numpy::PyReadonlyArray1;
use pyo3::{PyResult, pyclass, pymethods};
use std::collections::HashMap;

use crate::{PyModel, args::PyModelArg, types::PyVtype};

#[pyclass]
pub struct PyBqmTranslator;

#[unwindable]
#[pymethods]
impl PyBqmTranslator {
    #[allow(
        clippy::too_many_arguments,
        reason = "Python-facing API mirrors external call shape"
    )]
    #[staticmethod]
    fn to_lm(
        vars: Vec<String>,
        vtype: PyVtype,
        offset: Bias,
        linears: PyReadonlyArray1<f64>,
        linear_indices: PyReadonlyArray1<u64>,
        quads: PyReadonlyArray1<f64>,
        quads_rows: PyReadonlyArray1<u64>,
        quads_cols: PyReadonlyArray1<u64>,
        name: Option<String>,
    ) -> PyResult<PyModel> {
        // This entry point mirrors the sparse BQM representation used by
        // external Python ecosystems, so the implementation reconstructs a core
        // model term by term instead of routing through an intermediate format.
        let mut model = Model::new(name, Some(Sense::Min));

        let vars: Vec<_> = vars
            .iter()
            .map(|vname| model.add_var(vname, vtype.into(), None))
            .collect::<LunaModelResult<_>>()?;

        model.objective.add_assign(offset)?;
        let linear_indices: &[u64] = linear_indices.as_slice()?;
        let linears: &[Bias] = linears.as_slice()?;
        for (&i, &bias) in linear_indices.iter().zip(linears) {
            let var = &vars[i as usize];
            model.objective.add_assign((var * bias)?)?;
        }
        let quads: &[Bias] = quads.as_slice()?;
        let quads_rows: &[u64] = quads_rows.as_slice()?;
        let quads_cols: &[u64] = quads_cols.as_slice()?;
        for ((&u, &v), &bias) in quads_rows.iter().zip(quads_cols).zip(quads) {
            let u_var = &vars[u as usize];
            let v_var = &vars[v as usize];
            model.objective.add_assign(((u_var * v_var)? * bias)?)?
        }
        Ok(model.into())
    }

    #[allow(
        clippy::type_complexity,
        reason = "Python-facing API mirrors external result shape"
    )]
    #[staticmethod]
    fn from_lm(
        model: PyModelArg,
    ) -> PyResult<(
        Bias,
        Vec<Bias>,
        Vec<Bias>,
        Vec<i32>,
        Vec<i32>,
        PyVtype,
        Vec<String>,
    )> {
        // The Python BQM representation assumes one shared discrete variable
        // type, no constraints, and a quadratic objective. We validate those
        // invariants explicitly before unpacking coefficients.
        let model: &Model = &model.m.read_arc();
        if model.objective.has_higher_order() {
            return Err(LunaModelError::ModelNotQuadratic)?;
        }
        if !model.constraints.is_empty() {
            return Err(LunaModelError::ModelNotUnconstrained)?;
        }
        if !model.sense.is_min() {
            return Err(LunaModelError::ModelSenseNotMinimize)?;
        }
        let vtypes: Vec<Vtype> = model.vtypes().collect();
        if vtypes.len() != 1 {
            return Err(LunaModelError::Vtype(
                "variables have different vtypes".into(),
            ))?;
        }
        let vtype = vtypes[0];
        if !(vtype == Vtype::Binary || vtype == Vtype::Spin) {
            return Err(LunaModelError::Vtype(
                "vtype is not Vtype.BINARY or Vtype.SPIN".into(),
            ))?;
        }

        let varnames: Vec<_> = model
            .vars()
            .map(|v| v.name())
            .collect::<LunaModelResult<_>>()?;
        let nvars = varnames.len();
        let varidx_lookup: HashMap<String, usize> = varnames
            .iter()
            .enumerate()
            .map(|(idx, v)| (v.clone(), idx))
            .collect();

        let mut offset = Bias::default();
        let mut lin = Vec::new();
        lin.resize(nvars, Bias::default());
        let mut quad = Vec::new();
        let mut quad_rows = Vec::new();
        let mut quad_cols = Vec::new();

        for (vars, bias) in model.objective.items() {
            match &vars[..] {
                [] => offset = bias,
                [u] => lin[varidx_lookup[&u.name()?]] = bias,
                [u, v] => {
                    quad_rows.push(varidx_lookup[&u.name()?] as i32);
                    quad_cols.push(varidx_lookup[&v.name()?] as i32);
                    quad.push(bias);
                }
                _ => return Err(LunaModelError::ModelNotQuadratic)?,
            }
        }

        Ok((
            offset,
            lin,
            quad,
            quad_rows,
            quad_cols,
            vtype.into(),
            varnames,
        ))
    }
}
