# Copyright (C) 2020 Micah Snyder.

configure_file(cmakerust-version.h.in cmakerust-version.h)

#
# Features written in Rust
#
# Note: These will be compiled to static library targets and must be added as
# dependencies to both the C object targets and shared/static library targets.
#

# Colorful Logging
add_rust_library(TARGET colorlog WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/colorlog")

configure_file(cmakerust-version.h.in cmakerust-version.h)

#
# The cmakerust C library
#
add_library( cmakerust_obj OBJECT )
target_sources( cmakerust_obj
    PRIVATE     lib.c   lib_private.h
    PUBLIC      cmakerust.h )
target_link_libraries( cmakerust_obj PUBLIC colorlog )
target_include_directories( cmakerust_obj PUBLIC ${CMAKE_CURRENT_BINARY_DIR} )

# The cmakerust shared library.
add_library( cmakerust SHARED )
target_sources( cmakerust PUBLIC cmakerust.h )
target_link_libraries( cmakerust PUBLIC cmakerust_obj )
target_include_directories( cmakerust PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR} )
if(WIN32)
    set_target_properties( cmakerust PROPERTIES WINDOWS_EXPORT_ALL_SYMBOLS ON)
endif()
add_library( CMakeRust::lib ALIAS cmakerust )

# The cmakerust static library.
add_library( cmakerust_static STATIC )
target_sources( cmakerust_static PUBLIC cmakerust.h )
target_link_libraries( cmakerust_static PUBLIC cmakerust_obj )
set_target_properties( cmakerust_static PROPERTIES ARCHIVE_OUTPUT_NAME cmakerust_static )
target_include_directories( cmakerust PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR} )
add_library( CMakeRust::static_lib ALIAS cmakerust_static )
