#[test]
fn closure_test() {
    use std::thread;
    use std::time::Duration;

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    fn generate_workout(intensity: i32, random_number: i32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_result.value(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                )
            }
        }
    }

    //    fn add_one_v1(x: i32) -> i32 { x + 1 }
    //    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    //    let add_one_v3 = |x| { x + 1 };
    //    let add_one_v4 = |x| x + 1;

    {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        //        println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}

#[test]
fn iterators_test() {
    {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

    {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

use std::collections::HashMap;
use std;

struct Cacher<T, P, R>
    where
        T: Fn(P) -> R,
        P: std::hash::Hash + Eq + Copy,
        R: Copy
{
    calculation: T,
    result: HashMap<P, R>,
}

impl<T, P, R> Cacher<T, P, R>
    where
        T: Fn(P) -> R,
        P: std::hash::Hash + Eq + Copy,
        R: Copy
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            calculation,
            result: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> R {
        if self.result.contains_key(&arg) {
            self.result[&arg]
        } else {
            let v = (self.calculation)(arg);
            self.result.insert(arg, v);
            v
        }
    }
}