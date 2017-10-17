//encapsulation
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//trait objects
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw {:?}", self);
    }
}

#[test]
fn test_rust_gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };

    screen.run();
}

//Object Safety is Required for Trait Objects
//The trait does not require Self to be Sized
//All of the trait's methods are object safe.


//design patterns
#[test]
fn test_state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch ");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text("today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("another");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text("another");
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(&mut self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn reject(self: Box<Self>) -> Box<State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(&self, content: &mut String, text: &str) {}
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }

    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {
            approved_cnt: 0,
        })
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {
    approved_cnt: u8
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        if self.approved_cnt == 1 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview { approved_cnt: 1})
        }
    }
    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

mod encoding_states_as_type;