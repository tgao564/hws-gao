/*
TASK 1
*******
Ownership & Borrowing:
Step 1: Write a function that increments a number by 1.
        Make sure to create a mutable variable that you can reference
        outside of the scope of ownerhsip.
*/

pub fn plus_one(n: &mut i32) {
    *n +=1;
}


/*
TASK 2
*******
Struct and Implement

Create a Rectangle struct. Then, implement the following functions:
    1. Checks if the Rectangle is a square, and returns a boolean
    2. Calculates the area of the Rectangle.
*/

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn is_square(&self) -> bool {
     if self.width == self.height {
        return true
     }
     else{
        return false
     }
    }

    pub fn calc_area(&self) -> u8 {
        return (self.width * self.height) as u8
    }
}

/*
TASK 3
*******
Match - Create a function with a match pattern that is able to sort coins.

*/

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter

}

pub fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
