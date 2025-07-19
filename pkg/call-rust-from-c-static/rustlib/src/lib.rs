#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern "C" fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}
