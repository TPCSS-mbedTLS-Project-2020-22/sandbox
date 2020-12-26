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
    let _y = unsafe {  func_in_c(x);  };      // '_'  here says y wont be used.  avoids the warning
}



fn main() {
    call_c_func();

}

