#[test]
fn vector() {

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    {
        let v: Vec<i32> = Vec::new();
        let v = vec![1, 2, 3];

        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("{:?}", v);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);
    }

    //Invalid References
    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        //Adding a new element onto the end of the vector might require allocating new memory and copying the old elements over to the new space, in the circumstance that there isn’t enough room to put all the elements next to each other where the vector was. In that case, the reference to the first element would be pointing to deallocated memory.
//        v.push(6);
    }

    //vector with enum
    {
        let row = vec![
            Cell::Int(3),
            Cell::Text(String::from("blue")),
            Cell::Float(10.12),
        ];
    }
}

#[test]
fn string(){

    let s = String::from("initial contents");
    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
    //fn add(self, s: &str) -> String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}


#[test]
fn hash_maps(){
    use std::collections::HashMap;

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        {
            let team_name = String::from("Blue");
            let score = scores.get(&team_name);
        }

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    }

    {
        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}