/*
 * Example static lib for use in project apps
 *
 * Copyright (C) 2020 Micah Snyder.
 */

use libc::c_char;
use std::ffi::CString;

use uuid::Uuid;

#[no_mangle]
pub extern "C" fn gen_uuid() -> *mut c_char  {
    let uuid_str = Uuid::new_v4().to_string();

    let c_uuid = CString::new(uuid_str).unwrap();
    c_uuid.into_raw()
}

#[no_mangle]
pub extern  "C" fn free_uuid(uuid_ptr: *mut c_char ) -> () {
    unsafe {
        if uuid_ptr.is_null() {
            return;
        }
        CString::from_raw(uuid_ptr)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
