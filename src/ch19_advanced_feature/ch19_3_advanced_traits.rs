//associated type
trait Foo {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait Bar<T> {
    fn next(&mut self) -> Option<T>;
}

//when a trait has a generic parameter, we can implement that trait for a type multiple times, changing the generic type parameters' concrete types each time.
//With associated types, we can't implement a trait on a type multiple times.

trait GGraph<Node, Edge> {
    // methods would go here
}

//E is not used but have to specify
fn gDistance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { 1 }

trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}

fn aDistance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
    1
}

//trait objects with associated types
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 { 1 }

fn traverse(graph: &AGraph<Node=usize, Edge=(usize, usize)>) {}

//operator overloading and default type parameters
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn operator_overloading() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 }
    );
}

struct Millimeters(u32);

struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

//Fully Qualified Syntax for Disambiguation
#[test]
fn fully_qualified_syntax() {
    let b = Baz;
    b.f();//Baz's method

    <Baz as Foo2>::f(&b);
    <Baz as Bar2>::f(&b);
}

trait Foo2 {
    fn f(&self);
}

trait Bar2 {
    fn f(&self);
}

struct Baz;

impl Foo2 for Baz {
    fn f(&self) {
        println!("Baz's impl of Foo2");
    }
}

impl Bar2 for Baz {
    fn f(&self) {
        println!("Baz's impl of Bar2");
    }
}

impl Baz {
    fn f(&self) {
        println!("Baz's impl");
    }
}

//super trait
use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//newtype pattern
use std::fmt;
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}