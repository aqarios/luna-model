cmake_minimum_required(VERSION 3.20)
project(lunamodel-ctest LANGUAGES C)

set(LUNA_MODEL_INCLUDE_PATH "" CACHE PATH "path to dist/c/include")
set(LUNA_MODEL_LIB_DIR "" CACHE PATH "path to dist/c/lib")

enable_testing()

find_library(LUNA_MODEL_LIB lunamodel PATHS ${LUNA_MODEL_LIB_DIR} REQUIRED)
# So the test binary finds @rpath/liblunamodel.dylib at runtime.
set(CMAKE_BUILD_RPATH "${LUNA_MODEL_LIB_DIR}")

if(MSVC)
  add_compile_options(/W3 /WX)
else()
  add_compile_options(-Wall -Wextra -Werror)
endif()

# Gather test_*.c into a single CTest driver executable.
file(GLOB test_files RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} test_*.c)
create_test_sourcelist(driver_sources test_driver.c ${test_files})

add_executable(test_driver ${driver_sources})
target_include_directories(test_driver PRIVATE ${LUNA_MODEL_INCLUDE_PATH} ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(test_driver ${LUNA_MODEL_LIB})

foreach(test_file ${test_files})
  get_filename_component(test_name ${test_file} NAME_WE)
  add_test(NAME ${test_name} COMMAND test_driver ${test_name})
endforeach()
