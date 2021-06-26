use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::food::*;

pub type MealID = u64;

#[derive(Serialize, Deserialize)]
pub struct Meal {
	pub id: MealID,
	pub name: String,
	pub tags: String,

	pub time: u64, // Unix timestamp.
	pub year: u64,
	pub month: u8,
	pub day: u8,
	pub meal_name: String, // Breakfast, Lunch, Dinner, etc.

	// Normalized data, rolled up from the linked Foods.
	pub calories: u64,
	pub carbohydrate: f64,
	pub fat: f64,
	pub protein: f64,

	// Foods inside.
	pub foods: Vec<Food>,
}

impl Default for Meal {
	fn default() -> Self {
		Meal {
			id: 0,
			name: String::new(),
			meal_name: String::new(),
			tags: String::new(),

			time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
			year: 0u64,
			month: 0u8,
			day: 0u8,

			calories: 0,
			carbohydrate: 0.0,
			fat: 0.0,
			protein: 0.0,

			foods: vec![],
		}
	}
}

impl Meal {
	pub fn add_food_to_meal(&mut self, food:&Food) {
		self.calories += food.calories;
		self.carbohydrate += food.carbohydrate;
		self.fat += food.fat;
		self.protein += food.protein;
		self.foods.push(food.clone());
	}
}