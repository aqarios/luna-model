#[cfg(feature = "gen")]
use std::process::Command;

#[cfg(feature = "gen")]
fn py() {
    // Run the Python script to generate stubs
    let status = Command::new("python")
        .args(&["tools/gen_stubs.py"])
        .status()
        .expect("Failed to execute stub generation script");

    if !status.success() {
        panic!("Stub generation script failed");
    }

    let status = Command::new("python")
        .args(&["tools/gen_init.py"])
        .status()
        .expect("Failed to execute init generation script");

    if !status.success() {
        panic!("Init generation script failed");
    }

    // Add other build logic here if necessary
}
fn main() {
    #[cfg(feature = "gen")]
    py()
}
