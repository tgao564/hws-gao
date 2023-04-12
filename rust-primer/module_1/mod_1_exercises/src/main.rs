// pub mod solution;

use solution::{hello, is_leap_year};

fn main() {
    println!("{}", hello());
    println!("Is 1900 a leap year? {}", is_leap_year(1900));
}
