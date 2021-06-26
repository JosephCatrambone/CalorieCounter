use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize, Clone)]
pub enum FoodQuantity {
	Mass(u64), // in grams
	Volume(f64), // in cm^3, a.k.a., ml.
	Serving(f64)
}

pub type FoodID = u64;

#[derive(Serialize, Deserialize)]
pub struct Food {
	pub parent_id: FoodID,
	pub id: FoodID,
	pub name: String,
	pub manufacturer: String,
	pub tags: String,  // These are | separated.

	pub calories: u64,
	pub carbohydrate: f64,
	pub fat: f64,
	pub protein: f64,
	
	pub mass: u64, // Should be 100g
	pub volume: f64, // What is 100g in ml / cm^3?
	pub serving: f64, // How many servings is 100g?

	// Remove is_composite because can say this is true from ingredients being non-empty.
	//is_composite: bool, // Is this just 'defined' as something, or is this a product of other foods?  
	pub user_defined: bool,
	pub ingredients: Vec<Food>,
}

impl Clone for Food {
	fn clone(&self) -> Self {
		Food {
			parent_id: self.parent_id,
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
			ingredients: self.ingredients.clone(),
		}
	}
}

impl Default for Food {
	fn default() -> Self {
		Food {
			parent_id: 0,
			id: 1,
			name: String::new(),
			manufacturer: String::new(),
			tags: String::new(),

			calories: 0,
			carbohydrate: 0.0,
			fat: 0.0,
			protein: 0.0,

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
		self.carbohydrate *= scale_factor;
		self.fat *= scale_factor;
		self.protein *= scale_factor;
		self.mass *= (scale_factor as f64 * scale_factor) as u64;
		self.volume *= scale_factor;
		self.serving *= scale_factor;
	}

	/// convert_to_quantity will take the given food, clone it internally, and return a new food with the same ID that has a different quantity.
	pub fn convert_to_quantity(&self, to_quantity:FoodQuantity) -> Food {
		let mut converted_food = self.clone();

		converted_food.scale(match to_quantity {
			FoodQuantity::Mass(to_grams) => { to_grams as f64 / self.mass as f64 },
			FoodQuantity::Volume(to_volume) => { to_volume / self.volume },
			FoodQuantity::Serving(to_servings) => { to_servings / self.serving }
		});

		converted_food
	}
}



#[cfg(test)]
mod tests {
	use crate::food::*;

	#[test]
	fn test_scale() {
		let mut food = Food::default();
		food.name = String::from("asdfasdf");
		food.manufacturer = String::from("ruewiongew");
		food.tags = String::from("uifouaf");

		let mut same_food = food.clone();
		same_food.scale(1.0f64);

		assert_eq!(food.name, same_food.name);
		assert_eq!(food.calories, same_food.calories);
	}
}
