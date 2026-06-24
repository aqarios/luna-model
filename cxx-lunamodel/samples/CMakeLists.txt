cmake_minimum_required(VERSION 3.20)
project(lunamodel-cpp-samples LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(LUNA_MODEL_INCLUDE_PATH "" CACHE PATH "path to dist/c/include")
set(LUNA_MODEL_LIB_DIR "" CACHE PATH "path to dist/c/lib")
set(LUNA_MODEL_CPP_SRC "${CMAKE_CURRENT_SOURCE_DIR}/../src")

find_library(LUNA_MODEL_LIB lunamodel PATHS ${LUNA_MODEL_LIB_DIR} REQUIRED)
set(CMAKE_BUILD_RPATH "${LUNA_MODEL_LIB_DIR}")

if(MSVC)
  add_compile_options(/W3 /WX)
else()
  add_compile_options(-Wall -Wextra -Werror)
endif()

# One line per sample: add_sample(<name>) builds <name>.cpp.
function(add_sample name)
  add_executable(${name} ${name}.cpp)
  target_include_directories(${name} PRIVATE ${LUNA_MODEL_CPP_SRC} ${LUNA_MODEL_INCLUDE_PATH})
  target_link_libraries(${name} ${LUNA_MODEL_LIB})
endfunction()

add_sample(model_roundtrip)
