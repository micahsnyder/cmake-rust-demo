/*
 * Example static lib for use in project apps
 *
 * Copyright (C) 2020 Micah Snyder.
 */

#![feature(c_variadic)]

use colored::*;

enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

fn clog(level: LogLevel, message: &str) -> () {
    match level {
        LogLevel::Debug => println!("Debug: {}", message.green()),
        LogLevel::Info => println!("{}", message),
        LogLevel::Warning => println!("Warning: {}", message.yellow()),
        LogLevel::Error => println!("ERROR: {}", message.bright_red()),
    };
}

unsafe fn from_c_string<'t>(string: *const u8, string_len: usize) -> &'t str {
    let slice = std::slice::from_raw_parts(string, string_len);
    std::str::from_utf8(slice as &[u8]).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn clog_debug(message: *const u8, message_len: usize) -> () {
    clog(LogLevel::Debug, from_c_string(message, message_len));
}

#[no_mangle]
pub unsafe extern "C" fn clog_info(message: *const u8, message_len: usize) -> () {
    clog(LogLevel::Info, from_c_string(message, message_len));
}

#[no_mangle]
pub unsafe extern "C" fn clog_warning(message: *const u8, message_len: usize) -> () {
    clog(LogLevel::Warning, from_c_string(message, message_len));
}

#[no_mangle]
pub unsafe extern "C" fn clog_error(message: *const u8, message_len: usize) -> () {
    clog(LogLevel::Error, from_c_string(message, message_len));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
