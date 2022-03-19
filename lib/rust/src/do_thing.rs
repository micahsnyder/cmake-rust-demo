/*
 * Example module that uses cbindgen generated bindings (sys.rs)
 *
 * Copyright (C) 2022 Micah Snyder.
 */

use crate::sys;

/// A rust function that may be called from C, and also itself calls into C
///
/// # Safety
///
/// inout must be a valid pointer to some mutable data of size inout_len
#[no_mangle]
pub unsafe extern "C" fn do_thing_with_call_into_c(inout: *mut u8, inout_len: usize) -> bool {
    sys::do_the_thing(inout, inout_len.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::do_thing_with_call_into_c;

    #[test]
    fn test_do_thing_with_call_into_c() {
        let x = &mut [1, 2, 3];

        let ret = unsafe { do_thing_with_call_into_c(x.as_mut_ptr(), x.len()) };

        assert_eq!(ret, true);

        assert_eq!(x, &[2, 4, 6]);
    }
}
