# Copyright (C) 2020-2022 Micah Snyder.

cmake_minimum_required(VERSION 3.18)
set(RUSTC_MINIMUM_REQUIRED 1.56)

project(RustCMakeDemo
        VERSION "0.2.0"
        DESCRIPTION "A demo app to show a CMake project with components written in Rust.")

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})

#
# Find Build Tools
#
set(MAINTAINER_MODE_DEFAULT OFF)
option(MAINTAINER_MODE
    "Use `cbindgen` to generate Rust library API headers."
    ${MAINTAINER_MODE_DEFAULT})

find_package(Rust REQUIRED)

# Always use '-fPIC'/'-fPIE' option.
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Include GNUInstallDirs for access to CMAKE_INSTALL_LIBDIR, etc
include(GNUInstallDirs)

# Enable CTest
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    include(CTest)
    enable_testing()
endif()

# Enable source packages with CPack

#
# Configure CPack
#  - for installers
#  - for source tarball
#

set(CPACK_PACKAGE_NAME ${PACKAGE_STRING})

#set(CPACK_PACKAGE_ICON ${PROJECT_SOURCE_DIR}/logo.png)
#set(CPACK_PACKAGE_RELOCATABLE TRUE)
set(CPACK_STRIP_FILES TRUE)

set(CPACK_PACKAGE_VERSION_MAJOR ${RustCMakeDemo_VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${RustCMakeDemo_VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${RustCMakeDemo_VERSION_PATCH})
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY ${PROJECT_DESCRIPTION})
set(CPACK_PACKAGE_CONTACT https://blog.micahsnyder.dev/)

# point to readme and license files
set(CPACK_RESOURCE_FILE_README ${PROJECT_SOURCE_DIR}/README.md)
set(CPACK_RESOURCE_FILE_LICENSE ${PROJECT_SOURCE_DIR}/LICENSE-MIT.md)

set(CPACK_PACKAGE_FILE_NAME RustCMakeDemo-${PROJECT_VERSION})

set(CPACK_SOURCE_GENERATOR "TGZ")
set(CPACK_SOURCE_PACKAGE_FILE_NAME ${CPACK_PACKAGE_FILE_NAME})
set(CPACK_SOURCE_IGNORE_FILES "\\.git/" ".*~$" "^${CMAKE_BINARY_DIR}")
set(CPACK_VERBATIM_VARIABLES YES)

if (WIN32)
    set(CMAKE_INSTALL_SYSTEM_RUNTIME_DESTINATION .)
    include(InstallRequiredSystemLibraries)

    set(CPACK_GENERATOR                  "ZIP;NSIS")
    set(CPACK_PACKAGE_VENDOR             "Micah Snyder")
    set(CPACK_PACKAGE_FILE_NAME ${CPACK_PACKAGE_FILE_NAME}.win.${CMAKE_VS_PLATFORM_NAME})

    set(CPACK_PACKAGE_INSTALL_DIRECTORY  "RustCMakeDemo")

    install(
        FILES
            ${CMAKE_CURRENT_SOURCE_DIR}/README.md
        DESTINATION ".")

elseif(APPLE)
    set(CPACK_PACKAGE_NAME      RustCMakeDemo)
    set(CPACK_GENERATOR         productbuild)
    set(CPACK_PACKAGE_FILE_NAME ${CPACK_PACKAGE_FILE_NAME}.macos.universal)
    set(CPACK_PACKAGE_VENDOR    "micahsnyder")  # For Apple, creates short name com.micahsnyder.RustCMakeDemo.xxx in pkgutil

    # productbuild requires components or it won't add anything to the PKG
    set(CPACK_COMPONENT_LIBRARIES_DISPLAY_NAME "Libraries")
    set(CPACK_COMPONENT_LIBRARIES_DESCRIPTION "The RustCMakeDemo libraries including the header files for C/C++ development.")
    set(CPACK_COMPONENT_DOCUMENTATION_DISPLAY_NAME "Documentation")
    set(CPACK_COMPONENT_DOCUMENTATION_DESCRIPTION "Manpages and HTML documentation.")
    set(CPACK_COMPONENT_PROGRAMS_DISPLAY_NAME "Programs")
    set(CPACK_COMPONENT_PROGRAMS_DESCRIPTION "The RustCMakeDemo toolkit.")

    # dependencies between components
    set(CPACK_COMPONENT_DOCUMENTATION_DEPENDS libraries)
    set(CPACK_COMPONENT_PROGRAMS_DEPENDS libraries)
    set(CPACK_COMPONENT_LIBRARIES_REQUIRED ON) # always install the libs
    set(CPACK_COMPONENTS_ALL libraries documentation programs)

    if(RustCMakeDemo_SIGN_FILE)
        # This tells Xcode's signing phase to pretend that the binary was signed by the linker.
        # Then install_name_tool is willing to revise the ad-hoc signature.
        # See: https://gitlab.kitware.com/cmake/cmake/-/issues/21854#note_907691
        set(CMAKE_XCODE_ATTRIBUTE_OTHER_CODE_SIGN_FLAGS "-o linker-signed")

        # Stripping the files will invalidate the signatures.
        set(CPACK_STRIP_FILES FALSE)

        # Convert NEWS.md to HTML to match Apple productbuild requirements.
        # Not in the path, try using: python3 -m pytest
        execute_process(
            COMMAND ${Python3_EXECUTABLE} -m markdown --version
            RESULT_VARIABLE MARKDOWN_MODULE_EXIT_CODE
            ERROR_QUIET OUTPUT_QUIET
        )
        if(${MARKDOWN_MODULE_EXIT_CODE} EQUAL 0)
            # The markdown module is installed, we can do the conversion.
            set(PythonTest_COMMAND "${Python3_EXECUTABLE};-m;markdown;-v")

            execute_process(
                COMMAND echo "<html><body>"
                OUTPUT_FILE ${CMAKE_CURRENT_BINARY_DIR}/prefix.html)
            execute_process(
                COMMAND ${Python3_EXECUTABLE} -m markdown ${CMAKE_CURRENT_SOURCE_DIR}/NEWS.md
                OUTPUT_FILE ${CMAKE_CURRENT_BINARY_DIR}/body.html)
            execute_process(
                COMMAND echo "</html></body>"
                OUTPUT_FILE ${CMAKE_CURRENT_BINARY_DIR}/suffix.html)
            execute_process(
                COMMAND cat ${CMAKE_CURRENT_BINARY_DIR}/prefix.html
                            ${CMAKE_CURRENT_BINARY_DIR}/body.html
                            ${CMAKE_CURRENT_BINARY_DIR}/suffix.html
                OUTPUT_FILE ${CMAKE_CURRENT_BINARY_DIR}/NEWS.html)

            set(CPACK_RESOURCE_FILE_README ${PROJECT_BINARY_DIR}/NEWS.html)
        else()
            message("Failed to detect the Python 3 'markdown' package.")
            message(FATAL_ERROR "Try running 'pip3 install --user markdown' and then try again.")
        endif()
    endif()

elseif(C_BSD)
    # User must specify `-G FreeBSD` when building FreeBSD packages
    # Disclaimer: doesn't seem to actually exist, despite being in the documentation.
    set(CPACK_PACKAGE_NAME             RustCMakeDemo)
    set(CPACK_PACKAGE_FILE_NAME        ${CPACK_PACKAGE_FILE_NAME}.freebsd.${CMAKE_SYSTEM_PROCESSOR})

    set(CPACK_FREEBSD_PACKAGE_LICENSE "MIT")
    set(CPACK_FREEBSD_PACKAGE_MAINTAINER "https://blog.micahsnyder.dev/")

else()
    # User must specify `-G DEB` or `-G RPM` when building Linux packages
    set(CPACK_PACKAGE_NAME             RustCMakeDemo)
    set(CPACK_PACKAGE_VERSION          ${PROJECT_VERSION}${VERSION_SUFFIX})
    set(CPACK_PACKAGE_FILE_NAME        ${CPACK_PACKAGE_FILE_NAME}.linux.${CMAKE_SYSTEM_PROCESSOR})

    set(CPACK_RPM_PACKAGE_LICENSE "MIT")
    set(CPACK_RPM_PACKAGE_AUTOREQ " no")
    set(CPACK_RPM_EXCLUDE_FROM_AUTO_FILELIST_ADDITION
        "/usr/sbin"
        "/usr/local"
        "/usr/local/bin"
        "/usr/local/etc"
        "/usr/local/include"
        "/usr/local/lib"
        "/usr/local/sbin"
        "/usr/local/share"
        "/usr/local/share/man"
        "/usr/local/share/man/man1"
        "/usr/local/share/man/man5"
        "/usr/local/share/man/man8"
    )
endif()

# Including CPack must be the very last CPack thing we do.
include(CPack)

#
# Build targets.
#
add_subdirectory(lib)
add_subdirectory(common)
add_subdirectory(app_c)
add_subdirectory(app_rust)
add_subdirectory(test)

#
# The Summary Info.
#
string(TOUPPER "${CMAKE_BUILD_TYPE}" _build_type)
message(STATUS "Configuration Options Summary --
    Target system:          ${CMAKE_SYSTEM}
    Compiler:
        Build type:         ${CMAKE_BUILD_TYPE}
        C compiler:         ${CMAKE_C_COMPILER}
        Rust toolchain:     ${cargo_EXECUTABLE} (${cargo_VERSION})
                            ${rustc_EXECUTABLE} (${rustc_VERSION})
        CFLAGS:             ${CMAKE_C_FLAGS_${_build_type}} ${CMAKE_C_FLAGS}
    Build Options:
        Maintainer Mode:    ${MAINTAINER_MODE}")
