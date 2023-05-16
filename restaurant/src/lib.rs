mod front_of_house;

fn _serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    fn _cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
   hosting::add_to_waitlist();

    // Relative path
   hosting::add_to_waitlist();
   hosting::add_to_waitlist();
    

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
