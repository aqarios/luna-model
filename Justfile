# Build tasks for luna-model. Run `just` or `just --list` to see recipes.

c_dir_out          := "dist/c"
c_dir_out_lib      := c_dir_out / "lib"
c_dir_out_include  := c_dir_out / "include"
c_dir_cargo_target := "target"
c_dir_capi_include := "crates/capi-host/include"
c_dir_test         := "test/c"
c_dir_test_build   := c_dir_test / "build"

# Platform-specific cdylib name + soname/install-name linker flag.
c_lib_cargo_filename := if os() == "windows" { "lunamodel_capi_host.dll" } else if os() == "macos" { "liblunamodel_capi_host.dylib" } else { "liblunamodel_capi_host.so" }
c_lib_soname_flag    := if os() == "windows" { "" } else if os() == "macos" { "-Clink-arg=-Wl,-install_name,@rpath/liblunamodel.dylib" } else { "-Clink-arg=-Wl,-soname,liblunamodel.so" }
c_lib_out            := c_dir_out_lib / replace(c_lib_cargo_filename, "_capi_host", "")

proto_dir_src := "proto"
proto_dir_pkg := "py-lunamodel/src/luna_model/proto"

wheel_include := "py-lunamodel/src/luna_model/include"
cpp_src       := "cxx-lunamodel/src"

cpp_dir           := "cxx-lunamodel"
cpp_samples_build := cpp_dir / "samples/build"
cpp_test_build    := cpp_dir / "test/build"
cpp_plugin_test   := cpp_dir / "tests/plugin_ext"
rs_plugin_test    := "pyo3-lunamodel/tests/plugin_ext"

# Absolute paths cmake needs (it is invoked from various build dirs).
lib_dir_abs     := justfile_directory() / c_dir_out_lib
include_dir_abs := justfile_directory() / c_dir_out_include

# List recipes when invoked with no arguments.
_default:
    @just --list

# Check that the tools required to build + test luna-model are installed.
doctor:
    #!/usr/bin/env bash
    set -uo pipefail
    missing=0
    report() {  # label cmd hint required(1/0)
        if command -v "$2" >/dev/null 2>&1; then
            printf '  \033[32m✓\033[0m %-13s %s\n' "$1" "$("$2" --version 2>&1 | head -1)"
        elif [ "$4" = 1 ]; then
            printf '  \033[31m✗\033[0m %-13s missing — %s\n' "$1" "$3"
            missing=$((missing + 1))
        else
            printf '  \033[33m–\033[0m %-13s not found (optional) — %s\n' "$1" "$3"
        fi
    }
    echo "Required:"
    report rust         cargo        "install via https://rustup.rs" 1
    report cmake        cmake        "brew install cmake" 1
    report uv           uv           "brew install uv" 1
    report protoc       protoc       "brew install protobuf" 1
    report c++          c++          "install a C++20 compiler (Xcode CLT / gcc)" 1
    echo "Optional:"
    report clang-format clang-format "brew install clang-format (for 'just cformat')" 0
    report git          git          "install git" 0
    echo
    if [ "$missing" -gt 0 ]; then
        printf '\033[31m%d required tool(s) missing.\033[0m\n' "$missing"
        exit 1
    fi
    printf '\033[32mAll required tools present.\033[0m\n'

# --- C ABI library + headers ------------------------------------------------

# Build the release cdylib with the platform soname/install-name.
build-clib-release:
    cargo rustc -p lunamodel-capi-host --crate-type cdylib --release -- {{c_lib_soname_flag}}

# Build the dev-profile cdylib with the platform soname/install-name.
build-clib-dev:
    cargo rustc -p lunamodel-capi-host --crate-type cdylib --profile dev -- {{c_lib_soname_flag}}

# Regenerate the C headers (cbindgen via build.rs) into dist/c/include.
cheader:
    cargo build -p lunamodel-capi-host
    rm -rf {{c_dir_out_include}}
    mkdir -p {{c_dir_out_include}}
    cp -R {{c_dir_capi_include}}/. {{c_dir_out_include}}/

# Build + stage the release C library into dist/c/lib.
clib: build-clib-release
    mkdir -p {{c_dir_out_lib}}
    cp {{c_dir_cargo_target}}/release/{{c_lib_cargo_filename}} {{c_lib_out}}

# Build + stage the dev C library into dist/c/lib.
clib-dev: build-clib-dev
    mkdir -p {{c_dir_out_lib}}
    cp {{c_dir_cargo_target}}/debug/{{c_lib_cargo_filename}} {{c_lib_out}}

# Full C distribution: headers + release library.
c: cheader clib

# Fail if the committed C headers are stale vs the current ABI.
cdrift:
    cargo build -p lunamodel-capi-host
    git diff --exit-code -- {{c_dir_capi_include}}

# --- Shipped-in-wheel artifacts ---------------------------------------------

# Canonical protobuf schemas -> the copy bundled + shipped in the luna_model wheel (discoverable via
# luna_model.capi.get_proto()). Mirrors how the C headers are shipped.

# Sync the canonical proto/ schemas into the wheel's bundled copy.
proto:
    rm -rf {{proto_dir_pkg}}
    mkdir -p {{proto_dir_pkg}}
    cp -R {{proto_dir_src}}/. {{proto_dir_pkg}}/

# Fail if the wheel's bundled .proto copy is stale vs proto/.
protodrift: proto
    git diff --exit-code -- {{proto_dir_pkg}}

# Public C + C++ headers -> the copy bundled + shipped in the luna_model wheel (discoverable via
# luna_model.capi.get_include()). Mirrors how the .proto schemas are shipped (see `proto`): the
# C-ABI headers come from the cbindgen output, the header-only C++ model/plugin SDK from cxx source.

# Sync the C + C++ SDK headers into the wheel's bundled include/ copy.
wheelheaders: cheader
    rm -rf {{wheel_include}}
    mkdir -p {{wheel_include}}
    cp -R {{c_dir_capi_include}}/. {{wheel_include}}/
    cp -R {{cpp_src}}/. {{wheel_include}}/

# Fail if the wheel's bundled headers are stale vs their sources.
wheelheadersdrift: wheelheaders
    git diff --exit-code -- {{wheel_include}}

# Remove built C artifacts + the capi-host build cache.
cclean:
    rm -rf {{c_dir_out}} {{c_dir_test_build}}
    cargo clean -p lunamodel-capi-host

# --- Tests + formatting ------------------------------------------------------

# Build + run the C test suite (CMake + ctest).
ctest: cheader clib-dev
    cmake -S {{c_dir_test}} -B {{c_dir_test_build}} -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DLUNA_MODEL_LIB_DIR={{lib_dir_abs}} -DLUNA_MODEL_INCLUDE_PATH={{include_dir_abs}}
    cmake --build {{c_dir_test_build}}
    ctest -V --test-dir {{c_dir_test_build}}

# Check C/C++ formatting.
cformat:
    bash tools/run_clang_format.sh

# Apply C/C++ formatting.
fix_cformat:
    bash tools/run_clang_format.sh apply

# Build the C++ samples.
cpp-samples: c
    cmake -S {{cpp_dir}}/samples -B {{cpp_samples_build}} -DLUNA_MODEL_LIB_DIR={{lib_dir_abs}} -DLUNA_MODEL_INCLUDE_PATH={{include_dir_abs}}
    cmake --build {{cpp_samples_build}}

# Build + run the C++ test suite (CMake + ctest).
cpp-test: c
    cmake -S {{cpp_dir}}/test -B {{cpp_test_build}} -DLUNA_MODEL_LIB_DIR={{lib_dir_abs}} -DLUNA_MODEL_INCLUDE_PATH={{include_dir_abs}}
    cmake --build {{cpp_test_build}}
    ctest --test-dir {{cpp_test_build}} -V

# End-to-end test of a C++-authored plugin (nanobind + scikit-build-core) driven from Python: builds
# the luna-model wheel + the C++ extension via uv, then runs pytest. `wheelheaders` first so the
# freshly built wheel bundles up-to-date C/C++ headers (get_include()).

# Build + run the C++ plugin extension end-to-end test (uv + nanobind + pytest).
cpp-plugin-test: wheelheaders
    # Force a rebuild of both packages: plain `uv sync` does not re-detect edits, and the SDK
    # headers just synced into the luna-model wheel (get_include()) only reach the C++ build if
    # luna-model is reinstalled too — otherwise the test silently builds against stale headers.
    cd {{cpp_plugin_test}} && uv sync --group dev --reinstall-package luna-model --reinstall-package luna-cxx-plugin-ext && uv run pytest pytests/ -v

# End-to-end test of a Rust-authored plugin (pyo3 + maturin) driven from Python: a plugin transform
# consumes both another plugin's analysis and a built-in luna-model analysis. Reinstalls luna-model
# so the built-in registrations (+ shared proto tags) are current.
rs-plugin-test:
    cd {{rs_plugin_test}} && uv sync --group dev --reinstall-package luna-model --reinstall-package luna-plugin-ext && uv run pytest pytests/ -v
