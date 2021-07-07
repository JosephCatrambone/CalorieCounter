use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Serialize, Deserialize, Clone)]
pub struct Nutrients {
	calories: u32,  // In units
	fats: f32, // In grams
	carbohydrates: f32, // In grams
	proteins: f32 // In grams
}

impl Default for Nutrients {
	fn default() -> Self {
		Nutrients {
			calories: 0,
			fats: 0f32,
			carbohydrates: 0f32,
			proteins: 0f32,
		}
	}
}

impl Mul<f32> for Nutrients {
	type Output = Nutrients;

	fn mul(self, rhs: f32) -> Self::Output {
		Nutrients {
			calories: ((self.calories as f32) * rhs) as u32,
			fats: self.fats * rhs,
			carbohydrates: self.carbohydrates * rhs,
			proteins: self.proteins * rhs,
		}
	}
}

impl Mul<Nutrients> for f32 {
	type Output = Nutrients;

	fn mul(self, rhs: Nutrients) -> Self::Output {
		rhs * self
	}
}