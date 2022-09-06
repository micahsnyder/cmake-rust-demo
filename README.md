# CMake Rust Demo

![Build Test](https://github.com/micahsnyder/cmake-rust-demo/workflows/Build%20Test/badge.svg)

A C CMake demo using Rust static library components and building Rust executables.
The notable feature of this project is [cmake/FindRust.cmake](cmake/FindRust.cmake)

## Usage

Add `FindRust.cmake` to your project's `cmake` directory and use the following to enable Rust support:

```cmake
find_package(Rust REQUIRED)
```

### Rust-based C-style Static Libraries

To build a rust library and link it into your app, use:

```cmake
add_rust_library(TARGET yourlib
  SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  BINARY_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)

add_executable(yourexe)
target_sources(yourexe PRIVATE yourexe.c)
target_link_libraries(yourexe yourlib)
```

### Rust Library Unit Tests

For unit test support, you can use the `add_rust_test()` function, like this:

```cmake
add_rust_library(TARGET yourlib
  SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  BINARY_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)
add_rust_test(NAME yourlib
  SOURCE_DIRECTORY "${CMAKE_SOURCE_DIR}/path/to/yourlib"
  BINARY_DIRECTORY "${CMAKE_BINARY_DIR}/path/to/yourlib"
)
```

And don't forget to enable CTest early in your top-level `CMakeLists.txt` file:

```cmake
# Enable CTest
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    include(CTest)
    enable_testing()
endif()
```

### Rust-based Executables

To build a rust executable use:

```cmake
add_rust_executable(TARGET yourexe
  SOURCE_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  BINARY_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)
add_executable(YourProject::yourexe ALIAS yourexe)
```

## Minimum Rust version

You may set the CMake variable `RUSTC_MINIMUM_REQUIRED` to enforce a minimum Rust version, such as "1.56" for the 2021 edition support.

## `cbindgen` and `bindgen` FFI generation

This project demonstrates using `bindgen` and `cbindgen` within the `<src>/lib/rust/build.rs` script to generate the API's at build time.

`cbindgen` is used to generate a `demorust.h` C binding for the Rust exports every time you build. It'll be dropped in the build directory.

`bindgen`, on the otherhand, generates a `sys.rs` Rust binding for the C exports required by the Rust code. Unfortunately, `bindgen` depends on libclang for some features that aren't readily available on all systems. So we only run `bindgen` if you set the `MAINTAINER_MODE` CMake parameter to `ON`. That works out okay, as the `sys.rs` file is dropped into the source directory, not the build directory. But that means you have to remember to build with `MAINTAINER_MODE=ON` any time you change the internal C API's used by the Rust library.

The `cbindgen` and `bindgen` programs don't need to be pre-installed. `Cargo.toml` will pull them in during the build.

## Building this project

Requirements:
- CMake 3.18+
- The Rust toolchain (Cargo, etc)

Run:
```bash
mkdir build && cd build
cmake .. \
  -D MAINTAINER_MODE=ON \
  -D CMAKE_INSTALL_PREFIX=install
cmake --build .
ctest -V
cmake --build . --target install
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

# Installing Rust binaries with CMake

C static libraries don't link in other static libraries. Fully-static builds of projects that are composed of a few different static libraries will result in a collection of static libraries. If you're only providing an application, then you can link them into the app and only install the app. But if you provide a library for others to consume then you may need to install all of the static libraries that compose the features of "the library" you provide for downstream projects.

For example, let's say you've got a C library that may be built as a shared lib AND as a static lib, and you're slowly porting the C code into a Rust static library. You will link your Rust static library into the C library to maintain the original C API for your downstream users. With a shared-build of the C library, the Rust static library will get linked into the shared library and the downstream users never have to know it exists. Buf for a static-build of the C library, the C library is linked *against* your Rust library but they remain two separate libs that must both be linked with the downstream applications. You will need to install both the C static library *and* the Rust static library.

Projects built with CMake can use CMake to install the software directly (e.g. under `/usr/local`) or via a packager like WiX Toolset (Windows) or `.deb` / `.rpm` / `.pkg` packages.

Rust binaries aren't treated quite the same by CMake as native C binaries, but you can use CMake to install them.

## How to install Rust binaries with CMake

The first thing you'll probably need if you want to use CMake to install stuff, whether or not you bundle in some Rust binaries, is to include the GNUEInstallDirs module somewhere at the top of your top-level `CMakeLists.txt`:

```cmake
include(GNUInstallDirs)
```

Now with a regular C library or executable CMake target, you might configure them for installation like this:
```cmake
install(TARGETS demo DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT libraries)
```
or:
```cmake
install(TARGETS app DESTINATION ${CMAKE_INSTALL_BINDIR} COMPONENT programs)
```

Rust library CMake targets aren't normal CMake binary targets though. They're "custom" targets, which means you will instead have to use `install(FILES` instead of `install(TARGETS`, and then point CMake at the specific file you need installed instead of at a target. Our `FindRust.cmake`'s `add_rust_library()` function makes this easy. When you add a Rust library, it sets the target properties such that you can simply use CMake's `$<TARGET_FILE:target>` [generator expression](https://cmake.org/cmake/help/latest/manual/cmake-generator-expressions.7.html) to provide the file path.

In this demo, we configure installation for our `demorust` Rust static library like this:
```cmake
install(FILES $<TARGET_FILE:demorust> DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT libraries)
```

And for our `app_rust` Rust executable, we install like this:
```cmake
get_target_property(app_rust_EXECUTABLE app_rust IMPORTED_LOCATION)
install(PROGRAMS ${app_rust_EXECUTABLE} DESTINATION ${CMAKE_INSTALL_BINDIR} COMPONENT programs)
```
Note that we have to get the `IMPORTED_LOCATION` manually for the executable.

## License

This project is dual-licensed under MIT and Apache 2.0.

## Contribute

This project could use your help testing build support for various targets on various operating systems.
Please feel free to help work on the outstanding issues too!
