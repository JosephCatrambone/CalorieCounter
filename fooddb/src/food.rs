use serde::{Deserialize, Serialize};
//use serde_json::Result;

use crate::nutrition::Nutrients;

#[derive(Serialize, Deserialize, Clone)]
pub enum FoodQuantity {
	Mass(u32), // in grams.  Max value: approximate mass of Saturn-V rocket.
	Volume(f32), // in cm^3, a.k.a., ml.  Max value: a cube with sides of several million km.
	Serving(f32)
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

	pub mass: u32, // Should be 100g.  This is NOT molar mass.  Just a scalar, just in case.
	pub volume_of_100g: f32, // What is 100g in ml / cm^3?  This is the reciprocal of 'density', sometimes called 'specific volume'.
	pub servings_in_100g: f32, // How many servings is 100g?

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
			volume_of_100g: 0.0,
			servings_in_100g: 0.0, // If 1 serving is 200g, this is 0.5.  100 / mass_per_serving.
			user_defined: false,
			ingredients: vec![]
		}
	}
}

impl Food {
	fn get_nutrition(&self, amount:FoodQuantity) -> Nutrients {
		// Foods should be in 100g servings.
		let mut nutrients = self.nutrition.clone();

		// Convert the amount in scale factor to a 100g equivalent.
		let scale_factor = match amount {
			FoodQuantity::Mass(grams) => { (grams as f32) / (self.mass as f32) },
			FoodQuantity::Volume(cm3) => { cm3 / self.volume_of_100g },
			FoodQuantity::Serving(servings) => { servings / self.servings_in_100g }
		};

		nutrients * scale_factor
	}
}


#[cfg(test)]
mod tests {
	use crate::food::*;

	#[test]
	fn test_amount_scaling() {
		let mut sugar = Food::default();
		sugar.name = String::from("sugar");

		sugar.mass = 100; // This should be 100 by default.
		sugar.volume_of_100g = 118.29; // Some say 0.7g/cm^3 density, which is 1.428cm^3/g specific volume -> x100g = 142.8cm^3.  Going with 118.29.
		sugar.servings_in_100g = 3.57; // A serving is 28g.  3.57 servings = 100g.

		sugar.nutrition.calories = 387; // 387 calories per 100g.
		sugar.nutrition.carbohydrates = 10.0;
		sugar.nutrition.fats = 10.0;
		sugar.nutrition.proteins = 10.0;

		// Scale sugar.
		let sugar_5g = sugar.get_nutrition(FoodQuantity::Mass(5));
		let sugar_1tsp = sugar.get_nutrition(FoodQuantity::Volume(4.9f32)); // 1 tsp = 4.9ml^2
		// 16 calories in 1tsp sugar.  (Which is about 4.2g)
		assert_eq!(sugar_5g.calories, 387/20);
		assert_eq!(sugar_1tsp.calories, 16);
	}
}
