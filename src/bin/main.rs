use chrono::{NaiveDate, Datelike};
use fooddb::FoodDB;
use iced::*;

const MAX_YEAR:i32 = 9999;

struct App {
	active_year: i32,
	active_month: i8,
	active_day: i8,
	food_db: FoodDB,

	decrement_day: button::State,
	increment_day: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	DecrementDay,
	IncrementDay,
	JumpToDate,
	AddFood,
}

impl Application for App {
	type Executor = executor::Default;
	type Message = Message;
	type Flags = ();

	fn new(_flags: ()) -> (App, Command<Message>) {
		let mut app = App {
			active_year: 2000,
			active_month: 0,
			active_day: 0,
			food_db: FoodDB::new(),
			decrement_day: button::State::new(),
			increment_day: button::State::new()
		};
		(app, Command::none())
	}

	fn title(&self) -> String {
		String::from("Menu - Iced")
	}

	fn view(&mut self) -> Element<Message> {
		Container::new(
			Column::new()
				.push(
					// Top-row date picker and settings wheel.
					Row::new()
						.push(
							Button::new(&mut self.decrement_day, Text::new("<"))
								.on_press(Message::DecrementDay)
						)
						.push(
							Text::new(format!("{}/{}/{}", self.active_year, self.active_month, self.active_day)).size(30)
						)
						.push(
							Button::new(&mut self.increment_day, Text::new(">"))
								.on_press(Message::IncrementDay)
						)
				)
				.push(
					Text::new("Breakfast").size(50),
				)
				.push(
					Text::new("Second Breakfast").size(50),
				)
				.push(
					Text::new("Lunch").size(50),
				)
				.push(
					Text::new("Snack").size(50),
				)
				.push(
					Text::new("Dinner").size(50),
				)
		)
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into()

	}

	fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
		match message {
			Message::DecrementDay => {
				//self.value += 1;
			},
			Message::IncrementDay => {
				//self.value -= 1;
			},
			Message::JumpToDate => {

			},
			Message::AddFood => {}
		}

		Command::none()
	}
}

pub fn main() -> iced::Result {
	App::run(Settings::default())
}