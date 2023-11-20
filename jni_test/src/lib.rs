use std::any::Any;

#[no_mangle]
pub extern fn Java_tests_Main_rust_1test(_env: *mut dyn Any, _jclass: *mut dyn Any) {
    println!("Hello from rust!");
}