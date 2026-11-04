cmake_minimum_required(VERSION 3.15...3.30)
project(luna_cxx_plugin_ext LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Emit compile_commands.json so editors/clangd can resolve the extension-mode build (the full
# Python + nanobind + luna_model include paths and -DLUNA_MODEL_PYTHON_EXTENSION). See `just
# cpp-plugin-dev`, which symlinks it next to the sources.
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Python + nanobind. nanobind ships its CMake config in its pip package; scikit-build-core puts it
# on CMAKE_PREFIX_PATH, so `find_package(nanobind CONFIG)` resolves it.
find_package(Python 3.11 REQUIRED COMPONENTS Interpreter Development.Module)
find_package(nanobind CONFIG REQUIRED)

# protobuf (+ its abseil dependency). On macOS these come from Homebrew, which is not on CMake's
# default search path, so query the brew prefixes and add them. Harmless where brew is absent (e.g.
# Linux CI): the system packages are found via the default paths.
find_program(BREW_EXECUTABLE brew)
if(BREW_EXECUTABLE)
  foreach(_formula protobuf abseil)
    execute_process(COMMAND ${BREW_EXECUTABLE} --prefix ${_formula}
                    OUTPUT_VARIABLE _prefix OUTPUT_STRIP_TRAILING_WHITESPACE
                    RESULT_VARIABLE _rc ERROR_QUIET)
    if(_rc EQUAL 0)
      list(PREPEND CMAKE_PREFIX_PATH "${_prefix}")
    endif()
  endforeach()
endif()
# Prefer protobuf's own CMake package (Homebrew, vcpkg, source builds). Distro packages such as
# Ubuntu's libprotobuf-dev ship none, so fall back to CMake's FindProtobuf module.
find_package(Protobuf CONFIG QUIET)
if(NOT Protobuf_FOUND)
  find_package(Protobuf REQUIRED)
endif()

# Discover the luna_model wheel's bundled artifacts the way a real plugin author does.
#   get_include() -> the C ABI + C++ SDK headers
#   get_proto()   -> the .proto wire schemas of built-in analysis results
execute_process(
  COMMAND "${Python_EXECUTABLE}" -c
          "import luna_model.capi,sys; sys.stdout.write(luna_model.capi.get_include())"
  OUTPUT_VARIABLE LUNA_MODEL_INCLUDE RESULT_VARIABLE _luna_include_rc)
if(NOT _luna_include_rc EQUAL 0)
  message(FATAL_ERROR "could not locate luna_model headers via luna_model.capi.get_include()")
endif()
execute_process(
  COMMAND "${Python_EXECUTABLE}" -c
          "import luna_model.capi,sys; sys.stdout.write(luna_model.capi.get_proto())"
  OUTPUT_VARIABLE LUNA_MODEL_PROTO RESULT_VARIABLE _luna_proto_rc)
if(NOT _luna_proto_rc EQUAL 0)
  message(FATAL_ERROR "could not locate luna_model protos via luna_model.capi.get_proto()")
endif()
message(STATUS "luna_model headers: ${LUNA_MODEL_INCLUDE}")
message(STATUS "luna_model protos:  ${LUNA_MODEL_PROTO}")

nanobind_add_module(luna_cxx_plugin_ext src/plugin.cpp)

set(_proto_gen "${CMAKE_CURRENT_BINARY_DIR}/proto_gen")
# FindProtobuf's protobuf_generate does not create PROTOC_OUT_DIR itself.
file(MAKE_DIRECTORY "${_proto_gen}")

# This plugin's own artifact/result messages.
protobuf_generate(
  TARGET luna_cxx_plugin_ext LANGUAGE cpp
  PROTOS "${CMAKE_CURRENT_SOURCE_DIR}/proto/plugin.proto"
  IMPORT_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/proto"
  PROTOC_OUT_DIR "${_proto_gen}")

# luna-model's built-in analysis wire schema (from the wheel): lets this C++ plugin consume a
# built-in (Rust) analysis result across the C ABI, generated from the exact same .proto.
protobuf_generate(
  TARGET luna_cxx_plugin_ext LANGUAGE cpp
  PROTOS "${LUNA_MODEL_PROTO}/luna_model/plugin.proto"
  IMPORT_DIRS "${LUNA_MODEL_PROTO}"
  PROTOC_OUT_DIR "${_proto_gen}")

target_include_directories(luna_cxx_plugin_ext
  PRIVATE "${LUNA_MODEL_INCLUDE}" "${_proto_gen}")
target_link_libraries(luna_cxx_plugin_ext PRIVATE protobuf::libprotobuf)

# Build as a luna-model Python extension: dispatch lm_* through the imported _C_API table and enable
# the SDK's Python-interop headers. Setting it as a definition (not a source #define) makes it appear
# in compile_commands.json, so clangd resolves the extension-mode view of the SDK headers.
target_compile_definitions(luna_cxx_plugin_ext PRIVATE LUNA_MODEL_PYTHON_EXTENSION)

install(TARGETS luna_cxx_plugin_ext LIBRARY DESTINATION .)
