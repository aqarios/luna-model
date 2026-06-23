//! Generates the C declarations header (`include/luna_model_decls.h`) via cbindgen.
//!
//! The hand-written `include/luna_model.h` wraps this file and adds the dual-mode
//! (standalone vs. Python-extension) logic; it is not generated.

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let include_dir = PathBuf::from(&crate_dir).join("include");
    let out = include_dir.join("luna_model_decls.h");

    // Regenerate only when the API surface or the cbindgen config changes.
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let config = cbindgen::Config::from_root_or_default(&crate_dir);

    match cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
    {
        Ok(bindings) => {
            std::fs::create_dir_all(&include_dir).expect("failed to create include/ dir");
            bindings.write_to_file(&out);
        }
        Err(err) => {
            // Don't fail the library build on header issues; surface a warning instead.
            println!(
                "cargo:warning=cbindgen failed to generate {}: {err}",
                out.display()
            );
        }
    }
}
