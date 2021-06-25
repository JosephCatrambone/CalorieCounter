use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone)]
pub enum FoodQuantity {
	Mass(u64), // in grams
	Volume(f64), // in cm^3, a.k.a., ml.
	Serving(f64)
}

pub type FoodID = u64;

#[derive(Serialize, Deserialize)]
pub struct Food {
	pub id: u64,
	pub name: String,
	pub manufacturer: String,

	pub tags: String,  // These are | separated.

	pub calories: u64,
	pub carbohydrate: u64,
	pub fat: u64,
	pub protein: u64,
	
	pub mass: u64, // Should be 100g
	pub volume: f64, // What is 100g in ml / cm^3?
	pub serving: f64, // How many servings is 100g?

	// Remove is_composite because can say this is true from ingredients being non-empty.
	//is_composite: bool, // Is this just 'defined' as something, or is this a product of other foods?  
	pub user_defined: bool,
	pub ingredients: Vec<(FoodID, FoodQuantity)>,
}

impl Clone for Food {
	fn clone(&self) -> Self {
		Food {
			id: self.id,
			name: self.name.clone(),
			manufacturer: self.manufacturer.clone(),
			tags: self.tags.clone(),
			calories: self.calories,
			carbohydrate: self.carbohydrate,
			fat: self.fat,
			protein: self.protein,
			mass: self.mass,
			volume: self.volume,
			serving: self.serving,
			user_defined: self.user_defined,
			ingredients: self.ingredients.iter().map(|(fid, fq)| { (*fid, fq.clone()) }).collect::<Vec<(FoodID, FoodQuantity)>>()
		}
	}
}

impl Default for Food {
	fn default() -> Self {
		Food {
			id: 0,
			name: String::new(),
			manufacturer: String::new(),
			tags: String::new(),
			calories: 0,
			carbohydrate: 0,
			fat: 0,
			protein: 0,
			mass: 100,
			volume: 0f64,
			serving: 0f64,
			user_defined: false,
			ingredients: vec![]
		}
	}
}

impl Food {
	fn scale(&mut self, scale_factor: f64) {
		self.calories = (self.calories as f64 * scale_factor) as u64;
		self.carbohydrate = (self.carbohydrate as f64 * scale_factor) as u64;
		self.fat = (self.fat as f64 * scale_factor) as u64;
		self.protein = (self.protein as f64 * scale_factor) as u64;
		self.mass *= (scale_factor as f64 * scale_factor) as u64;
		self.volume *= scale_factor;
		self.serving *= scale_factor;
	}
}

/// ```
/// /// convert_food_quantity will take the given food, clone it internally, and return a new food with the same ID that has a different quantity.
/// # let base_food = Food::default();
/// let new_food = convert_food_quantity(&base_food, FoodQuantity::Mass(200)); // What would the nutrition properties be for a 200g serving?
/// ```
fn convert_food_quantity(food: &Food, to_quantity:FoodQuantity) -> Food {
	let mut converted_food = food.clone();

	converted_food.scale(match to_quantity {
		FoodQuantity::Mass(to_grams) => { to_grams as f64 / food.mass as f64 },
		FoodQuantity::Volume(to_volume) => { to_volume / converted_food.volume },
		FoodQuantity::Serving(to_servings) => { to_servings / converted_food.serving }
	});

	converted_food
}

#[cfg(test)]
mod tests {
	use crate::food::*;

	#[test]
	fn test_scale() {
		let mut food = Food::default();
		food.id = 123;
		food.name = String::from("asdfasdf");
		food.manufacturer = String::from("ruewiongew");
		food.tags = String::from("uifouaf");

		let mut same_food = food.clone();
		same_food.scale(1.0f64);

		assert_eq!(food.id, same_food.id);
		assert_eq!(food.name, same_food.name);
		assert_eq!(food.calories, same_food.calories);
	}
}
