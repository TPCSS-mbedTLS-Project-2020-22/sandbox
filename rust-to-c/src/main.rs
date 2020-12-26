// extern crate libc;
// use libc::size_t;
use libc::c_int;


/************************* declare the external c function ********************/
// this can be considered as a replacement of   #include "test-r2c.h"
#[link(name = "test-r2c")]
extern {
    fn func_in_c(x: c_int) -> c_int;
}




/*********** a function that calls the externally defined c function *******/
fn call_c_func() {

    let x = 1234;
    let _y = unsafe {  func_in_c(x)  };      // '_'  here says y wont be used.  avoids the warning
}

/************* another style would be to define a wrapper function for each of the external c function ********/
// it might be a good idea to have a wrapper for each of the external c functions
fn wrapper_func_in_c(x: i32) -> i32 {
    let y = unsafe {  func_in_c(x)  };
    return y;
}


fn main() {
    call_c_func();

    let z = wrapper_func_in_c(1000);
    println!("calling C through the wrapper func:  {0}", z);
}

