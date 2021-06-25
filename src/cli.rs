
use fooddb::{FoodDB};

fn main() {
	let mut db = FoodDB::new();
	let new_food_id = db.new_food();
	//println!("New food id: {}", new_food_id);
	db.save("test.fdb");
}