use std::env;
use std::path::{Path, PathBuf};

use bindgen::builder;

// A list of environment variables to query to determine additional libraries
// that need to be linked to resolve dependencies.
const LIB_ENV_LINK: &[&str] = &["LIBDEMO"];

// Additional [verbatim] libraries to link on Windows platforms
const LIB_LINK_WINDOWS: &[&str] = &["wsock32", "ws2_32", "Shell32", "User32"];

// Generate bindings for these functions:
const BINDGEN_FUNCTIONS: &[&str] = &["do_the_thing"];

// Generate bindings for these types (structs, enums):
const BINDGEN_TYPES: &[&str] = &[];

// Find the required functions and types in these headers:
const BINDGEN_HEADERS: &[&str] = &["../c/dostuff.h"];

// Find the required headers in these directories:
const BINDGEN_INCLUDE_PATHS: &[&str] = &["-I../lib/c"];

// Write the bindings to this file:
const BINDGEN_OUTPUT_FILE: &str = "src/sys.rs";

// Environment variable name prefixes worth including for diags
const ENV_PATTERNS: &[&str] = &["CARGO_", "RUST", "LIB"];

fn main() -> Result<(), &'static str> {
    eprintln!("build.rs command line: {:?}", std::env::args());
    eprintln!("Environment:");
    std::env::vars()
        .filter(|(k, _)| ENV_PATTERNS.iter().any(|prefix| k.starts_with(prefix)))
        .for_each(|(k, v)| eprintln!("  {}={:?}", k, v));

    // We only want to generate bindings for `cargo build`, not `cargo test`.
    // FindRust.cmake defines $CARGO_CMD so we can differentiate.
    let cargo_cmd = env::var("CARGO_CMD").unwrap_or_else(|_| "".into());

    match cargo_cmd.as_str() {
        "build" => {
            println!("cargo:rerun-if-env-changed=LIBDEMO");

            // Generate bindings as a part of the build.

            let maintainer_mode = env::var("MAINTAINER_MODE").unwrap_or_else(|_| "".into());
            if maintainer_mode == "ON" {
                // Only generate the `.rs` bindings when maintainer-mode is enabled.
                // Bindgen requires libclang, which may not readily available, so we will commit the
                // bindings to version control and use maintainer-mode to update them, as needed.
                // On the plus-side, this means that our `.rs` file is present before our first build,
                // so at least rust-analyzer will be happy.
                generate_rust_bindings()?;
            }

            // Link executable with library dependencies.
            for var in LIB_ENV_LINK {
                if !search_and_link_lib(var)? {
                    eprintln!("Undefined library dependency environment variable: {}", var);
                    return Err("Undefined library dependency environment variable");
                }
            }

            if cfg!(windows) {
                for lib in LIB_LINK_WINDOWS {
                    println!("cargo:rustc-link-lib={}", lib);
                }
            }
        }

        _ => {
            return Ok(());
        }
    }

    Ok(())
}

/// Use bindgen to generate Rust bindings to call into C libraries.
fn generate_rust_bindings() -> Result<(), &'static str> {
    let build_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| ".".into()));
    let build_include_path = format!("-I{}", build_dir.join(".").to_str().unwrap());

    // Configure and generate bindings.
    let mut builder = builder()
        // Silence code-style warnings for generated bindings.
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]")
        // Make the bindings pretty.
        .rustfmt_bindings(true)
        // Disable the layout tests because we're committing `sys.rs` to source control.
        // Pointer width, integer size, etc. are probably not the same when generated as when compiled.
        .layout_tests(false)
        // Enable bindgen to find generated headers in the build directory, too.
        .clang_arg(build_include_path);

    for &include_path in BINDGEN_INCLUDE_PATHS {
        builder = builder.clang_arg(include_path);
    }
    for &header in BINDGEN_HEADERS {
        builder = builder.header(header);
    }
    for &c_function in BINDGEN_FUNCTIONS {
        builder = builder.allowlist_function(c_function);
    }
    for &c_type in BINDGEN_TYPES {
        builder = builder.allowlist_type(c_type);
    }

    // Generate!
    builder
        .generate()
        .expect("Unable to generate Rust bindings for C code")
        .write_to_file(BINDGEN_OUTPUT_FILE)
        .expect("Failed to write Rust bindings to output file");

    eprintln!("bindgen outputting \"{}\"", BINDGEN_OUTPUT_FILE);

    Ok(())
}

/// Return whether the specified environment variable has been set, and output
/// linking directives as a side-effect
fn search_and_link_lib(environment_variable: &str) -> Result<bool, &'static str> {
    eprintln!("  - checking for {:?} in environment", environment_variable);
    let filepath_str = match env::var(environment_variable) {
        Err(env::VarError::NotPresent) => return Ok(false),
        Err(env::VarError::NotUnicode(_)) => return Err("environment value not unicode"),
        Ok(s) => {
            if s.is_empty() {
                return Ok(false);
            } else {
                s
            }
        }
    };

    let parsed_path = parse_lib_path(&filepath_str)?;
    eprintln!(
        "  - adding {:?} to rustc library search path",
        &parsed_path.dir
    );
    println!("cargo:rustc-link-search={}", parsed_path.dir);
    eprintln!("  - requesting that rustc link {:?}", &parsed_path.libname);
    println!("cargo:rustc-link-lib={}", parsed_path.libname);

    Ok(true)
}

/// Struct to store a lib name and directory.
/// Not the
struct ParsedLibraryPath {
    dir: String,
    libname: String,
}

/// Parse a library path, returning:
/// - the directory containing the library
/// - the portion expected after the `-l`
fn parse_lib_path<'a>(path: &'a str) -> Result<ParsedLibraryPath, &'static str> {
    let path = PathBuf::from(path);
    let file_name = path
        .file_name()
        .ok_or("file name not found")?
        .to_str()
        .ok_or("file name not unicode")?;

    // This can't fail because it came from a &str
    let dir = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_str()
        .unwrap()
        .to_owned();

    // Grab the portion up to the first '.'
    let full_libname = file_name
        .split('.')
        .next()
        .ok_or("no '.' found in file name")?;

    let libname = if !cfg!(windows) {
        // Trim off the "lib" for Linux/Unix systems
        full_libname
            .strip_prefix("lib")
            .ok_or(r#"file name doesn't begin with "lib""#)?
    } else {
        // Keep the full libname on Windows.
        full_libname
    }
    .to_owned();

    Ok(ParsedLibraryPath { dir, libname })
}
