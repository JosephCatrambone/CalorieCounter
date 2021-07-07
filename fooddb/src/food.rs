use serde::{Deserialize, Serialize};
//use serde_json::Result;

use crate::nutrition::Nutrients;

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

	pub nutrition: Nutrients,

	pub mass: u64, // Should be 100g
	pub volume_of_100g: f64, // What is 100g in ml / cm^3?
	pub servings_in_100g: f64, // How many servings is 100g?

	// Remove is_composite because can say this is true from ingredients being non-empty.
	//is_composite: bool, // Is this just 'defined' as something, or is this a product of other foods?  
	pub user_defined: bool,
	pub ingredients: Vec<(FoodID, FoodQuantity)>,
}

impl Clone for Food {
	fn clone(&self) -> Self {
		Food {
			parent_id: self.parent_id,
			id: self.id,
			name: self.name.clone(),
			manufacturer: self.manufacturer.clone(),
			tags: self.tags.clone(),
			
			nutrition: self.nutrition.clone(),
			
			mass: self.mass,
			volume_of_100g: self.volume_of_100g,
			servings_in_100g: self.servings_in_100g,
			
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
			nutrition: Nutrients::default(),
			mass: 100,
			volume_of_100g: 0f64,
			servings_in_100g: 0f64, // If 1 serving is 200g, this is 0.5.  100 / mass_per_serving.
			user_defined: false,
			ingredients: vec![]
		}
	}
}

impl Food {
	fn get_nutrition(&self, amount:FoodQuantity) -> Nutrients {
		// Foods should be in 100g servings.
		let mut nutrients = self.nutrition.clone();

		if self.mass != 100 {
			// Oi!  This food should be a 100g serving!  Convert the nutrients to that.
			nutrients *= 100f32 / (self.mass as f32);
		}

		let scale_factor = match amount {
			FoodQuantity::Mass(grams) => { grams as f32 / 100f32 }
		}

		nutrients
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
