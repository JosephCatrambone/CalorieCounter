use serde::{Deserialize, Serialize};
//use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

use crate::food::*;
use crate::nutrition::Nutrients;

pub type MealID = usize;

// This duplicates a lot of fields from Foods, but is distinct and is a focus of lib.
#[derive(Serialize, Deserialize)]
pub struct Meal {
	pub id: MealID,
	pub name: String,
	pub tags: String,

	pub time: DateTime<Utc>,
	pub meal_name: String, // Breakfast, Lunch, Dinner, etc.

	// Normalized data, rolled up from the linked Foods.
	pub nutrients: Nutrients,

	// Foods inside.
	// Note that we can't do a roll-up in here because it relies on a Food dict and we can't loop up FoodID internally.
	pub foods: Vec<(FoodID, FoodQuantity)>,
}

impl Default for Meal {
	fn default() -> Self {
		Meal {
			id: 0,
			name: String::new(),
			meal_name: String::new(),
			tags: String::new(),

			time: Utc::now(),

			nutrients: Nutrients::default(),

			foods: vec![],
		}
	}
}