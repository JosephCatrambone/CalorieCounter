use egui::Ui;
use std::str::FromStr;
use chrono::NaiveDate;

/*
DatePicker:
  /\   /\  /\
[YYYY][MM][DD]
  \/   \/  \/
----------------
[  ] [  ] [  ] [  ] [  ] [ 1] [ 2]
[ 3] [ 4] [ 5] [ 6] [ 7] [ 8] [ 9]
[10] [11] [12] [13] [14] [15] [16]
[17] [18] [19] [20] [21] [22] [23]
[24] [25] [26] [27] [28] [29] [30]
[31] [  ] [  ] [  ] [  ] [  ] [  ]
*/

pub struct DatePicker {
	year: i32,
	month: u8, // 1 through 12
	day: u8, // 1 through 31
}

impl DatePicker {
	pub fn new() -> Self {
		DatePicker {
			year: 2000,
			month: 1,
			day: 1
		}
	}

	pub fn update(&mut self, ui:&mut Ui) {
		ui.vertical(|ui|{
			// Draw the top half -- text input areas with up/down arrows and editable text.
			ui.horizontal(|ui|{
				// Draw year input.
				ui.vertical_centered(|ui|{
					let mut year_str = format!("{}", self.year);
					if ui.button("/\\").clicked() {
						self.year += 1;
					};
					//ui.label(&year_str);
					if ui.text_edit_singleline(&mut year_str).lost_focus() {
						self.year = i32::from_str(&year_str).unwrap_or(self.year);
					};
					if ui.button("\\/").clicked() {
						self.year -= 1;
					}
				});

				// Draw month input.
				ui.vertical_centered(|ui|{
					let mut month_str = format!("{}", self.month);
					if ui.button("/\\").clicked() {
						self.month += 1;
					};
					if ui.text_edit_singleline(&mut month_str).lost_focus() {
						self.month = u8::from_str(&month_str).unwrap_or(self.month);
					}
					if ui.button("\\/").clicked() {
						self.month -= 1;
					}

					// Bounds checking and wrapping.
					// TODO: Do we want to wrap years when this happens?
					if self.month > 12 {
						self.month = 1;
					} else if self.month < 1 {
						self.month = 12;
					}
				});

				// Draw day input.  We need to do some math to get the days in this month during the bounds check.
				ui.vertical_centered(|ui|{
					let mut day_str = format!("{}", self.day);
					if ui.button("/\\").clicked() {
						self.day += 1;
					};
					if ui.text_edit_singleline(&mut day_str).lost_focus() {
						self.day = u8::from_str(&day_str).unwrap_or(self.day);
					}
					if ui.button("\\/").clicked() {
						self.day -= 1;
					}

					// Bounds checking and wrapping.
					// TODO: Do we want to wrap years when this happens?
					if self.day < 1 || self.day > 26 {
						let days_in_month = days_in_month(self.year, self.month as u32);
						if self.day < 1 {
							self.day = days_in_month as u8;
						} else if self.day > days_in_month as u8 {
							self.day = 1;
						}
					}
				});
			});

			// Draw the bottom-half, the day picker.

		});
	}
}

// From Boussif Asma
// Takes the one-indexed month and 0-indexed year and returns the number of days in the month.
pub fn days_in_month(year: i32, month: u32) -> i64 {
	NaiveDate::from_ymd(
		match month {
			12 => year + 1,
			_ => year,
		},
		match month {
			12 => 1,
			_ => month + 1,
		},
		1,
	)
		.signed_duration_since(NaiveDate::from_ymd(year, month, 1))
		.num_days()
}