cmake_minimum_required(VERSION 3.27)
project(rust_ffi_demo  C CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Пути
set(RUST_PROJECT_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set(RUST_TARGET_DIR ${RUST_PROJECT_DIR}/target)

# Определяем профиль cargo по типу сборки CMake
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_PROFILE debug)
    set(CARGO_CMD cargo build)
else()
    set(RUST_PROFILE release)
    set(CARGO_CMD cargo build --release)
endif()

# 1) Собираем Rust библиотеку
set(RUST_LIB_PREFIX ${RUST_TARGET_DIR}/${RUST_PROFILE})

add_custom_command(
    OUTPUT ${RUST_LIB_PREFIX}/librust_ffi_demo.a
           ${RUST_LIB_PREFIX}/librust_ffi_demo.so
           ${RUST_LIB_PREFIX}/librust_ffi_demo.dylib
    COMMAND ${CARGO_CMD}
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    COMMENT "Building Rust library (${RUST_PROFILE}) with cargo"
)

add_custom_target(rustlib ALL
    DEPENDS ${RUST_LIB_PREFIX}/librust_ffi_demo.a
            ${RUST_LIB_PREFIX}/librust_ffi_demo.so
            ${RUST_LIB_PREFIX}/librust_ffi_demo.dylib
)

# 2) Генерируем заголовок через cbindgen
set(GENERATED_HEADER ${RUST_PROJECT_DIR}/include/rust_ffi_demo.h)

add_custom_command(
    OUTPUT ${GENERATED_HEADER}
    COMMAND cbindgen -v --config ${RUST_PROJECT_DIR}/cbindgen.toml
                     --crate rust-ffi-demo
                     --output ${GENERATED_HEADER}
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    DEPENDS rustlib
    COMMENT "Generating C header with cbindgen"
)

add_custom_target(header ALL DEPENDS ${GENERATED_HEADER})

# 3) Собираем C++ пример
add_executable(cpp_example cpp/main.cpp ${GENERATED_HEADER})
add_dependencies(cpp_example rustlib header)

target_include_directories(cpp_example PRIVATE ${RUST_PROJECT_DIR}/include)

if(WIN32)
    target_link_libraries(cpp_example PRIVATE ${RUST_LIB_PREFIX}/rust_ffi_demo.dll.lib)
elseif(APPLE)
    target_link_libraries(cpp_example PRIVATE ${RUST_LIB_PREFIX}/librust_ffi_demo.dylib)
else()
    target_link_libraries(cpp_example PRIVATE ${RUST_LIB_PREFIX}/librust_ffi_demo.so)
endif()

# Прописываем RPATH, чтобы бинарь видел библиотеку в target/{debug,release}
if(UNIX AND NOT APPLE)
    set_target_properties(cpp_example PROPERTIES
        BUILD_WITH_INSTALL_RPATH ON
        INSTALL_RPATH "${RUST_LIB_PREFIX}"
    )
elseif(APPLE)
    set_target_properties(cpp_example PROPERTIES
        BUILD_WITH_INSTALL_RPATH ON
        INSTALL_RPATH "${RUST_LIB_PREFIX}"
        MACOSX_RPATH ON
    )
endif()