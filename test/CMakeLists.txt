# Copyright (C) 2020-2022 Micah Snyder.

#
# Targets for all the application tests and main library unit tests.
#

if(WIN32)
    file(TO_NATIVE_PATH $<TARGET_FILE:demo_static> LIBDEMO)
else()
    set(LIBDEMO $<TARGET_FILE:demo_static>)
endif()

set(ENVIRONMENT
    LIBDEMO=${LIBDEMO}
)

add_rust_test(NAME demorust
    SOURCE_DIRECTORY "${CMAKE_SOURCE_DIR}/lib/rust"
    BINARY_DIRECTORY "${CMAKE_BINARY_DIR}"
)
set_property(TEST demorust PROPERTY ENVIRONMENT ${ENVIRONMENT})
