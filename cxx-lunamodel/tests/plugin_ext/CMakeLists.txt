cmake_minimum_required(VERSION 3.15...3.30)
project(luna_cxx_plugin_ext LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

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
find_package(Protobuf CONFIG REQUIRED)

# The luna_model wheel bundles the C ABI + C++ SDK headers; discover them the way a real plugin
# author does — `luna_model.capi.get_include()`. Requires luna-model in the build environment
# (declared as a build dependency in pyproject.toml).
execute_process(
  COMMAND "${Python_EXECUTABLE}" -c
          "import luna_model.capi,sys; sys.stdout.write(luna_model.capi.get_include())"
  OUTPUT_VARIABLE LUNA_MODEL_INCLUDE
  RESULT_VARIABLE _luna_include_rc)
if(NOT _luna_include_rc EQUAL 0)
  message(FATAL_ERROR "could not locate luna_model headers via luna_model.capi.get_include()")
endif()
message(STATUS "luna_model headers: ${LUNA_MODEL_INCLUDE}")

nanobind_add_module(luna_cxx_plugin_ext src/plugin.cpp proto/plugin.proto)

# Generate the C++ protobuf sources from the .proto and compile them into the module.
protobuf_generate(
  TARGET luna_cxx_plugin_ext
  LANGUAGE cpp
  IMPORT_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/proto"
  PROTOC_OUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/proto_gen")

target_include_directories(luna_cxx_plugin_ext
  PRIVATE "${LUNA_MODEL_INCLUDE}" "${CMAKE_CURRENT_BINARY_DIR}/proto_gen")
target_link_libraries(luna_cxx_plugin_ext PRIVATE protobuf::libprotobuf)

install(TARGETS luna_cxx_plugin_ext LIBRARY DESTINATION .)
