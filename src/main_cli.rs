
use cursive::{Cursive, align, views::{Dialog, SelectView, TextView}};
use fooddb::{FoodDB};

fn main() {
	println!("Starting up!");

	let mut db = FoodDB::new();
	let new_food_id = db.new_food();
	//println!("New food id: {}", new_food_id);
	db.save("test.fdb");

	// Creates the cursive root - required for every application.
	let mut siv = cursive::default();

	// Creates a dialog with a single "Quit" button
	siv.add_layer(Dialog::around(TextView::new("Calorie Counter"))
		.title("Food Diary {Day}")
		.button("Quit", |s| s.quit()));

	// Starts the event loop.
	siv.run();
}

fn display_day_layer(siv: &mut Cursive, year: u32, month: u8, day: u8) {
	siv.pop_layer();
	siv.add_layer();
}

fn display_search_food(siv: &mut Cursive, food_db: &FoodDB) -> String {
	// Search Foods: Fo|______
	// +---------------------+
	// | Food A              |
	// | Food B              |
	// +---------------------+
	// [SUBMIT]

	let mut food_select_view = SelectView::new()
		.h_align(align::HAlign::Center)
		.autojump();



	siv.add_layer(

	);
}