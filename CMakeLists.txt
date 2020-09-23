# Copyright (C) 2020 Micah Snyder.

cmake_minimum_required(VERSION 3.18)

project( RustCMakeDemo
         VERSION "0.1.0"
         DESCRIPTION "A demo app to show a CMake project with components written in Rust." )

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})

#
# Find Build Tools
#
set(MAINTAINER_MODE_DEFAULT OFF)
option(MAINTAINER_MODE
    "Use `cbindgen` to generate Rust library API headers."
    ${MAINTAINER_MODE_DEFAULT})

if(MAINTAINER_MODE)
    set(cbindgen_REQUIRED 1)
endif()
find_package(Rust REQUIRED)

# Always use '-fPIC'/'-fPIE' option.
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

#
# Build targets.
#
add_subdirectory( lib )
add_subdirectory( common )
add_subdirectory( app )

#
# The Summary Info.
#
string(TOUPPER "${CMAKE_BUILD_TYPE}" _build_type)
message(STATUS "Configuration Options Summary --
    Target system:          ${CMAKE_SYSTEM}
    Compiler:
        Build type:         ${CMAKE_BUILD_TYPE}
        C compiler:         ${CMAKE_C_COMPILER}
        Rust toolchain:     ${cargo_EXECUTABLE} (${cargo_VERSION})
        CFLAGS:             ${CMAKE_C_FLAGS_${_build_type}} ${CMAKE_C_FLAGS}
    Build Options:
        Maintainer Mode:    ${MAINTAINER_MODE}")
if(MAINTAINER_MODE)
message("\
        cbindgen:           ${cbindgen_EXECUTABLE} (${cbindgen_VERSION})")
endif()