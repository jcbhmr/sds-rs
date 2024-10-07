#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn sdsnewlen() {
        // Rust view into a Rust-owned statically allocated string
        let expected = "He\0llo\0 wo\0\0rld!\0";
        // C-owned heap allocated string
        let sds = unsafe {
            crate::sdsnewlen(expected.as_ptr() as *const std::ffi::c_void, expected.len())
        };
        // ScopeGuard<*mut i8, impl FnOnce(...)> that implements Drop and Deref
        let sds = scopeguard::guard(sds, |sds| {
            unsafe {
                crate::sdsfree(sds);
            }
        });
        // Rust view into a C-owned heap allocated string
        let actual = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                *sds as *const u8,
                crate::sdslen(*sds),
            ))
        };
        assert_eq!(expected, actual);
    }
}
