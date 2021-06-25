
//use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Result, prelude::*};

mod food;
mod meal;

use food::{Food, FoodID, FoodQuantity};
use meal::{Meal, MealID};

#[derive(Serialize, Deserialize)]
pub struct FoodDB {
	last_food_id: u64,
	last_meal_id: u64,
	foods: Vec<Food>,
	meals: Vec<Meal>,
}

impl FoodDB {
	pub fn new() -> Self {
		FoodDB {
			last_food_id: 0,
			last_meal_id: 0,
			foods: vec![],
			meals: vec![],
		}
	}

	pub fn open(filename: &str) -> Result<Self> {
		let mut fin = File::open(filename)?;
		let mut reader = BufReader::new(fin);
		let mut json_buffer = String::new();
		reader.read_to_string(&mut json_buffer);
		let deserialized: FoodDB = serde_json::from_str(&json_buffer)?;
		Ok(deserialized)
	}

	pub fn save(&self, filename:&str) -> Result<()> {
		let serialized = serde_json::to_vec(self)?;
		//let mut fout = OpenOptions::new().write(true).create(true).truncate(true).open(filename);
		let mut fout = File::create(filename)?;
		fout.write_all(&serialized)?;
		Ok(())
	}

	pub fn new_meal(&mut self) -> MealID {
		let next_meal_id = self.last_meal_id+1;
		self.last_meal_id = next_meal_id;
		let meal = Meal {
			id: next_meal_id,
			..Meal::default()
		};
		self.meals.push(meal);
		next_meal_id
	}

	pub fn new_food(&mut self) -> FoodID {
		let next_food_id = self.last_food_id+1;
		self.last_food_id = next_food_id;
		let food = Food {
			id: next_food_id,
			..Food::default()
		};
		self.foods.push(food);
		next_food_id
	}

	pub fn add_food_to_meal(meal: MealID, food: FoodID, quantity: FoodQuantity) {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
