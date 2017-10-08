use std::fmt::Display;

#[test]
fn generic_test() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    let result = largest2(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    let result = largest2(&numbers);
    println!("The largest number is {}", result);
}

#[test]
fn trait_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}

#[test]
fn lifetime_test() {
    //    &i32        // a reference
    //    &'a i32     // a reference with an explicit lifetime
    //    &'a mut i32 // a mutable reference with an explicit lifetime

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    //    {
    //        let string1 = String::from("long string is long");
    //        let result;
    //        {
    //            let string2 = String::from("xyz");
    //            result = longest(string1.as_str(), string2.as_str());
    //        }
    //        println!("The longest string is {}", result);
    //    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }

    //Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a function with two arguments gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.
    //If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
    //If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, then the lifetime of self is assigned to all output lifetime parameters. This makes writing methods much nicer.
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn notify<T: Summarizable + Display>(item: T) {
    println!("Breaking news! {}", item.summary());
}

fn some_function<T: Display + Clone, U: Clone + Display>(t: T, u: U) -> i32 {
    1
}

fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Display {
    1
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = list.first().unwrap();

    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        self.author.to_string()
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}