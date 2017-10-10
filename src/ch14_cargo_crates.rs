/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use learn_rust::ch14_cargo_crates::add_one;
///
/// let five = 5;
///
/// assert_eq!(6, add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use ch14_cargo_crates::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

#[test]
fn test_normal_use() {
    use ch14_cargo_crates::kinds::PrimaryColor;
    use ch14_cargo_crates::utils::mix;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

pub use ch14_cargo_crates::kinds::PrimaryColor;
pub use ch14_cargo_crates::kinds::SecondaryColor;
pub use ch14_cargo_crates::utils::mix;

#[test]
fn test_pub_use() {
    use ch14_cargo_crates::PrimaryColor;
    use ch14_cargo_crates::mix;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

//cargo login, publish, yank

//A workspace is a set of packages that will all share the same Cargo.lock and output directory.
//cargo.toml  [workspace]
//[dependencies]
//add-one = { path = "add-one" }
//cargo test -p add-one

//If a binary in your $PATH is named cargo-something, you can run it as if it were a Cargo subcommand by running cargo something.
//cargo --list
