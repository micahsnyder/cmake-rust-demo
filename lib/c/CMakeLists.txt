# Copyright (C) 2020-2022 Micah Snyder.

configure_file(demo-version.h.in demo-version.h)

#
# The demo C library
#
add_library(demo_obj OBJECT)
target_sources(demo_obj
    PRIVATE
        demo.c      demo-private.h
        dostuff.c   dostuff.h
    PUBLIC
        demo.h
)
target_link_libraries(demo_obj PUBLIC demorust)
if(WIN32)
    target_link_libraries(demo_obj PUBLIC Userenv wsock32 ws2_32)
endif()
target_include_directories(demo_obj PUBLIC ${CMAKE_CURRENT_BINARY_DIR})

# The demo shared library.
add_library(demo SHARED)
target_sources(demo PUBLIC demo.h)
target_link_libraries(demo PUBLIC demo_obj)
target_include_directories(demo PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR})
if(WIN32)
    set_target_properties(demo PROPERTIES WINDOWS_EXPORT_ALL_SYMBOLS ON)
    target_link_libraries(demo PRIVATE Bcrypt)
endif()
install(TARGETS demo DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT libraries)
add_library(demo::lib ALIAS demo)

# The demo static library.
add_library(demo_static STATIC)
target_sources(demo_static PUBLIC demo.h)
target_link_libraries(demo_static PUBLIC demo_obj)
target_include_directories(demo PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR})
set_target_properties(demo_static PROPERTIES ARCHIVE_OUTPUT_NAME demo_static)
install(TARGETS demo_static DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT libraries)
add_library(demo::static_lib ALIAS demo_static)
