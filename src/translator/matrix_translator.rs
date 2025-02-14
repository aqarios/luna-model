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
        // let make_matrix_start = Instant::now();
        let matrix = make_native_qubo_matrix(qubo);
        // let make_matrix_elapsed = make_matrix_start.elapsed();

        // let new_model_start = Instant::now();
        let mut model = Model::new(name);
        // let new_model_elapsed = new_model_start.elapsed();

        // let fill_model_start = Instant::now();
        fill_model_from_qubo(&mut model, matrix);
        // let fill_model_elapsed = fill_model_start.elapsed();

        // println!(
        //     "Make Matrix: {:?}\nNew Model: {:?}\nFill Model: {:?}",
        //     make_matrix_elapsed, new_model_elapsed, fill_model_elapsed
        // );
        // println!("========");

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
    // let create_variables_start = Instant::now();
    let num_vars = qubo.len();
    let variables: Vec<VarRef> = (0..num_vars)
        .into_iter()
        .map(|v| {
            model
                .environment
                .add_var(&format!("x_{}", v), Some(Vtype::Binary), None)
                .unwrap()
        })
        .collect();

    // Let's try a reallocated approach...
    // We know that there are at most variables * variables in the hashmap.
    // let alloc_start = Instant::now();
    let max_num_vars = num_vars * (num_vars + 1) / 2;
    model.objective.linear.allocate(num_vars);
    model.objective.quadratic.allocate(max_num_vars);
    // let alloc_elapsed = alloc_start.elapsed();
    // println!("Preallocating takes: {:?}", alloc_elapsed);

    // OPTIMIZED LOOP -> QUBO -> Symmetric -> Just look at upper.
    // let fill_start = Instant::now();
    // linear components
    for i in 0..num_vars {
        let var = &variables[i];
        let value = qubo[i][i];
        if value == 0.0 {
            continue;
        }
        model.objective.linear.insert_linear(var, value);
    }

    // let mut acc: Vec<Duration> = Vec::new();
    // quadratic components.
    // (0..num_vars).into_iter().for_each(|i| {
    //     ((i + 1)..num_vars).into_iter().for_each(|j| {
    //         let a = 1 * 1;
    //     })
    // });

    for i in 0..num_vars {
        let a = &variables[i];
        // let row = &qubo[i];
        for j in (i + 1)..num_vars {
            // let access_start = Instant::now();
            let value = &qubo[i][j] + &qubo[j][i];
            // let value = row[j] + row[j];
            // let access_elapsed = access_start.elapsed();
            // acc.push(access_elapsed);
            if value == 0.0 {
                continue;
            }
            let b = &variables[j];
            // model.objective.quadratic.insert_quadratic(a, b, 1.0);
            model.objective.quadratic.insert_quadratic(a, b, value);
        }
    }
    //println!("access: {:?}", acc.iter().sum::<Duration>())
    // let fill_elapsed = fill_start.elapsed();
    // println!("Filling takes: {:?}", fill_elapsed);
}
