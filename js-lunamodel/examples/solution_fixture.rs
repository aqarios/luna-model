use std::time::{Duration, UNIX_EPOCH};

use lunamodel_core::{Solution, Timing};
use lunamodel_serializer::prelude::*;

fn main() {
    let fixture = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "full".to_string());
    let solution = match fixture.as_str() {
        "full" => full_solution(),
        "nulls" => Solution::default(),
        "timing" => timing_solution(),
        other => panic!("unknown fixture {other:?}; expected full, nulls, or timing"),
    };

    print_solution_hex(solution);
}

fn full_solution() -> Solution {
    let mut solution = Solution::default();
    solution
        .add_binary("x".to_string(), vec![0.0, 1.0], None)
        .expect("fixture binary column should be valid");
    solution.counts = vec![2, 3];
    solution.raw_energies = Some(vec![1.5, 2.5]);
    solution.obj_values = Some(vec![10.0, 20.0]);
    solution
        .constraints
        .insert("c0".to_string(), vec![true, false]);
    solution
        .variable_bounds
        .insert("x".to_string(), vec![true, true]);

    solution
}

fn timing_solution() -> Solution {
    Solution {
        timing: Some(Timing::new(
            UNIX_EPOCH + Duration::from_secs(1),
            UNIX_EPOCH + Duration::from_secs(3),
            Some(0.25),
        )),
        ..Default::default()
    }
}

fn print_solution_hex(solution: Solution) {
    let bytes = solution
        .encode(Some(false), None)
        .expect("solution fixture should encode");

    for byte in bytes {
        print!("{byte:02x}");
    }
    println!();
}
