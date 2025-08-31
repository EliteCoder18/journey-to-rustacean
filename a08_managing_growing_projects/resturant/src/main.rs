use resturant::back_of_house::Breakfast;

fn main() {
    eat_at_resturant();
}

fn eat_at_resturant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = resturant::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like to order {} toast please", meal.toast);
}