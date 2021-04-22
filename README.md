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

## Building this project

Requirements:
- CMake 3.18+
- The Rust toolchain (Cargo, etc)

Run:
```bash
mkdir build && cd build
cmake .. && cmake --build . && ctest
```

## Vendoring dependencies

For building a source package with CPack (E.g., the `TGZ` package archive generator), you may wish to vendor the Cargo dependencies so your users can do offline builds when using your source package.

The `FindRust.cmake` module makes that easy. Simply define `VENDOR_DEPENDENCIES=ON` and it will run `cargo vendor` during the configuration stage. The dependencies and assocaited `config.toml` file, instructing cargo to use the vendored dependencies, will be placed in a `.cargo` directory next to each of your `Cargo.toml` files.

Example building a source tarball with vendored dependencies:
```bash
mkdir build && cd build
cmake .. -D VENDOR_DEPENDENCIES=ON && cpack --config CPackSourceConfig.cmake
```

Afterwards, you should have a source package (E.g., `cmake-rust-0.1.0.tar.gz`) containing the project with vendored dependencies in your working directory, and if you run `git status`, you'll see the `.cargo` directories:
```bash
Untracked files:
  (use "git add <file>..." to include in what will be committed)
        ../common/gen_uuid/.cargo/
        ../lib/colorlog/.cargo/
```

At this point you could do something like this, to see it build w/out downloading the crates:
```bash
tar xzf cmake-rust-0.1.0.tar.gz
cd cmake-rust-0.1.0
mkdir build && cd build
cmake .. && cmake --build . && ctest
```

You'll note that the vendored dependencies appear in your source directory. To remove them, you can do this:
```bash
rm -rf **/.cargo
```

## License

This project is dual-licensed under MIT and Apache 2.0.

## Contribute

This project could use your help testing build support for various targets on various operating systems.
Please feel free to help work on the outstanding issues too!
