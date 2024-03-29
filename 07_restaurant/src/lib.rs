// we can do this since the file `front_of_house.rs`
// exists in the same directory
mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// external code importing this library
// can use hosting due to `pub` keyboard
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Relative path with `use`
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("naan");
    // Change our mind about what bread we'd like
    meal.toast = String::from("wholemeal");
    println!("I'd like {} toast, please!", meal.toast);

    // This line would error:
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
