ifneq ($(OS),Windows_NT)
    OS := $(shell uname -s)
endif

C_DIR_OUT = dist/c
C_DIR_OUT_LIB = $(C_DIR_OUT)/lib
C_DIR_OUT_INCLUDE = $(C_DIR_OUT)/include
C_DIR_CARGO_TARGET = target
C_DIR_CEXT_INCLUDE = crates/cext/include
C_DIR_TEST = test/c
C_DIR_TEST_BUILD = $(C_DIR_TEST)/build

ifeq ($(OS),Windows_NT)
    C_LIB_CARGO_FILENAME = lunamodel_cext.dll
    C_LIB_SONAME_FLAG =
else ifeq ($(OS),Darwin)
    C_LIB_CARGO_FILENAME = liblunamodel_cext.dylib
    C_LIB_SONAME_FLAG = -Clink-arg=-Wl,-install_name,@rpath/liblunamodel.dylib
else
    C_LIB_CARGO_FILENAME = liblunamodel_cext.so
    C_LIB_SONAME_FLAG = -Clink-arg=-Wl,-soname,liblunamodel.so
endif
C_LIB_OUT = $(C_DIR_OUT_LIB)/$(subst _cext,,$(C_LIB_CARGO_FILENAME))

.PHONY: c cheader clib clib-dev build-clib-release build-clib-dev cdrift cclean

build-clib-release:
	cargo rustc -p lunamodel-cext --crate-type cdylib --release -- $(C_LIB_SONAME_FLAG)

build-clib-dev:
	cargo rustc -p lunamodel-cext --crate-type cdylib --profile dev -- $(C_LIB_SONAME_FLAG)

cheader:
	cargo build -p lunamodel-cext
	rm -rf $(C_DIR_OUT_INCLUDE)
	mkdir -p $(C_DIR_OUT_INCLUDE)
	cp -R $(C_DIR_CEXT_INCLUDE)/. $(C_DIR_OUT_INCLUDE)/

clib: build-clib-release
	mkdir -p $(C_DIR_OUT_LIB)
	cp $(C_DIR_CARGO_TARGET)/release/$(C_LIB_CARGO_FILENAME) $(C_LIB_OUT)

clib-dev: build-clib-dev
	mkdir -p $(C_DIR_OUT_LIB)
	cp $(C_DIR_CARGO_TARGET)/debug/$(C_LIB_CARGO_FILENAME) $(C_LIB_OUT)

c: cheader clib

cdrift:
	cargo build -p lunamodel-cext
	git diff --exit-code -- $(C_DIR_CEXT_INCLUDE)

cclean:
	rm -rf $(C_DIR_OUT) $(C_DIR_TEST_BUILD)
	cargo clean -p lunamodel-cext

.PHONY: ctest
ctest: cheader clib-dev
	cmake -S $(C_DIR_TEST) -B $(C_DIR_TEST_BUILD) -DLUNA_MODEL_LIB_DIR=$(abspath $(C_DIR_OUT_LIB)) -DLUNA_MODEL_INCLUDE_PATH=$(abspath $(C_DIR_OUT_INCLUDE))
	cmake --build $(C_DIR_TEST_BUILD)
	ctest -V --test-dir $(C_DIR_TEST_BUILD)

.PHONY: cformat fix_cformat
cformat:
	bash tools/run_clang_format.sh
fix_cformat:
	bash tools/run_clang_format.sh apply

CPP_DIR = cxx-lunamodel
CPP_SAMPLES_BUILD = $(CPP_DIR)/samples/build
CPP_TEST_BUILD = $(CPP_DIR)/test/build

.PHONY: cpp-samples cpp-test
cpp-samples: c
	cmake -S $(CPP_DIR)/samples -B $(CPP_SAMPLES_BUILD) -DLUNA_MODEL_LIB_DIR=$(abspath $(C_DIR_OUT_LIB)) -DLUNA_MODEL_INCLUDE_PATH=$(abspath $(C_DIR_OUT_INCLUDE))
	cmake --build $(CPP_SAMPLES_BUILD)

cpp-test: c
	cmake -S $(CPP_DIR)/test -B $(CPP_TEST_BUILD) -DLUNA_MODEL_LIB_DIR=$(abspath $(C_DIR_OUT_LIB)) -DLUNA_MODEL_INCLUDE_PATH=$(abspath $(C_DIR_OUT_INCLUDE))
	cmake --build $(CPP_TEST_BUILD)
	ctest --test-dir $(CPP_TEST_BUILD) -V
