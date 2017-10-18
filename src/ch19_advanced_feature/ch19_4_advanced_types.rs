type Kilometers = i32;
type Thunk = Box<FnOnce() + Send + 'static>;

use std::io;

//type Result<T> = io::Result<T, io::Error>;

//never type
fn bar() -> ! {
    panic!();
    //continue;
    //panic!
    //loop
}

//unsized type
//str, trait... hide behind a pointer
fn generic<T>(t: T){}
fn generic2<T: Sized>(t: T) {}
fn generic3<T: ?Sized>(t: &T) {}