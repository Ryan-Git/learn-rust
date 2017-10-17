#[test]
fn places_patterns_may_use() {
    {
        let o = Some(1);
        match o {
            Some(t) => t,
            None => 0,
        };
    }

    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    {
        let v = vec![1, 2, 3];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    {
        let x = 5;//let PATTERN = EXPRESSION;
        let (x, y, z) = (1, 2, 3);
    }

    //function parameter list
    {
        let point = (3, 5);
        print_coordinates(&point);
    }

    {
        //literals
        {
            let x = 1;

            match x {
                1 => println!("one"),
                2 => println!("two"),
                3 => println!("three"),
                _ => println!("anything"),
            }
        }

        //named variables
        {
            let x = Some(5);
            let y = 10;

            match x {
                Some(50) => println!("Got 50"),
                Some(y) => println!("Matched, y = {:?}", y),
                _ => println!("Default case, x = {:?}", x),
            }

            println!("at the end: x = {:?}, y = {:?}", x, y);
        }

        //multiple patterns
        {
            let x = 1;

            match x {
                1 | 2 => println!("one or two"),
                3 => println!("three"),
                _ => println!("anything"),
            }
        }

        //range
        {
            let x = 5;

            match x {
                1 ... 5 => println!("one through five"),
                _ => println!("something else"),
            }

            let x = 'c';

            match x {
                'a' ... 'j' => println!("early ASCII letter"),
                'k' ... 'z' => println!("late ASCII letter"),
                _ => println!("something else"),
            }
        }

        //Destructuring to Break Apart Values
        {
            let p = Point { x: 0, y: 7 };

            let Point { x, y } = p;
            assert_eq!(0, x);
            assert_eq!(7, y);

            let Point { x: a, y: b } = p;
            assert_eq!(0, a);
            assert_eq!(7, b);

            match p {
                Point { x, y: 0 } => println!("On the x axis at {}", x),
                Point { x: 0, y } => println!("On the y axis at {}", y),
                Point { x, y } => println!("On neither axis: ({}, {})", x, y),
            }

            let points = vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 5 },
                Point { x: 10, y: -3 },
            ];
            let sum_of_squares: i32 = points
                .iter()
                .map(|&Point {x, y}| x * x + y * y)
                .sum();

            let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
        }

        //ignoring
        {
            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, _, third, _, fifth) => {
                    println!("Some numbers: {}, {}, {}", first, third, fifth)
                },
            }

            let _x = 5; //no warning about unused
            let y = 10;

            let origin = Point { x: 0, y: 0};

            match origin {
                Point { x, .. } => println!("x is {}", x),
            }

            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, .., last) => {
                    println!("Some numbers: {}, {}", first, last);
                },
            }
        }

        //ref & ref mut
        {
            let robot_name = Some(String::from("Bors"));

            match robot_name {
                Some(ref name) => println!("Found a name: {}", name),
                None => (),
            }

            println!("robot_name is: {:?}", robot_name);

            let mut robot_name = Some(String::from("Bors"));

            match robot_name {
                Some(ref mut name) => *name = String::from("Another name"),
                None => (),
            }

            println!("robot_name is: {:?}", robot_name);
        }

        //guards
        {
            let num = Some(4);

            match num {
                Some(x) if x < 5 => println!("less than five: {}", x),
                Some(x) => println!("{}", x),
                None => (),
            }

            let x = 4;
            let y = false;

            match x {
                4 | 5 | 6 if y => println!("yes"), //(4 | 5 | 6) if y
                _ => println!("no"),
            }
        }

        //@ binding
        {
            let msg = Message::Hello { id: 5 };

            match msg {
                Message::Hello { id: id @ 3...7 } => {
                    println!("Found an id in range: {}", id)
                },
                Message::Hello { id: 10...12 } => {
                    println!("Found an id in another range")
                },
                Message::Hello { id } => {
                    println!("Found some other id: {}", id)
                },
            }
        }
    }
}

enum Message {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

fn foo(x: i32) {
    //x is a pattern
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

//Refutability: Whether a Pattern Might Fail to Match
//Patterns which cannot fail to match for any possible value are said to be irrefutable, and patterns which can fail to match for some possible value are said to be refutable.
//let statements, function parameters, and for loops are restricted to only accept irrefutable patterns, since there's nothing correct the program could do if the pattern fails to match.
//if let, and while let expressions are restricted to only accept refutable patterns, since they're made to handle possible failure and we wouldn't need their functionality if the pattern could never fail.