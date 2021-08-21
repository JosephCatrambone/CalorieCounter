
use macroquad as mq;
use macroquad::prelude::*;
use fooddb::FoodDB;
use chrono::{NaiveDate, Datelike};

const MAX_YEAR:i32 = 9999;

struct AppState {
	date_input: String,
	active_year: i32,
	active_month: u32,
	active_day: u32,
	food_db: FoodDB,
}

#[macroquad::main("Calorie Counter")]
async fn main() {
	set_pc_assets_folder("assets");

	//let mut food_db = FoodDB::from_string(mq::file::load_string(""));
	/*
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
	*/
	let mut food_db = FoodDB::from_string(include_str!("assets/default.fdb")).unwrap();
	food_db.reindex();
	food_db.save("food_db.fdb");

	let mut app = AppState {
		date_input: "".to_string(),
		active_year: 1970,
		active_month: 1,
		active_day: 1,
		food_db
	};

	loop {
		clear_background(WHITE);

		// Process keys, mouse etc.

		// Configure and display UI.
		egui_macroquad::ui(|egui_ctx| {
			egui::CentralPanel::default()
			//egui::Window::new("Calorie Counter")
				.show(egui_ctx, |ui| {
					//egui::menu::menu(ui, "Title!", |ui|{

					//ui.heading("My egui Application");
					ui.label(format!("Food Diary for {}/{}/{}", &app.active_year, &app.active_month, &app.active_day));

					/*
					ui.horizontal(|ui| {
						ui.label("Your name: ");
						//ui.text_edit_singleline(&mut name);
					});
					*/

					// Pick Date.
					ui.horizontal(|ui|{
						ui.text_edit_singleline(&mut app.date_input);
						if(app.date_input)
						/*
						ui.add(egui::Slider::new(&mut app.active_year, 1970..=MAX_YEAR).text("Year"));
						if ui.button("+").clicked() { app.active_year += 1; }
						if ui.button("-").clicked() { app.active_year -= 1; } // Should address underflow.

						ui.add(egui::Slider::new(&mut app.active_month, 1..=12).text("Month"));
						if ui.button("+").clicked() { app.active_month += 1; }
						if ui.button("-").clicked() { app.active_month -= 1; } // Should address underflow.

						let days = days_in_month(app.active_year, app.active_month) as u32;
						ui.add(egui::Slider::new(&mut app.active_day, 1..=days).text("Day"));
						if ui.button("+").clicked() { app.active_day += 1; }
						if ui.button("-").clicked() { app.active_day -= 1; } // Should address underflow.
						*/
					});

					//ui.label(format!("Hello '{}', age {}", name, age));

					//});
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

fn date_picker(ui: &mut egui::Ui, app_state: &mut AppState) {

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