#[cfg(feature = "py")]
use numpy::PyReadonlyArray2;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{Model, VarRef, Vtype};

#[cfg_attr(feature = "py", pyclass)]
pub struct MatrixTranslator {}

#[pymethods]
impl MatrixTranslator {
    #[staticmethod]
    #[pyo3(signature=(qubo, name=None))]
    fn to_model(qubo: PyReadonlyArray2<f64>, name: Option<String>) -> Model {
        let matrix = make_native_qubo_matrix(qubo);
        let mut model = Model::new(name);
        fill_model_from_qubo(&mut model, matrix);
        model
    }
}

fn make_native_qubo_matrix(qubo: PyReadonlyArray2<f64>) -> Vec<Vec<f64>> {
    // more efficient way possible? I.e., no iter required?
    qubo.as_array()
        .outer_iter()
        .map(|row| row.to_vec())
        .collect()
}

fn fill_model_from_qubo(model: &mut Model, qubo: Vec<Vec<f64>>) {
    // We have qubo.len() number of variables.
    let variables: Vec<VarRef> = (0..qubo.len())
        .into_iter()
        .map(|v| {
            model
                .environment
                .add_var(&format!("x_{}", v), Some(Vtype::Binary), None)
                .unwrap()
        })
        .collect();

    for (i, row) in qubo.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let a = &variables[i] * value;
            let b = (&a * (&variables[j], &model.environment)).unwrap();
            model.objective += &b;
        }
    }
}
