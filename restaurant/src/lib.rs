mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // Summer breakfast with Rye toast
    // If we put `pub use meals::*;` in *back_of_house.rs*, this could be:
    // `back_of_house::Breakfast` directly w/o `meals::`
    let mut meal = back_of_house::meals::Breakfast::summer("Rye");
    // Actually I changed my mind
    meal.toast = String::from("White");
    println!("I'd like {} toast please", meal.toast);

    // Can't do this: seasonal_fruit can't be accessed or changed. Chef's choice.
    // Struct fields are private by default.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}
