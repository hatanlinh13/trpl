#![allow(dead_code)]

mod back_of_house;
mod front_of_house;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use self::front_of_house::hosting::add_to_waitlist;
use crate::front_of_house::hosting;
pub use back_of_house::Appetizer as appetizerMeal;
pub use back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // next line cannot compile
    //meal.seasonal_fruit = String::from("Blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    if let back_of_house::Appetizer::Soup = _order1 {
        //there is no warning when "_ prefixed" variables was used
        println!("tada!!");
    }

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    add_to_waitlist();
}

#[allow(unused_imports)]
use std::io::{self, Write};
