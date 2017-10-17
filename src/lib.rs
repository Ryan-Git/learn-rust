mod ch4_ownership;
mod ch5_rectangle;
mod ch5_structs;
mod ch6_enums;
mod ch7_modules;
mod ch8_collections;
mod ch9_error;
mod ch10_generic_trait_lifetime;
mod ch11_test;
mod ch13_fp;
pub mod ch14_cargo_crates;
mod ch15_smart_pointers;
mod ch16_concurrency;
mod ch17_oop;
mod ch18_pattern_match;

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}