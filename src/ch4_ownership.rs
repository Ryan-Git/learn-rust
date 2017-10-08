//Ownership Rules
//Each value in Rust has a variable that’s called its owner.
//There can only be one owner at a time.
//When the owner goes out of scope, the value will be dropped.

//The Rules of References
//Let’s recap what we’ve discussed about references:
//
//At any given time, you can have either but not both of:
//  One mutable reference.
//  Any number of immutable references.
//References must always be valid.
#[test]
fn main() {
    //Ways Variables and Data Interact: Move
    {
        let s1 = String::from("hello");

        // this is a move, because String doesn't implement the Copy trait
        let s2 = s1;

        //use of moved value, can't compile
//        println!("{}, world!", s1);
    }


    //Ways Variables and Data Interact: Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }


    //Ownership and Functions
    {
        let s = String::from("hello");  // s comes into scope.
        takes_ownership(s);  // s's value moves into the function...
        // ... and so is no longer valid here.
    }// Here, since s's value was moved, nothing special happens.


    //Return Values and Scope
    {
        let s1 = gives_ownership();         // gives_ownership moves its return
        // value into s1.

        let s2 = String::from("hello");     // s2 comes into scope.

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3.
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.


    //References and Borrowing
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }


    //Mutable References
    // you can only have one mutable reference to
    // a particular piece of data in a particular scope
    //We also cannot have a mutable reference while we have an immutable one
    {
        let mut s = String::from("hello");

        change(&mut s);

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        {
            let r2 = &mut s;
        }

        {
            let r1 = &s; // no problem
            let r2 = &s; // no problem
            //        let r3 = &mut s; // BIG PROBLEM
        }
    }


    //Dangling References
//    {
//        let reference_to_nothing = dangle();
//    }


    //Slice
    //A string slice(&str) is a reference to the starting position
    //and the length of the slice
    //String Literals Are Slices
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
    // moves out to the calling
    // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope.

    a_string  // a_string is returned and moves out to the calling function.
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {
    // dangle returns a reference to a String

//    let s = String::from("hello");// s is a new String

//    &s// we return a reference to the String, s
//}// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!