
use bzip2::{Compression, read::{BzEncoder, BzDecoder}};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Result, prelude::*};

mod food;
mod meal;
mod nutrition;
mod search;

pub use food::{Food, FoodID, FoodQuantity};
pub use meal::{Meal, MealID};
use search::*;
use chrono::Datelike;
use bzip2::Decompress;

#[derive(Serialize, Deserialize)]
pub struct FoodDB {
	foods: Vec<Food>,
	meals: Vec<Meal>,
	#[serde(skip)]
	food_index: SearchIndex,
}

impl Default for FoodDB {
	fn default() -> Self {
		FoodDB {
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
		Self::from_string(&json_buffer)
	}

	pub fn from_string(json_string:&str) -> Result<Self> {
		let mut deserialized: FoodDB = serde_json::from_str(json_string)?;
		deserialized.food_index.reindex(&deserialized.foods);
		Ok(deserialized)
	}

	pub fn from_bz2(blob:&[u8]) -> Result<Self> {
		let mut buffer = String::new();
		let mut decompressor = BzDecoder::new(blob);
		decompressor.read_to_string(&mut buffer);
		Self::from_string(&buffer)
	}

	pub fn save(&self, filename:&str) -> Result<()> {
		let serialized = serde_json::to_vec(self)?;
		//let mut fout = OpenOptions::new().write(true).create(true).truncate(true).open(filename);
		let mut fout = File::create(filename)?;
		fout.write_all(&serialized)?;
		Ok(())
	}

	pub fn save_compressed(&self, filename:&str) -> Result<()> {
		todo!()
	}

	pub fn new_meal(&mut self) -> MealID {
		let next_meal_id = self.meals.len();
		let meal = Meal {
			id: next_meal_id,
			..Meal::default()
		};
		self.meals.push(meal);
		next_meal_id
	}

	pub fn new_food(&mut self) -> &mut Food {
		let next_food_id = self.foods.len();
		let food = Food {
			id: next_food_id as u64,
			user_defined: true,
			..Food::default()
		};
		self.foods.push(food);
		self.food_index.reindex(&self.foods);
		self.foods.get_mut(next_food_id).expect("Unable to fetch newly added food reference.  Out of memory?")
	}

	pub fn add_food_to_meal(&mut self, meal: MealID, food: FoodID, quantity: FoodQuantity) -> bool {
		let food_ref = self.get_food_from_id(food);

		if food_ref.is_none() {
			return false;
		}

		let nutrition = food_ref.unwrap().get_nutrition(quantity);

		let mut meal_ref = self.get_meal_mut_from_id(meal);

		// If we can't find the food or meal, abort.
		if let Some(m) = meal_ref {
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

	pub fn get_food_from_id(&self, food_id:FoodID) -> Option<&Food> {
		// food_id should be the position in the array.
		let opt_food = self.foods.get(food_id as usize);
		if let Some(f) = opt_food {
			if f.id != food_id {
				eprintln!("ERROR: food {} not found at ID index.  {} != {}", f.name, f.id, food_id);
			}
		}

		opt_food
	}

	pub fn get_food_mut_from_id(&mut self, food_id:FoodID) -> Option<&mut Food> {
		// food_id should be the position in the array.
		match self.foods.get_mut(food_id as usize) {
			Some(f) => {
				if f.id != food_id {
					panic!("ERROR: food {} not found at ID index.  {} != {}", f.name, f.id, food_id);
				}
				Some(f)
			},
			None => None
		}
	}

	pub fn get_meal_from_id(&self, meal_id:MealID) -> Option<&Meal> {
		self.meals.get(meal_id)
	}

	pub fn get_meal_mut_from_id(&mut self, meal_id:MealID) -> Option<&mut Meal> {
		self.meals.get_mut(meal_id)
	}

	pub fn get_meals_from_date(&self, year:i32, month:u32, day:u32) -> Vec<MealID> {
		let mut results = vec![];
		// Brute force search.  :/
		for m in &self.meals {
			if m.time.year() == year && m.time.month() == month && m.time.day() == day {
				results.push(m.id);
			}
		}
		results
	}

	pub fn get_autocomplete_suggestions(&self, food_name:String) -> Vec<(FoodID, String)> {
		self.food_index.search(&food_name, None).iter().map(|fsr|{ (fsr.id, fsr.name.clone()) }).collect()
	}

	pub fn reindex(&mut self) {
		self.food_index.reindex(&self.foods);
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
		let db2 = FoodDB::open("empty.fdb").unwrap();
	}

	#[test]
	fn make_single_entry_food_db() {
		let mut db = FoodDB::new();
		{
			let mut food = db.new_food();
			food.name = "Tasty Food".to_string();
			food.nutrition.proteins = 20.0f32;
			food.nutrition.fats = 20.0f32;
			food.nutrition.carbohydrates = 60.0f32;
			food.nutrition.calories = 9 * 20 + 4 * 20; // 9 calories per gram of fat.  4 per gram of carbs.
		}
		db.reindex();
		db.save("single_food.fdb");
		let mut db2 = FoodDB::open("single_food.fdb").unwrap();
		assert_eq!(db2.foods.len(), 1);
		db2.reindex();
		assert_eq!(db2.get_autocomplete_suggestions("Tasty".to_string()).len(), 1);
	}
}
