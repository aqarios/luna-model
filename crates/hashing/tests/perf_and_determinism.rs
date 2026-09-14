//! Ad-hoc verification (not part of the permanent suite) for the rewritten
//! hashing scheme: confirms hashing a large, sparse, high-variable-ID model
//! stays fast/low-memory, and that the result is deterministic regardless of
//! constraint insertion order.

use std::time::Instant;

use lunamodel_core::prelude::Linear;
use lunamodel_core::{ArcEnv, Constraint, Expression, Model};
use lunamodel_hashing::hash_model;
use lunamodel_types::{Comparator, Vtype};

fn build_model(num_vars: usize, shuffle_constraints: bool) -> Model {
    let mut env = ArcEnv::default();
    let mut idx = Vec::with_capacity(num_vars);
    for i in 0..num_vars {
        let v = env
            .insert(&format!("x{i}"), Vtype::Binary, None)
            .expect("insert var");
        idx.push(v.id());
    }

    let mut model = Model::new(Some("perf-test".into()), None);
    model.environment = env.clone();

    // Objective touches only one variable despite the huge variable-id space.
    let mut objective = Expression::empty(env.clone());
    objective.linear = Linear::for_var(idx[0], 1.0);
    model.objective = objective;

    // One constraint per adjacent variable pair (i, i+1) - each constraint
    // references only 2 of the `num_vars` global variable ids, mirroring a
    // UFLP-style assignment constraint structure.
    let mut order: Vec<usize> = (0..num_vars - 1).collect();
    if shuffle_constraints {
        order.reverse();
    }
    for i in order {
        let mut lhs = Expression::empty(env.clone());
        lhs.linear = Linear::for_vars((idx[i], 1.0), (idx[i + 1], 1.0));
        // Explicit, insertion-order-independent name: auto-naming derives
        // "cN" from the collection's length at insert time, which would
        // make insertion order part of the constraint's actual identity -
        // not what this test is trying to isolate.
        let c = Constraint::new(lhs, 1.0, Comparator::Le, Some(format!("pair_{i}")))
            .expect("constraint");
        model
            .constraints
            .add_constraint(c, None)
            .expect("add_constraint");
    }

    model
}

#[test]
fn hashing_is_fast_and_low_memory_for_sparse_high_id_model() {
    // Comparable scale to the platform-be repro (13203 vars / 13284 constraints).
    let model = build_model(13_000, false);

    let start = Instant::now();
    let _ = hash_model(&model);
    let elapsed = start.elapsed();

    // The old O(variables x constraints) implementation would allocate on
    // the order of gigabytes and take a very long time here; this should
    // complete in well under a second.
    assert!(
        elapsed.as_secs() < 2,
        "hash_model took {elapsed:?}, expected it to stay fast for a sparse model"
    );
}

#[test]
fn hashing_is_deterministic_regardless_of_constraint_insertion_order() {
    let model_a = build_model(200, false);
    let model_b = build_model(200, true);

    assert_eq!(
        hash_model(&model_a),
        hash_model(&model_b),
        "hash must not depend on constraint insertion order"
    );
}

#[test]
fn hashing_is_deterministic_across_repeated_calls() {
    let model = build_model(200, false);
    assert_eq!(hash_model(&model), hash_model(&model));
}

#[test]
fn print_timing_for_repro_scale_model() {
    let model = build_model(13_203, false);
    let start = Instant::now();
    let h = hash_model(&model);
    let elapsed = start.elapsed();
    println!("hash_model on 13203-var/13202-constraint model: {elapsed:?} (hash={h})");
}
