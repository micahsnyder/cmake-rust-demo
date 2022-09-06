# Copyright (C) 2020-2022 Micah Snyder.

#
# Libraries that may be used by the applications.
#

# A library to generate UUID's
add_rust_library(TARGET gen_uuid
    SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    BINARY_DIRECTORY "${CMAKE_BINARY_DIR}"
)

# The unit tests for this module have no dependencies on C libraries or other special
# test environment considerations, so we may as well add the test right here instead of
# adding it in the `test/CMakeLists.txt` file.
add_rust_test(NAME gen_uuid
    SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    BINARY_DIRECTORY "${CMAKE_BINARY_DIR}"
)
add_library(demo::gen_uuid ALIAS gen_uuid)
