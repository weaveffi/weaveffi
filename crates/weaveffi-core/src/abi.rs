//! C ABI runtime: error struct, memory helpers, and utility functions.
#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Public opaque handle type exposed to foreign callers.
pub type weaveffi_handle_t = u64;

/// Error struct passed across the C ABI boundary.
///
/// Safety:
/// - `message` is a NUL-terminated UTF-8 C string allocated by Rust and must be
///   released by calling `weaveffi_error_clear` or `weaveffi_free_string`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct weaveffi_error {
    pub code: i32,
    pub message: *const c_char,
}

impl Default for weaveffi_error {
    fn default() -> Self {
        Self { code: 0, message: ptr::null() }
    }
}

/// Set the error to OK (code = 0) and free any prior message.
pub fn error_set_ok(out_err: *mut weaveffi_error) {
    if out_err.is_null() { return; }
    // SAFETY: Pointer checked for null above
    let err = unsafe { &mut *out_err };
    if !err.message.is_null() {
        // SAFETY: Message was allocated via `CString::into_raw` in this module
        unsafe { drop(CString::from_raw(err.message as *mut c_char)) };
    }
    err.code = 0;
    err.message = ptr::null();
}

/// Populate an error with the given code and message (copying message).
pub fn error_set(out_err: *mut weaveffi_error, code: i32, message: &str) {
    if out_err.is_null() { return; }
    // SAFETY: Pointer checked for null above
    let err = unsafe { &mut *out_err };
    if !err.message.is_null() {
        // SAFETY: Message was allocated via `CString::into_raw` in this module
        unsafe { drop(CString::from_raw(err.message as *mut c_char)) };
    }
    err.code = code;
    // `CString::new` fails if message contains interior NULs; replace them defensively
    let owned_message = message.replace('\0', "");
    let cstr = CString::new(owned_message).expect("CString::new sanitized input");
    err.message = cstr.into_raw();
}

/// Convenience adapter: map a `Result<T, E>` to `Option<T>` by writing into `out_err`.
pub fn result_to_out_err<T, E: std::fmt::Display>(result: Result<T, E>, out_err: *mut weaveffi_error) -> Option<T> {
    match result {
        Ok(value) => {
            error_set_ok(out_err);
            Some(value)
        }
        Err(e) => {
            // Default unspecified error code
            error_set(out_err, -1, &e.to_string());
            None
        }
    }
}

/// Allocate a new C string from a Rust string, returning an owned pointer.
/// Caller must later free with `weaveffi_free_string` or `weaveffi_error_clear`.
pub fn string_to_c_ptr(s: impl AsRef<str>) -> *const c_char {
    let s = s.as_ref();
    // Sanitize interior NULs which are invalid for C strings
    let sanitized = if s.as_bytes().contains(&0) { s.replace('\0', "") } else { s.to_owned() };
    let cstr = CString::new(sanitized).expect("string_to_c_ptr: unexpected NUL after sanitization");
    cstr.into_raw()
}

/// Free a C string previously allocated by this runtime.
pub fn free_string(ptr_: *const c_char) {
    if ptr_.is_null() { return; }
    // SAFETY: Pointer must be returned from `CString::into_raw`
    unsafe { drop(CString::from_raw(ptr_ as *mut c_char)) };
}

/// Free a byte buffer previously allocated by Rust and returned to foreign code.
/// The buffer must have been allocated as a single contiguous block and its
/// length passed back alongside the pointer.
pub fn free_bytes(ptr: *mut u8, len: usize) {
    if ptr.is_null() { return; }
    // SAFETY: This reconstructs the original Box<[u8]> for deallocation
    unsafe { drop(Box::from_raw(std::slice::from_raw_parts_mut(ptr, len))) };
}

/// Clear an error by freeing any message and zeroing fields.
pub fn error_clear(err: *mut weaveffi_error) { error_set_ok(err); }

/// Utility to borrow a `&str` from a NUL-terminated C string. Returns `None`
/// if `ptr` is null or not valid UTF-8.
pub fn c_ptr_to_str<'a>(ptr_: *const c_char) -> Option<&'a str> {
    if ptr_.is_null() { return None; }
    // SAFETY: caller guarantees `ptr_` points to a NUL-terminated string
    let c = unsafe { CStr::from_ptr(ptr_) };
    c.to_str().ok()
}
