use std::os::raw::c_char;
use std::ffi::{CStr, CString};

extern crate msvc_demangler;

#[no_mangle]
pub extern "C" fn demangle(symbol: *const c_char, flags: u32) -> *const c_char {
    let symbol_str = unsafe { CStr::from_ptr(symbol) }
        .to_str()
        .ok();
    let demangle_flags = msvc_demangler::DemangleFlags::from_bits(flags);

    if let (Some(symbol_str), Some(flags)) = (symbol_str, demangle_flags) {
        // Demangle the symbol using the msvc_demangler crate
        if let Ok(demangled) = msvc_demangler::demangle(symbol_str, flags) {
            return CString::new(demangled).unwrap().into_raw();
        }
    }
    // Return null pointer in case of any error
    std::ptr::null()
}

