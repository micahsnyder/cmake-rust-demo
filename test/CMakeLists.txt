# Copyright (C) 2020-2022 Micah Snyder.

#
# Targets for all the application tests and main library unit tests.
#

if(WIN32)
    file(TO_NATIVE_PATH $<TARGET_FILE:demo> LIBDEMO)
else()
    set(LD_LIBRARY_PATH $<TARGET_FILE_DIR:demo>:$ENV{LD_LIBRARY_PATH})
    set(LIBDEMO $<TARGET_FILE:demo>)
endif()

set(ENVIRONMENT
    LD_LIBRARY_PATH=${LD_LIBRARY_PATH}
    DYLD_LIBRARY_PATH=${LD_LIBRARY_PATH}
    LIBDEMO=${LIBDEMO}
)

add_rust_test(NAME demorust WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/lib/rust")
set_property(TEST demorust PROPERTY ENVIRONMENT ${ENVIRONMENT})
