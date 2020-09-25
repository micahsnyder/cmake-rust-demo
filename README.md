# CMake Rust Demo

![Build Test](https://github.com/micahsnyder/cmake-rust-demo/workflows/Build%20Test/badge.svg)

A C CMake demo using Rust static library components.
The notable feature of this project is [cmake/FindRust.cmake](cmake/FindRust.cmake)

## Usage

Add `FindRust.cmake` to your project's `cmake` directory and use the following to enable Rust support:

```cmake
if(MAINTAINER_MODE)
    set(cbindgen_REQUIRED 1)
endif()
find_package(Rust REQUIRED)
```

To build a rust library and link it into your app, use:

```cmake
add_rust_library( TARGET yourlib WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/yourlib" )

add_executable( yourexe )
target_sources( yourexe PRIVATE yourexe.c )
target_link_libraries( yourexe yourlib )
```

For unit test support, you can use the `add_rust_test()` function, like this:

```cmake
add_rust_library( TARGET yourlib WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/yourlib" )
add_rust_test( NAME yourlib WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/yourlib" )
```

And don't forget to enable CTest early in your top-level `CMakeLists.txt` file:

```cmake
# Enable CTest
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    include(CTest)
    enable_testing()
endif()
```

## `cbindgen` C API generation support

If you set `cbindgen_REQUIRED` as shown above, then `cbindgen` will need to be installed. It will also require a `cbindgen.toml` file next to each `Cargo.toml`.

## License

This project is dual-licensed under MIT and Apache 2.0.

## Contribute

This project could use your help testing build support for various targets on various operating systems.
Please feel free to help work on the outstanding issues too!
