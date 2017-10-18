//Dereferencing a raw pointer
//Calling an unsafe function or method
//Accessing or modifying a mutable static variable
//Implementing an unsafe trait
#[test]
fn test_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

#[test]
fn test_unsafe_func() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

#[test]
fn create_safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    //(&mut slice[..mid], &mut slice[mid..]) //can't borrow mut twice!

    let ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}

//#[test]
//fn extern_fn() {
//    unsafe {
//        some_function();
//    }
//}
//
//extern "C" {
//    fn some_function();
//}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

#[test]
fn test_static_var() {
    println!("name is: {}", HELLO_WORLD);

    unsafe {
        COUNTER += 1;
    }

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {

}