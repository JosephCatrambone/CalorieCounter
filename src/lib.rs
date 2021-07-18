
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Result, prelude::*};

mod food;
mod meal;
mod nutrition;
mod search;

use food::{Food, FoodID, FoodQuantity};
use meal::{Meal, MealID};
use search::*;

#[derive(Serialize, Deserialize)]
pub struct FoodDB {
	last_food_id: u64,
	last_meal_id: u64,
	foods: Vec<Food>,
	meals: Vec<Meal>,
	#[serde(skip)]
	food_index: SearchIndex,
}

impl Default for FoodDB {
	fn default() -> Self {
		FoodDB {
			last_food_id: 0,
			last_meal_id: 0,
			foods: vec![],
			meals: vec![],
			food_index: SearchIndex::empty(),
		}
	}
}

impl FoodDB {
	pub fn new() -> Self {
		FoodDB::default()
	}

	pub fn open(filename: &str) -> Result<Self> {
		let fin = File::open(filename)?;
		let mut reader = BufReader::new(fin);
		let mut json_buffer = String::new();
		reader.read_to_string(&mut json_buffer)?;
		let mut deserialized: FoodDB = serde_json::from_str(&json_buffer)?;
		deserialized.food_index.reindex(&deserialized.foods);
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
			user_defined: true,
			..Food::default()
		};
		self.foods.push(food);
		self.food_index.reindex(&self.foods);
		next_food_id
	}

	pub fn add_food_to_meal(&mut self, meal: MealID, food: FoodID, quantity: FoodQuantity) -> bool {
		let food_ref = self.get_food_from_id(food);

		if food_ref.is_none() {
			return false;
		}

		// Look up the meal.
		let mut meal_ref: Option<&mut Meal> = None;
		for m in &mut self.meals {
			if m.id == meal {
				meal_ref = Some(m);
				break;
			}
		}

		// If we can't find the food or meal, abort.
		if let (Some(m), Some(f)) = (meal_ref, food_ref) {
			let nutrition = f.get_nutrition(quantity);
			m.nutrients.calories += nutrition.calories;
			m.nutrients.proteins += nutrition.proteins;
			m.nutrients.carbohydrates += nutrition.carbohydrates;
			m.nutrients.fats += nutrition.fats;
			m.foods.push((food, quantity));
			return true;
		} else {
			return false;
		}
	}

	fn get_food_from_id(&self, food:FoodID) -> Option<Food> {
		// Look up the food.
		for f in &self.foods {
			if f.id == food {
				return Some(f.clone());
			}
		}
		return None;
	}

	fn find_food_by_name(&self, name:&str) -> FoodID {
		todo!()
	}

	fn get_autocomplete_suggestions(&self, food_name:String) -> Vec<String> {
		self.food_index.search(&food_name).iter().map(|fsr|{ fsr.name.clone() }).collect()
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn make_empty_food_db() {
		let mut db = FoodDB::new();
		let new_food_id = db.new_food();
		//println!("New food id: {}", new_food_id);
		db.save("empty.fdb");
	}
}
