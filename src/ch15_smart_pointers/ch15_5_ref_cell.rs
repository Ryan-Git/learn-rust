//Interior mutability is a design pattern in Rust for allowing you to mutate data even though there are immutable references to that data, which would normally be disallowed by the borrowing rules.
// The interior mutability pattern is used when you can ensure that the borrowing rules will be followed at runtime, even though the compiler can't ensure that. The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32){
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32){
    *b += 1;
}

fn demo(r: &RefCell<i32>){
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}

#[test]
fn test_refcell(){
    {
        let data = RefCell::new(5);
        demo(&data);
    }

    {
        let s = RefCell::new(String::from("hello"));

        let r1 = s.borrow_mut();
//        let r2 = s.borrow_mut(); //panic at runtime
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::rc::Rc;

#[test]
fn test_rc_refcell(){
    let value = Rc::new(RefCell::new(5));

    let a = Cons(value.clone(), Rc::new(Nil));
    let shared_list = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    *value.borrow_mut() += 10;

    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}