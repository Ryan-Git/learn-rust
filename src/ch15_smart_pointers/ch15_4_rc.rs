//In the majority of cases, ownership is very clear: you know exactly which variable owns a given value.
//However, this isn't always the case; sometimes, you may actually need multiple owners.

//Note that Rc<T> is only for use in single-threaded scenarios; the next chapter on concurrency will cover how to do reference counting in multithreaded programs. If you try to use Rc<T> with multiple threads, you'll get a compile-time error.

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use self::List::{Cons, Nil};
use std::rc::Rc;

#[test]
fn test_rc() {
    //Cloning an Rc<T> Increases the Reference Count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("rc = {}", Rc::strong_count(&a));

    let b = Cons(3, a.clone());
    println!("rc after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, a.clone());
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}