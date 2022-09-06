# Copyright (C) 2022 Micah Snyder.

#
# Rust executable
#
# Example app that links with our C (and Rust) libs.
#

add_rust_executable(TARGET app_rust
    SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    BINARY_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)
# Linking with C libraries is done within `build.rs`
# Linking with nearby Rust crates is done within `Cargo.toml`
get_target_property(app_rust_EXECUTABLE app_rust IMPORTED_LOCATION)
install(PROGRAMS ${app_rust_EXECUTABLE} DESTINATION ${CMAKE_INSTALL_BINDIR} COMPONENT programs)
