#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    //Associated Functions(without self)
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

#[test]
fn main() {
    let rect = Rectangle { length: 50, width: 30 };

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    println!("area rectangle: {}", area_rec(&rect));
    println!("rectangle.area(): {}", rect.area());

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("square(10): {:?}", Rectangle::square(10));
}

fn area_rec(rec: &Rectangle) -> u32 {
    rec.length * rec.width
}
