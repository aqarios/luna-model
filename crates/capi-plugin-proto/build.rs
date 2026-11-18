use std::{env, path::PathBuf};

fn main() {
    // Canonical schemas live in the repo-root `proto/` dir (language-neutral, shared with the wheel
    // and other-language plugins), not vendored per-crate.
    let proto_root = "../../proto";
    println!("cargo:rerun-if-changed={proto_root}");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Pure-Rust: parse .proto -> FileDescriptorSet (no system protoc), then prost generates Rust.
    let fds = protox::compile(["luna_model/plugin.proto"], [proto_root]).expect("compile .proto");
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_fds(fds)
        .expect("prost codegen");
}
