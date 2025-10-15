#[no_mangle]
pub extern "C" fn weaveffi_calculator_add(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "C" fn weaveffi_calculator_mul(a: i32, b: i32) -> i32 { a * b }

#[no_mangle]
pub extern "C" fn weaveffi_calculator_echo(ptr: *const u8, len: usize) -> *mut u8 {
    // Safety contract: ptr must be valid UTF-8 bytes of length len
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let s = String::from_utf8_lossy(slice).into_owned();
    let mut bytes = s.into_bytes();
    bytes.push(0); // NUL terminate
    let mut boxed = bytes.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}
