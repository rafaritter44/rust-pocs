use std::collections::{HashMap, VecDeque};

struct Dish {
    name: String,
    prep_time: u32, // in minutes
}

struct Restaurant {
    menu: HashMap<String, u32>, // dish name -> prep time
    queue: VecDeque<Dish>,      // queue of ordered dishes
}

impl Restaurant {
    fn new() -> Self {
        Restaurant {
            menu: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn add_dish_to_menu(&mut self, name: &str, prep_time: u32) {
        self.menu.insert(name.to_string(), prep_time);
    }

    fn place_order(&mut self, dish_name: &str) -> Result<(), String> {
        match self.menu.get(dish_name) {
            Some(&prep_time) => {
                self.queue.push_back(Dish {
                    name: dish_name.to_string(),
                    prep_time,
                });
                Ok(())
            }
            None => Err(format!("Dish '{}' not on menu.", dish_name)),
        }
    }

    fn print_estimated_times(&self) {
        let mut total_time = 0;

        for dish in &self.queue {
            total_time += dish.prep_time;
            println!("{} will be ready in {} minutes.", dish.name, total_time);
        }
    }
}

fn main() {
    let mut restaurant = Restaurant::new();

    // Setup menu
    restaurant.add_dish_to_menu("Burger", 10);
    restaurant.add_dish_to_menu("Fries", 5);
    restaurant.add_dish_to_menu("Salad", 3);

    // Place orders
    restaurant.place_order("Burger").unwrap();
    restaurant.place_order("Fries").unwrap();
    restaurant.place_order("Salad").unwrap();

    // Output estimated times
    restaurant.print_estimated_times();
}