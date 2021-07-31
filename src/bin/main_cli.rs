use fooddb::{FoodDB, FoodID, FoodQuantity};
use std::io;
use std::io::prelude::*;
use hashbrown::HashMap;
use std::iter::FromIterator;
use std::array::IntoIter;
use std::convert::TryFrom;

// Structure across all app modes.
struct AppState {
	result_stack: Vec<String>,
	food_db: FoodDB,
	quit: bool,
}

fn main() {
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
	food_db.reindex();
	food_db.save("food_db.fdb");

	let mut app_state = AppState {
		result_stack: vec![],
		food_db,
		quit: false
	};

	while !&app_state.quit {
		main_menu(&mut app_state);
	}
}

fn main_menu(app_state: &mut AppState) {
	// Until we have hashmap macros...
	let main_menu_options = HashMap::<char, &str>::from_iter(IntoIter::new([
		('n', "New Food"),
		('s', "Search Food"),
		('q', "Quit"),
	]));
	match show_map_menu(
		"Please choose an operation.",
		main_menu_options,
		None
	) {
		'q' => { app_state.quit = true; },
		's' => { search_food_menu(app_state); },
		_ => {}
	};
}

fn search_food_menu(app_state: &AppState) -> FoodID {
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let mut buffer = String::new();

	println!("Enter (partial) food name: ");
	if let Ok(bytes_read) = stdin.read_line(&mut buffer) {
		if bytes_read == 0 {
			panic!("Failed to read from STDIN.");
		}
		println!("Matching...");
		let matches:Vec<(FoodID, String)> = app_state.food_db.get_autocomplete_suggestions(buffer.trim().to_string());
		let (food_ids, food_names): (Vec<FoodID>, Vec<String>) = matches.iter().map(|(fid, fname)|{ (fid, fname.clone()) }).unzip();
		return food_ids[show_list_menu("Best Matches:", food_names) as usize];
	} else {
		panic!("Unable to read from STDIN.");
	}
}

fn confirm(prompt: &str) -> bool {
	let options = HashMap::<char, &str>::from_iter(IntoIter::new([('y', "Yes"), ('n', "No")]));
	show_map_menu(prompt, options, None) == 'y'
}

fn show_list_menu(prompt: &str, options: Vec<String>) -> u32 {
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let mut buffer = String::new();
	let mut choice: u32 = 0;

	while choice == 0 {
		println!("{}", &prompt);
		for (ch, desc) in options.iter().enumerate() {
			println!("{}: {}", ch+1, desc);
		}

		if let Ok(bytes_read) = stdin.read_line(&mut buffer) {
			if let Ok(num) = buffer.parse::<u32>() {
				if num >= 1 && num <= options.len() as u32 {
					choice = num;
				}
			}
		}

		if choice == 0 { // Invalid selection.
			println!("Please choose an item in the range of 1 to {}", &options.len()+1);
		}
	}

	choice-1
}

fn show_map_menu(prompt: &str, options: HashMap<char, &str>, default:Option<char>) -> char {
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	//let stdout = io::stdout();
	let mut choice:Option<char> = None;
	let mut buffer = String::new();

	while choice.is_none() {
		println!("{}", &prompt);
		for (ch, desc) in options.iter() {
			println!("{}: {}", ch, desc);
		}
		if let Some(def) = default {
			println!("Default: {}", def);
		}

		if let Ok(bytes_read) = stdin.read_line(&mut buffer) {
			if bytes_read == 0 {
				// END OF STREAM!
				// Failure.
			} else if buffer.len() == 0 {

			} else {
				let user_input = buffer.chars().nth(0).expect("Buffer len greater than zero must be true.");
				// Did user hit the default?
				if user_input == '\n' && default.is_some() {
					choice = default;
				} else if options.contains_key(&user_input) {
					choice = Some(user_input);
				} else {
					println!("Sorry, {} is an invalid selection.", &user_input);
					choice = None;
				}
				// Mark the bytes as consumed.
				stdin.consume(bytes_read);
				buffer.clear();
			}
		}
	}

	choice.unwrap()
}