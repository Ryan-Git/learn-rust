use std::thread;

#[test]
fn test_thread(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join();
}

#[test]
fn test_move_closures(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move ||{
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}