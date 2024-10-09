//! # Simple Dynamic Strings
//!
//!

pub use sds_sys::sds as c_sds;
use sds_sys::{
    sdsAllocPtr, sdsAllocSize, sdsIncrLen, sdsMakeRoomFor, sdsRemoveFreeSpace, sdsResize, sds_free,
    sds_malloc, sds_realloc, sdsalloc, sdsavail, sdscat, sdscatfmt, sdscatlen, sdscatprintf,
    sdscatrepr, sdscatsds, sdscatvprintf, sdsclear, sdscmp, sdscpy, sdscpylen, sdsdup, sdsempty,
    sdsfree, sdsfreesplitres, sdsfromlonglong, sdsgrowzero, sdsinclen, sdsjoin, sdsjoinsds, sdslen,
    sdsmapchars, sdsneedsrepr, sdsnew, sdsnewlen, sdsrange, sdssetalloc, sdssetlen, sdssplitargs,
    sdssplitlen, sdssubstr, sdstolower, sdstoupper, sdstrim, sdsupdatelen,
};
use std::{
    borrow::{Borrow, Cow},
    ffi::{
        c_char, c_void, CStr, CString, FromBytesUntilNulError, FromBytesWithNulError, OsStr,
        OsString,
    },
    fmt::{Debug, Display},
    mem::ManuallyDrop,
    ops::{Deref, Index, RangeFrom},
    str::Utf8Error,
    sync::Arc,
};

/// Representation of a borrowed C SDS string.
///
/// This type represents a borrowed reference to a length prefixed array of
/// bytes. It can be constructed safely from a <code>&[[u8]]</code> slice or
/// unsafely from a raw valid **SDS-created** [`c_sds`] (alias for <code>*const [c_char]</code>).
///
/// The [`SdsStr`] can then be converted to a Rust <code>&[str]</code> by performing UTF-8 validation, or into an owned [`SdsString`].
///
/// <code>&SdsStr</code> is to [`SdsString`] as <code>&[str]</code> is to <code>String</code>: the former in each pair are borrowed references; the later are owned strings.
///
/// # Examples
///
/// Exposing a Rust function to C that borrows an SDS string:
/// ```
/// # use sds::{SdsStr, c_sds};
/// # use std::ffi::c_int;
///
/// pub extern "C" fn count_nuls(s: c_sds) -> c_int {
///    let s = unsafe { SdsStr::from_raw(s) };
///    s.iter().filter(|&&c| c == b'\0').count() as c_int;
/// }
/// ```
#[derive(Hash)]
#[cfg_attr(not(doc), repr(transparent))]
pub struct SdsStr(c_sds);

impl SdsStr {
    pub const unsafe fn from_ptr<'a>(ptr: c_sds) -> &'a Self {}

    pub const fn from_bytes_until_nul(bytes: &[u8]) -> Result<&Self, FromBytesUntilNulError> {}

    pub const fn from_bytes_with_nul(bytes: &[u8]) -> Result<&Self, FromBytesWithNulError> {}

    pub const unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &Self {}

    pub const fn as_ptr(&self) -> c_sds {}

    pub const fn len(&self) -> usize {}

    pub const fn is_empty(&self) -> bool {}

    pub const fn as_bytes(&self) -> &[u8] {}

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {}

    pub const fn as_bytes_with_nul(&self) -> &[u8] {}

    pub fn as_bytes_with_nul_mut(&mut self) -> &mut [u8] {}

    pub const fn to_str(&self) -> Result<&str, Utf8Error> {}

    pub fn to_string_lossy(&self) -> Cow<'_, str> {}

    pub fn into_sds_string(self: Box<Self>) -> SdsString {}
}

impl AsRef<SdsStr> for SdsStr {
    fn as_ref(&self) -> &SdsStr {
        self
    }
}

impl AsRef<SdsStr> for SdsString {
    fn as_ref(&self) -> &SdsStr {
        self
    }
}

impl Borrow<SdsStr> for SdsString {
    fn borrow(&self) -> &SdsStr {
        self
    }
}

impl Clone for Box<SdsStr> {
    fn clone(&self) -> Self {}
}

impl Debug for SdsStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_bytes().escape_ascii())
    }
}

impl Default for &SdsStr {
    fn default() -> Self {
        thread_local! {
            static EMPTY: c_sds = unsafe { sdsempty() };
        }
    }
}

impl Default for Box<SdsStr> {
    fn default() -> Self {}
}

impl From<&SdsStr> for Arc<SdsStr> {
    fn from(value: &SdsStr) -> Self {}
}

impl From<&SdsStr> for Box<SdsStr> {
    fn from(value: &SdsStr) -> Self {}
}

impl From<&SdsStr> for SdsString {
    fn from(value: &SdsStr) -> Self {}
}

impl<'a> From<&'a SdsStr> for Cow<'a, SdsStr> {
    fn from(value: &'a SdsStr) -> Self {}
}

impl From<&SdsStr> for Rc<SdsStr> {
    fn from(value: &SdsStr) -> Self {}
}

impl From<SdsString> for Box<SdsStr> {
    fn from(value: SdsString) -> Self {}
}

impl From<Cow<'_, SdsStr>> for Box<SdsStr> {
    fn from(value: Cow<'_, SdsStr>) -> Self {}
}

impl Index<RangeFrom<usize>> for SdsStr {
    type Output = SdsStr;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {}
}

impl Ord for SdsStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {}
}

impl PartialEq for SdsStr {
    fn eq(&self, other: &Self) -> bool {}
}

impl PartialOrd for SdsStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {}
}

impl ToOwned for SdsStr {
    type Owned = SdsString;

    fn to_owned(&self) -> Self::Owned {}

    fn clone_into(&self, target: &mut Self::Owned) {}
}

impl Eq for SdsStr {}

/// A type representing an owned, C-compatible, nul-terminated SDS string with
/// potential nul bytes in the middle.
///
/// This type serves the purpose of being able to safely generate an SDS string
/// from a Rust byte slice or vector. An instance of this type **is not** a
/// static guarentee that the underlying bytes contain no interior 0 bytes ("nul
/// characters"). An instance of this type **is** a static guarentee that the
/// final byte is 0 ("nul terminator"). SDS strings have an associated length
/// stored before the raw pointer that underlies this type.
///
/// [`SdsString`] is to <code>&[[SdsStr]]</code> as [`String`] is to
/// <code>&[[str]]</code>: the former in each pair are owned strings; the later
/// are borrowed references.
///
/// # Creating an [`SdsString`]
///
/// An [`SdsString`] is created from either a byte slice or a byte vector, or
/// anything that implements <code>[Into]<[Vec]<[u8]>></code> (for example, you
/// can build an [`SdsString`] straight out of a [`String`] or a
/// <code>&[str]</code>, since both implement that trait).
///
/// # Extracting a raw pointer to the whole SDS string
///
/// [`SdsString`] implements an [`as_ptr`][SdsStr::as_ptr] method through the
/// [`Deref`] trait. This method will give you a <code>*const [c_char]</code>
/// which you can feed directly to extern functions that expect an SDS string or
/// a nul-terminated string, like C's `strdup()`. Notice that
/// [`as_ptr`][SdsStr::as_ptr] returns a read-only pointer; if the C code writes
/// to it, that causes undefined behavior.
///
/// # Extracting a slice of the whole SDS string
///
/// Alternatively, you can obtain a <code>&[[u8]]</code> slice from an
/// [`SdsString`] with the [`SdsString::as_bytes`] method. Slices produced in
/// this way do _not_ contain the trailing nul terminator. This is useful when
/// you will be calling an extern function that takes a `*const c_char` function
/// which is not necessarily nul-terminated, plus another argument with the
/// length of the string &mdash; like C's `strndup()`. You can of course get the
/// slice's length with its `len` method.
///
/// If you need a <code>&[[u8]]</code> slice _with_ the nul terminator, you can
/// use [`SdsString::as_bytes_with_nul`] instead.
///
/// Once you have the kind of slice you need (with or without the nul
/// terminator), you can pass the slice's own `as_ptr` method to get a read-only
/// raw pointer to pass to extern functions. See the documentation for that
/// function for a discussion on ensuring the lifetime of a raw pointer.
///
/// # Examples
///
/// ```ignore (extern-declaration)
/// use sds::SdsString;
/// use std::ffi::c_char;
///
/// extern "C" {
///     fn my_printer(s: *const c_char);
/// }
///
/// let sds_to_print = SdsString::new("Hello, world!");
/// unsafe {
///     my_printer(sds_to_print.as_ptr());
/// }
/// ```
///
/// # Safety
///
/// [`SdsString`] is intended for working with SDS (and traditional C-style)
/// strings; the primary use case for these kinds of strings is interoperating
/// with C-like code. Often you will need to transfer ownership to/from that
/// external code. It is strongly recommended that you thoroughly read through
/// the documentation of [`SdsString`] before use, as improper ownership
/// management of [`SdsString`] instances can lead to invalid memory accesses,
/// memory leaks, and other memory errors.
pub struct SdsString(c_sds);

impl SdsString {
    /// Creates a new SDS string from a container of bytes.
    ///
    /// This function will consume the provided data and use [`sdsnewlen`] to to
    /// construct a new string, ensuring that there is a trailing 0 byte. This
    /// trailing 0 byte will be appended by this function; the provided data
    /// should _not_ contain a trailing 0 byte. The probided data may contain
    /// interior 0 bytes.
    ///
    /// # Examples
    ///
    /// ```ignore (extern-declaration)
    /// use sds::SdsString;
    /// use std::ffi::c_char;
    ///
    /// extern "C" {
    ///     fn puts(s: *const c_char);
    /// }
    ///
    /// let to_print = SdsString::new("Hello!");
    /// unsafe {
    ///    puts(to_print.as_ptr());
    /// }
    /// ```
    pub fn new(t: impl Into<Vec<u8>>) -> Self {
        let bytes = t.into();
        Self(unsafe { sdsnewlen(bytes.as_ptr() as *const c_void, bytes.len()) })
    }

    /// Retakes ownership of an [`SdsString`] that was transferred to C via
    /// [`SdsString::into_raw`].
    ///
    /// # Safety
    ///
    /// This should only ever be called with a pointer that was earlier obtained
    /// by calling [`SdsString::into_raw`]. Other usage (e.g. trying to take
    /// ownership of a string that was allocated by foreign code) is likely to
    /// lead to undefined behavior or allocator corruption.
    ///
    /// > **Note:** If you need to borrow a string that was allocated by foreign
    /// > code, use [`SdsStr`]. If you need to take ownership of a string that
    /// > was allocated by foreign code, you will need to make your own
    /// > previsions for freeing it appropriately, like with the foreign code's
    /// > API to do that.
    ///
    /// # Examples
    ///
    /// Creates an [`SdsString`], pass ownership to an `extern` function (via raw pointer), then retake ownership with `from_raw`:
    ///
    /// ```ignore (extern-declaration)
    /// use sds::SdsString;
    /// use std::ffi::c_char;
    ///
    /// extern "C" {
    ///     fn some_extern_function(s: *mut c_char);
    /// }
    ///
    /// let sds_string = SdsStrong::new("Hello!");
    /// let raw = c_string.into_raw();
    /// unsafe {
    ///     some_extern_function(raw);
    ///     let sds_string = SdsString::from_raw(raw);
    /// }
    /// ```
    pub unsafe fn from_raw(ptr: *mut c_char) -> Self {
        Self(ptr)
    }

    /// Consumes the [`SdsString`] and transfers ownership of the string to a C
    /// caller.
    ///
    /// The pointer which function returns must be returned to Rust and
    /// reconstituted using [`SdsString::from_raw`] to be properly deallocated.
    /// Specifically, one should _not_ use the standard C `free()` function to
    /// deallocate this string.
    ///
    /// Failure to call [`SdsString::from_raw`] will lead to a memory leak.
    ///
    /// The C side must **not** modify the length of the string before it makes
    /// it back into Rust using [`SdsString::from_raw`]. See the safety section
    /// in [`SdsString::from_raw`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sds_string = SdsString::new("foo");
    ///
    /// let ptr = sds_string.into_raw();
    ///
    /// unsafe {
    ///     assert_eq!(b'f', *ptr as u8);
    ///     assert_eq!(b'o', *ptr.add(1) as u8);
    ///     assert_eq!(b'o', *ptr.add(2) as u8);
    ///     assert_eq!(b'\0', *ptr.add(3) as u8);
    ///
    ///     // retake pointer to free memory
    ///     let _ = SdsString::from_raw(ptr);
    /// }
    /// ```
    pub fn into_raw(self) -> *mut c_char {
        ManuallyDrop::new(self).0
    }

    /// Converts the [`SdsString`] into a [`String`] if it contains valid UTF-8 data.
    ///
    /// On failure, ownership of the original [`SdsString`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sdsstring = SdsString::new(b"foo");
    /// assert_eq!(sdsstring.into_string().expect("into_string() call failed"), "foo");
    ///
    /// let sdsstring = SdsString::new(b"f\xffoo");
    /// let err = sdsstring.into_string().err().expect("into_string().err() failed");
    /// assert_eq!(err.utf8_error().valid_up_to(), 1);
    /// ```
    pub fn into_string(self) -> Result<String, IntoStringError> {
        String::from_utf8(self.into_bytes()).map_err(|e| IntoStringError {
            error: e.utf8_error(),
            inner: Self::new(e.into_bytes()),
        })
    }

    /// Consumes the [`SdsString`] and returns the copied byte buffer.
    ///
    /// The returned buffer does **not** contain the trailing nul terminator, and it may have interior nul bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sds_string = SdsString::new("foo");
    /// let bytes = sds_string.into_bytes();
    /// assert_eq!(bytes, b"foo".to_vec());
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        let len = unsafe { sdslen(self.0) };
        let mut bytes = Vec::with_capacity(len);
        unsafe {
            bytes.set_len(len);
            std::ptr::copy_nonoverlapping(self.0 as *const u8, bytes.as_mut_ptr(), len);
        }
        bytes
    }

    /// Equivalent to [`SdsString::into_bytes`] except that the returned vector includes the trailing nul terminator.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sds_string = SdsString::new("foo");
    /// let bytes = sds_string.into_bytes_with_nul();
    /// assert_eq!(bytes, b"foo\0".to_vec());
    /// ```
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        let len = unsafe { sdslen(self.0) + 1 };
        let mut bytes = Vec::with_capacity(len);
        unsafe {
            bytes.set_len(len);
            std::ptr::copy_nonoverlapping(self.0 as *const u8, bytes.as_mut_ptr(), len);
        }
        bytes
    }

    /// Returns the contents of this [`SdsString`] as a slice of bytes.
    ///
    /// The returned slice does **not** contain the trailing nul terminator, and may have interior nul bytes. If you need the nul terminator, use [`SdsString::as_bytes_with_nul`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sds_string = SdsString::new("foo");
    /// let bytes = sds_string.as_bytes();
    /// assert_eq!(bytes, b"foo".as_slice());
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0 as *const u8, sdslen(self.0)) }
    }

    /// Equivalent to [`SdsString::as_bytes`] except that the returned slice includes the trailing nul terminator.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    ///
    /// let sds_string = SdsString::new("foo");
    /// let bytes = sds_string.as_bytes_with_nul();
    /// assert_eq!(bytes, b"foo\0".as_slice());
    /// ```
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0 as *const u8, sdslen(self.0) + 1) }
    }

    /// Extracts a [`CStr`] slice containing the contents of this [`SdsString`] up until the first nul byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use sds::SdsString;
    /// use std::ffi::CStr;
    ///
    /// let sds_string = SdsString::new(b"foo\0bar");
    /// let cstr = sds_string.as_c_str();
    /// assert_eq!(cstr, c"foo");
    /// ```
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    /// Converts a <code>[Vec]<[u8]></code> to an [`SdsString`] without checking the invariants on the given [`Vec`].
    ///
    /// # Safety
    ///
    /// The given [`Vec`] **must** have one nul byte as its last element. It may contain interior nul bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use sds::SdsString;
    /// assert_eq!(
    ///     unsafe { SdsString::from_vec_with_nul_unchecked(b"foo\0bar\0".to_vec()) },
    ///     SdsString::new("foo\0bar")
    /// )
    /// ```
    pub unsafe fn from_vec_with_nul_unchecked(v: Vec<u8>) -> Self {
        let v = ManuallyDrop::new(v);
        Self(sdsnewlen(v.as_ptr() as *const c_void, v.len()))
    }

    pub fn from_vec_with_nul(v: Vec<u8>) -> Result<Self, ()> {
        todo!()
    }
}

impl Deref for SdsString {
    type Target = SdsStr;

    fn deref(&self) -> &Self::Target {}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntoStringError {
    inner: SdsString,
    error: Utf8Error,
}

impl IntoStringError {
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_sdsstring(self) -> SdsString {
        self.inner
    }

    #[must_use]
    pub fn utf8_error(&self) -> Utf8Error {
        self.error
    }

    fn description(&self) -> &str {
        "SDS string contained non-utf8 bytes"
    }
}

impl Display for IntoStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.description(), f)
    }
}
