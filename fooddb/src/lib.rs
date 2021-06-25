
//use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read};
use std::time::{SystemTime, UNIX_EPOCH};

mod food;
mod meal;

use food::{Food, FoodID, FoodQuantity};
use meal::{Meal, MealID};

#[derive(Serialize, Deserialize)]
pub struct FoodDB {
	last_food_id: FoodID,
	last_meal_id: MealID,
	foods: Vec<Food>,
	meals: Vec<Meal>,
}

impl FoodDB {
	fn new(filename: &str) -> Self {
		FoodDB {
			last_food_id: 0,
			last_meal_id: 0,
			foods: vec![],
			meals: vec![],
		}
	}

	fn open(filename: &str) -> Result<Self, Error> {
		let mut fin = File::open(filename)?;
		let mut json_buffer = String::new();
		fin.read_to_string(&mut json_buffer);
		let deserialized: FoodDB = serde_json::from_str(&json_buffer)?;
		Ok(deserialized)
	}

	fn new_meal(&mut self) -> MealID {
		let next_meal_id = self.last_meal_id+1;
		self.last_meal_id = next_meal_id;
		let meal = Meal {
			id: next_meal_id,
			..Meal::default()
		};
		self.meals.push(meal);
		next_meal_id
	}

	fn new_food(&mut self) -> FoodID {
		unimplemented!()
	}

	fn add_food_to_meal(meal:MealID, food:FoodID, quantity:FoodQuantity) {
		unimplemented!()
	}
}


pub fn save_db(db:&FoodDB, filename: &str) {
    let serialized = serde_json::to_string(db).unwrap();
    println!("serialized = {}", serialized);
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
