
mod date_picker;

use macroquad as mq;
use macroquad::prelude::*;
use fooddb::FoodDB;
use chrono::{NaiveDate, Datelike};
use egui::Frame;

const MAX_YEAR:i32 = 9999;

struct AppState {
	date_picker: date_picker::DatePicker,
	food_db: FoodDB,
}

#[macroquad::main("Calorie Counter")]
async fn main() {
	set_pc_assets_folder("assets");

	//let mut food_db = FoodDB::from_string(mq::file::load_string(""));
	let mut food_db = FoodDB::default();
	{
		let mut fid = food_db.new_food();
		fid.name = "TestFood".to_string();
		fid.manufacturer = "TestManufacturer".to_string();
		fid.volume_of_100g = 100.0;
		fid.servings_in_100g = 1.0;
		fid.nutrition.calories = 0;
		fid.nutrition.carbohydrates = 0.0;
		fid.nutrition.fats = 0.0;
		fid.nutrition.proteins = 0.0;
	}
	//let mut food_db = FoodDB::from_string(include_str!("../assets/default.fdb")).unwrap();
	food_db.reindex();
	food_db.save("food_db.fdb");

	let mut app = AppState {
		date_picker: date_picker::DatePicker::new(),
		food_db
	};

	loop {
		clear_background(WHITE);

		// Process keys, mouse etc.

		// Configure and display UI.
		egui_macroquad::ui(|egui_ctx| {
			//egui::Window::new("Calorie Counter").show(egui_ctx, |ui|{});
			egui::CentralPanel::default().show(egui_ctx, |ui| {
					//egui::menu::menu(ui, "Title!", |ui|{

					//ui.vertical(|ui|{
						//ui.label(format!("Food Diary for {}/{}/{}", &app.active_year, &app.active_month, &app.active_day));

					//});
					ui.allocate_ui(egui::Vec2::new(480f32, 480f32), |ui|{
					ui.heading("Calorie Counter");
					ui.label("Before date picker.");
					app.date_picker.update(ui);
					ui.label("After date picker.");
					});
				});
		});

		// Draw things before egui

		// Draw the UI.
		egui_macroquad::draw();

		// Draw things after egui

		// Advance.
		next_frame().await;
	}
}

/// Given a year and a month (with Jan = 1), return the number of days in the given month.
fn days_in_month(year:i32, month:u32) -> u8 {
	let next_date_year = if month == 12 {
		year+1
	} else {
		year
	};
	let next_date_month = if month == 12 {
		1
	} else {
		month + 1
	};
	NaiveDate::from_ymd(next_date_year, next_date_month, 1).signed_duration_since(NaiveDate::from_ymd(year, month, 1)).num_days() as u8
}