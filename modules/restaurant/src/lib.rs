use rand::{CryptoRng, ErrorKind::Transient, Rng};

use std::io::*;
mod front_of_house;
pub use crate::front_of_house::hosting;
use back_of_house::Appetizer;
pub fn eat_at_restaurant() {
    let secret_number: i32 = rand::thread_rng().gen_range(0, 10);

    // absolute path
    front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal: crate::back_of_house::Breakfast = back_of_house::Breakfast::summer("toast");
    meal.toast = String::from("Wheat");

    hosting::add_to_waitlist();

    let order1: Appetizer = Appetizer::Soup;
    let order2: Appetizer = Appetizer::Salad;
}

fn server_order() {}

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }
    fn cook_order() {}
}

//use std::fmt;
//use std::io;

//fn function1() -> fmt::Result {}

//fn function2() -> io::Result<()> {}
