# Copyright (C) 2020-2022 Micah Snyder.

#
# The demo Rust library (features ported from the "original" C library to Rust)
#
# Note: The rust port will be compiled to static library target and must be added as
# a dependency to both the C object targets and shared/static library targets.
#
add_rust_library(TARGET demorust WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
if (WIN32)
    # You can add link library dependencies for the rust libarry target, if you need.
    target_link_libraries(demorust PUBLIC INTERFACE Userenv)
endif()

install(FILES $<TARGET_FILE:demorust> DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT libraries)
add_library(demo::rust_static_lib ALIAS demorust)

# The unit tests for the demorust module dependends on the C library and may even need
# to be linked with the C library's 3rd party library dependencies. So to be able to build
# that application, we'll need to pass these things through the environment.
# Thus, we'll set all of that up and add the Rust-test in `tests/CMakelists.txt`.
