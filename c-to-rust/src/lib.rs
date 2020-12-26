
#[no_mangle]
// pub unsafe extern "C" fn func_in_rust(x: i32) -> i32 {
pub extern "C" fn func_in_rust(x: i32) -> i32 {
    return x + 10;
}

