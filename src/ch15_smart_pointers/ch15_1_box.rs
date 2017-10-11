#[test]
fn test_box() {
    //Boxes allow you to put a single value on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

//This is the main area where boxes are useful: breaking up an infinite data structure so that the compiler can know what size it is.
//another is trait objects
#[test]
fn test_cons() {
    use self::List::{Cons, Nil};
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

use std::ops::Deref;

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

#[test]
fn test_deref() {
    let mut x = 5;
    {
        let y = &mut x;

        *y += 1
    }

    assert_eq!(6, x);

    let my_fav_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen Spirit")),
    };

    assert_eq!(vec![1, 2, 3], *my_fav_song);

    //deref coercion
    //A deref coercion will automatically convert a reference to any pointer into a reference to that pointer's contents.
    //A deref coercion happens when the reference type of the argument passed into the function differs from the reference type of the parameter defined in that function's signature.

    //From &T to &U when T: Deref<Target=U>.
    //From &mut T to &mut U when T: DerefMut<Target=U>.
    //From &mut T to &U when T: Deref<Target=U>.
    fn compress_mp3(audio: &[u8]) -> Vec<u8> { vec![1, 3] }

    compress_mp3(my_fav_song.audio.as_slice());
    compress_mp3(&my_fav_song);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

#[test]
fn test_drop(){
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
//    drop(c);
    println!("Wait for it...");
}