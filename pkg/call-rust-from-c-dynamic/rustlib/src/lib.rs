use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn greet(name: *const std::os::raw::c_char) {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = c_str.to_str().unwrap_or("Stranger");
    println!("Hello, {name_str}!");
}
