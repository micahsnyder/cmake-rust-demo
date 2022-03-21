/*
 * Example static lib for use in project apps
 *
 * Copyright (C) 2020-2022 Micah Snyder.
 */

use libc::c_char;
use std::ffi::CString;

use uuid::Uuid;

/// Generate / allocate a UUID structure
#[no_mangle]
pub extern "C" fn gen_uuid() -> *mut c_char {
    let uuid_str = Uuid::new_v4().to_string();

    let c_uuid = CString::new(uuid_str).unwrap();
    c_uuid.into_raw()
}

/// Free a UUID structure
///
/// # Safety
///
/// uuid_ptr must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn free_uuid(uuid_ptr: *mut c_char) {
    if uuid_ptr.is_null() {
        return;
    }
    let _ = CString::from_raw(uuid_ptr);
}

#[cfg(test)]
mod tests {
    /// faux test to demonstrate running rust unit tests through CMake / CTest
    #[test]
    fn test_gen_uuid() {
        assert_eq!(2 + 2, 4);
    }
}
