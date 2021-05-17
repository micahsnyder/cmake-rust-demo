# Copyright (C) 2022 Micah Snyder.

#
# C executable
#
# Example app that used a static lib written in Rust and a traditional
# shared library written in C with some Rust sprinkled in.
#

add_executable(app_c)
target_sources(app_c PRIVATE app.c)
target_link_libraries(app_c
    PRIVATE
        demo::rust_static_lib
        demo::static_lib
        demo::gen_uuid)
if(WIN32)
    target_link_libraries(app_c
        PRIVATE
            Bcrypt)
endif()
install(TARGETS app_c DESTINATION ${CMAKE_INSTALL_BINDIR} COMPONENT programs)
