# Copyright (C) 2020 Micah Snyder.

#
# Rust common libraries
#

# A library to generate UUID's
add_rust_library(TARGET gen_uuid WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/gen_uuid")
add_rust_test(NAME gen_uuid WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/gen_uuid")
add_library(CMakeRust::gen_uuid ALIAS gen_uuid)
