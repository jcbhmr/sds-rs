use sds_sys::{
    sds, sdsAllocPtr, sdsAllocSize, sdsIncrLen, sdsMakeRoomFor, sdsRemoveFreeSpace, sdsResize,
    sds_free, sds_malloc, sds_realloc, sdsalloc, sdsavail, sdscat, sdscatfmt, sdscatlen,
    sdscatprintf, sdscatrepr, sdscatsds, sdscatvprintf, sdsclear, sdscmp, sdscpy, sdscpylen,
    sdsdup, sdsempty, sdsfree, sdsfreesplitres, sdsfromlonglong, sdsgrowzero, sdsinclen, sdsjoin,
    sdsjoinsds, sdslen, sdsmapchars, sdsneedsrepr, sdsnew, sdsnewlen, sdsrange, sdssetalloc,
    sdssetlen, sdssplitargs, sdssplitlen, sdssubstr, sdstolower, sdstoupper, sdstrim, sdsupdatelen,
};
use std::ffi::{c_char, c_void, CStr, CString, OsStr, OsString};

/// A Rust-owned C-heap allocated SDS string. [SdsBuf] is to &[Sds] as [String]
/// is to &[str]. This type is intended to be used to facilitate interop between
/// Rust and C code.
///
/// # The problem with C strings
///
/// The Rust [CString] and related types are designed to facilitate the usage of
/// nul-termianted C strings in Rust. This works well right up until you need
/// to deal with binary data (most likely from the filesystem). nul-terminated
/// strings cannot have `\0` characters in them.
///
/// A good practical example is WebAssembly (`.wasm`) files. These files always
/// start with the magic numbers `\0asm` which computes as `strlen("\0asm") ==
/// 0`. This means that if you try to pass the `wasm_data` from Rust to C or
/// vice versa to do _something_ to it without an extra `len` argument or field
/// it won't work.
///
/// # A solution: SDS strings
///
/// From the C side, here's what an SDS string looks like:
///
/// ```c
/// +--------+-------------------------------+-----------+
/// | Header | Binary safe C alike string... | Null term |
/// +--------+-------------------------------+-----------+
///          |
///          `-> Pointer returned to the user.
///
/// sds = char*
/// ```
///
/// This trick where the header is prefixed to the string but the pointer
/// returned to the user is the start of the string data has some advantages and
/// disadvantages.
///
/// <table>
/// <thead><th>Advantages <th>Disadvantages
/// <tbody><td>
///
/// - The `char*` pointer can be used with existing C-string functions like
///   `printf()`. The string data is still nul-terminated for C compatibility.
/// - Acessing individual characters is easy: `sds[12]`.
///
/// <td>
///
/// - You can't `free()` the returned `char*` pointer directly. You must use
///   `sdsfree()`.
/// - No automatic internal reallocations if the reserved capacity is exceeded.
///   You must `s = sdscat(s, "foo")`.
///
/// </table>
///
/// # Example
///
/// ```
/// # use std::ffi::{CString, CStr, OsString, OsStr, c_char, c_void};
/// # use sds::{SdsBuf, Sds, c_sds};
/// #[no_mangle]
/// pub extern "C" fn mylib_data() -> c_sds {
///     // 1. Allocate a new SDS string from some data.
///     let data = Sds::from("He\0llo A\0lan Turin\0g!");
///     // 2. Consume & transfer ownership to C.
///     data.into_raw()
/// }
/// ```
///
/// ```c
/// // 1. Aquire ownership of the SDS string from Rust.
/// sds data = mylib_data();
/// // 2. Use the SDS string. Note how we get the length.
/// printf("%.*s\n", sdslen(data), data);
/// // 3. Free the SDS string from C.
/// sdsfree(data);
/// ```
pub struct Sds(sds_sys::sds);

impl Sds {
    /// Returns the contents of this [Sds] string as a slice of bytes.
    ///
    /// The returned slice does **not** contain the trailing nul terminator. The
    /// string may contain nul bytes as part of its contents. If you want the
    /// C-style string with the appended nul terminator (even if there are nul
    /// bytes in the string itself), use [Sds::as_bytes_with_nul] instead.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0 as *const u8, sds_sys::sdslen(self.0)) }
    }

    /// Equivalent to [Sds::as_bytes] but includes the trailing nul terminator.
    ///
    /// The returned slice may contain other nul bytes as part of the string.
    /// It's recommended to use another channel to communicate the length of the
    /// string instead of relying on the nul terminator to mark the end due to
    /// the possibility of nul bytes in the string body.
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0 as *const u8, sds_sys::sdslen(self.0) + 1) }
    }

    /// Extracts a &[CStr] slice containing the contents of this [Sds] string up
    /// until the first nul byte.
    ///
    /// The returned &[CStr] may not be the entire string if the string body
    /// contains nul bytes. Prefer [Sds::as_os_str] or [Sds::to_str].
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    /// Retakes ownership of a [Sds] string that was transferred to C via
    /// [Sds::into_raw].
    ///
    /// # Safety
    ///
    /// This should only ever be called with a pointer that was earlier obtained
    /// by calling [Sds::into_raw]. Other usage (e.g., trying to take ownership
    /// of an SDS string that was allocated by foreign code) may result in
    /// undefined behavior or allocator corruption.
    /// 
    /// > **Note:** If you need to borrow an SDS string that was allocated by foreign code, use 
    pub unsafe fn from_raw(ptr: *mut i8) -> Self {
        Self(ptr)
    }

    pub fn into_raw(self) -> *mut i8 {
        let ptr = self.0;
        std::mem::forget(self);
        ptr
    }

    pub fn new() -> Self {
        Self(unsafe { sds_sys::sdsempty() })
    }

    pub fn as_os_str(&self) -> &std::ffi::OsStr {
        unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(self.as_bytes()) }
    }

    pub fn capacity(&self) -> usize {
        unsafe { sds_sys::sdsalloc(self.0) }
    }

    pub fn clear(&mut self) {
        unsafe {
            sds_sys::sdsclear(self.0);
        }
    }
}

impl Drop for Sds {
    fn drop(&mut self) {
        unsafe {
            sds_sys::sdsfree(self.0);
        }
    }
}

impl std::ops::Deref for Sds {
    type Target = std::ffi::OsStr;

    fn deref(&self) -> &Self::Target {
        self.as_os_str()
    }
}
