# Copyright (C) 2020 Micah Snyder.

#
# Build example app that used a static lib written in Rust and a traditional
# shared library writtne in C with some Rust sprinkled in.
#

# The app.
add_executable( app )
target_sources( app PRIVATE app.c )
target_link_libraries( app
    PRIVATE
        CMakeRust::lib
        CMakeRust::gen_uuid )
if(WIN32)
    target_link_libraries( app
        PRIVATE
            Bcrypt )
endif()
