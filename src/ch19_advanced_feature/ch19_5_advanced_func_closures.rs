fn add_one(x: i32) -> i32 { x + 1 }

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn func_pointer(){
    let answer = do_twice(add_one, 5);
    assert_eq!(12, answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}

//Fn doesn't implement Sized, so have to be boxed
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}