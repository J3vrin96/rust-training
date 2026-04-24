    mod back_of_house {
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
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast.
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like.
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // // Code failed to compile here because seasonal_fruit is a private field
        // meal.seasonal_fruit = String::from("Strawberry");
        // println!("I'd like {} fruit please", meal.seasonal_fruit);
    }