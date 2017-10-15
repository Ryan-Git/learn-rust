use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;


#[test]
fn test_mutex_api() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m={:?}", m);
}

#[test]
fn test_mutex() {
    //    let counter = Mutex::new(0);
    //    let mut handles = vec![];
    //
    //    for _ in 0..10 {
    //        //can't compile, borrow
    //        //borrow can't compile, multiple ownership
    //        let handle = thread::spawn(|| {
    //            let mut num = counter.lock().unwrap();
    //            *num += 1;
    //        });
    //        handles.push(handle);
    //    }
    //
    //    for h in handles {
    //        h.join().unwrap();
    //    }
    //
    //    println!("Result: {}", *counter.lock().unwrap())
}

#[test]
fn test_mutex_rc() {
//    let counter = Rc::new(Mutex::new(0));
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        //`std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for h in handles {
//        h.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap())
}

#[test]
fn test_mutex_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}