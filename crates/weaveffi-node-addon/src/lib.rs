use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use once_cell::sync::OnceCell;
use libloading::{Library, Symbol};

type AddFn = unsafe extern "C" fn(i32, i32, *mut WeaveError) -> i32;
type MulFn = unsafe extern "C" fn(i32, i32, *mut WeaveError) -> i32;
type DivFn = unsafe extern "C" fn(i32, i32, *mut WeaveError) -> i32;
type EchoFn = unsafe extern "C" fn(*const u8, usize, *mut WeaveError) -> *const c_char;
type FreeStringFn = unsafe extern "C" fn(*const c_char);
type ErrorClearFn = unsafe extern "C" fn(*mut WeaveError);

struct FfiApi {
  add: AddFn,
  mul: MulFn,
  div: DivFn,
  echo: EchoFn,
  free_string: FreeStringFn,
  error_clear: ErrorClearFn,
}

static API: OnceCell<(Library, FfiApi)> = OnceCell::new();

fn load_api() -> napi::Result<&'static (Library, FfiApi)> {
  API.get_or_try_init(|| {
    let path = std::env::var("WEAVEFFI_LIB").ok().unwrap_or_else(|| {
      // default to calculator debug dylib
      let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      p.pop(); p.pop(); // up to crates/
      p.push("target"); p.push("debug"); p.push("libcalculator.dylib");
      p.display().to_string()
    });
    let lib = unsafe { Library::new(&path) }.map_err(|e| Error::new(Status::GenericFailure, format!("load {}: {}", path, e)))?;
    // Extract symbols while keeping lib alive, then return both
    let api: FfiApi = unsafe {
      let add: Symbol<AddFn> = lib.get(b"weaveffi_calculator_add").map_err(map_err)?;
      let mul: Symbol<MulFn> = lib.get(b"weaveffi_calculator_mul").map_err(map_err)?;
      let div: Symbol<DivFn> = lib.get(b"weaveffi_calculator_div").map_err(map_err)?;
      let echo: Symbol<EchoFn> = lib.get(b"weaveffi_calculator_echo").map_err(map_err)?;
      let free_string: Symbol<FreeStringFn> = lib.get(b"weaveffi_free_string").map_err(map_err)?;
      let error_clear: Symbol<ErrorClearFn> = lib.get(b"weaveffi_error_clear").map_err(map_err)?;
      FfiApi { add: *add, mul: *mul, div: *div, echo: *echo, free_string: *free_string, error_clear: *error_clear }
    };
    Ok((lib, api))
  })
}

fn map_err(e: libloading::Error) -> Error { Error::new(Status::GenericFailure, e.to_string()) }

#[repr(C)]
#[derive(Clone, Copy)]
struct WeaveError { code: i32, message: *const c_char }

fn take_error(err: &mut WeaveError) -> Option<(i32, String)> {
  if err.code == 0 { return None; }
  let msg = if err.message.is_null() { String::new() } else {
    // SAFETY: message is NUL-terminated string from Rust side
    let s = unsafe { CStr::from_ptr(err.message) }.to_string_lossy().to_string();
    s
  };
  // SAFETY: clear frees message buffer
  if let Ok((_, api)) = load_api() { unsafe { (api.error_clear)(err as *mut WeaveError) }; }
  Some((err.code, msg))
}

#[napi]
pub fn add(a: i32, b: i32) -> napi::Result<i32> {
  let mut err = WeaveError { code: 0, message: std::ptr::null() };
  let (_, api) = load_api()?;
  let rv = unsafe { (api.add)(a, b, &mut err as *mut WeaveError) };
  if let Some((code, msg)) = take_error(&mut err) { return Err(Error::new(Status::GenericFailure, format!("({}) {}", code, msg))); }
  Ok(rv)
}

#[napi]
pub fn mul(a: i32, b: i32) -> napi::Result<i32> {
  let mut err = WeaveError { code: 0, message: std::ptr::null() };
  let (_, api) = load_api()?;
  let rv = unsafe { (api.mul)(a, b, &mut err as *mut WeaveError) };
  if let Some((code, msg)) = take_error(&mut err) { return Err(Error::new(Status::GenericFailure, format!("({}) {}", code, msg))); }
  Ok(rv)
}

#[napi]
pub fn div(a: i32, b: i32) -> napi::Result<i32> {
  let mut err = WeaveError { code: 0, message: std::ptr::null() };
  let (_, api) = load_api()?;
  let rv = unsafe { (api.div)(a, b, &mut err as *mut WeaveError) };
  if let Some((code, msg)) = take_error(&mut err) { return Err(Error::new(Status::GenericFailure, format!("({}) {}", code, msg))); }
  Ok(rv)
}

#[napi]
pub fn echo(s: String) -> napi::Result<String> {
  let mut err = WeaveError { code: 0, message: std::ptr::null() };
  let (_, api) = load_api()?;
  let bytes = s.into_bytes();
  let ptr = bytes.as_ptr();
  let len = bytes.len();
  // Call; rust side copies string when producing c string
  let c_ptr = unsafe { (api.echo)(ptr, len, &mut err as *mut WeaveError) };
  if let Some((code, msg)) = take_error(&mut err) { return Err(Error::new(Status::GenericFailure, format!("({}) {}", code, msg))); }
  if c_ptr.is_null() { return Err(Error::new(Status::GenericFailure, "null string".to_string())); }
  let out = unsafe { CStr::from_ptr(c_ptr) }.to_string_lossy().to_string();
  unsafe { (api.free_string)(c_ptr) };
  Ok(out)
}
