use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=proto");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Pure-Rust: parse .proto -> FileDescriptorSet (no system protoc), then prost generates Rust.
    let fds = protox::compile(["proto/plugin.proto"], ["proto"]).expect("compile .proto");
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_fds(fds)
        .expect("prost codegen");
}
