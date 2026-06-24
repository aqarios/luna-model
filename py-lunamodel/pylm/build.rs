//! Copies the generated C API headers into the Python package so the wheel ships
//! them (discoverable via `luna_model.capi.get_include()`). The headers themselves
//! are produced by lunamodel-cext's build.rs, which runs first as a dependency.

use std::path::Path;

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let manifest = Path::new(&manifest); // py-lunamodel/pylm
    let dst = manifest.join("../src/luna_model/include"); // bundled into the wheel

    let h = manifest.join("../../crates/cext/include"); // C headers (.h)
    let hpp = manifest.join("../../cxx-lunamodel/src"); // C++ headers (.hpp)

    println!("cargo:rerun-if-changed={}", h.display());
    println!("cargo:rerun-if-changed={}", hpp.display());

    let _ = std::fs::remove_dir_all(&dst);
    if h.exists() {
        copy_dir(&h, &dst);
    }
    if hpp.exists() {
        copy_dir(&hpp, &dst);
    }
}

fn copy_dir(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("create dst");
    for entry in std::fs::read_dir(src).expect("read src") {
        let entry = entry.expect("entry");
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            std::fs::copy(&path, &target).expect("copy file");
        }
    }
}
