use chrono::prelude::*;
use datetimeutils::{days_in_month, month_from_index, month_string};
use slint::{SharedString, VecModel};
use std::rc::Rc;
slint::include_modules!();

/**
 * Function to calculate the number of days from the previous month that
 * should be displayed at the start of the current month's calendar grid.
 * The calculation is based on the weekday of the first day of the current month.
 */
fn get_last_days_of_prev_month(weekday: Weekday) -> u32 {
    match weekday {
        Weekday::Sun => 0, // If the first day is Sunday, no previous month days are needed.
        Weekday::Mon => 1, // If the first day is Monday, 1 day from the previous month is needed.
        Weekday::Tue => 2, // Continue similarly for other weekdays.
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

/**
 * Function to calculate the number of days from the next month that
 * should be displayed at the end of the current month's calendar grid.
 * The calculation is based on the weekday of the last day of the current month.
 */
fn get_first_days_of_next_month(weekday: Weekday) -> u32 {
    match weekday {
        Weekday::Sun => 6, // If the last day is Sunday, 6 days from the next month are needed.
        Weekday::Mon => 5, // If the last day is Monday, 5 days from the next month are needed.
        Weekday::Tue => 4, // Continue similarly for other weekdays.
        Weekday::Wed => 3,
        Weekday::Thu => 2,
        Weekday::Fri => 1,
        Weekday::Sat => 0, // If the last day is Saturday, no next month days are needed.
    }
}

/**
 * Helper function to insert a sequence of days into the calendar grid.
 *
 * Parameters:
 * - `boxes`: The vector model that holds the calendar days to be displayed.
 * - `days`: The number of days to insert.
 * - `start_day`: The starting day number for the sequence.
 */
fn insert_days(boxes: Rc<VecModel<NewBox>>, days: u32, start_day: i32) {
    for i in 0..days {
        boxes.insert(
            i as usize,
            NewBox {
                visible: true,
                day: start_day + i as i32, // Sets the day to start.
            },
        );
    }
}

/**
 * Function to determine the weekday of a given date.
 *
 * Parameters:
 * - `year`: The year part of the date.
 * - `month`: The month part of the date.
 * - `day`: The day part of the date.
 *
 * Returns:
 * - The `Weekday` corresponding to the provided date.
 */
fn get_week_day(year: u64, month: u32, day: u32) -> Weekday {
    let date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
    date.weekday()
}

/**
 * Function to get the current year as a `u64`.
 */
fn current_year() -> u64 {
    Utc::now().year() as u64
}

/**
 * Function to get the current month as a `u32`.
 */
fn current_month() -> u32 {
    Utc::now().month()
}

/**
 * Function to get the current day as a `u32`.
 */
fn current_day() -> u32 {
    Utc::now().day()
}

/**
 * Function to calculate the number of days in a given month of a given year.
 *
 * Parameters:
 * - `year`: The year for which to calculate the days.
 * - `month`: The month for which to calculate the days.
 *
 * Returns:
 * - The number of days in the specified year and month.
 */
fn generate_month(year: u64, month: u32) -> u64 {
    let current_month = month_from_index(month as u64);
    days_in_month(year, current_month.unwrap())
}

/**
 * Main function to load the calendar data for the specified month and year.
 *
 * This function populates the calendar grid with days from the previous month,
 * the current month, and the next month to ensure the grid is fully filled.
 *
 * Parameters:
 * - `boxes`: The vector model that holds the calendar days to be displayed.
 * - `year`: The year for which the calendar is being generated.
 * - `month`: The month for which the calendar is being generated.
 */
fn load_calendar(boxes: Rc<VecModel<NewBox>>, year: u64, month: u32) {
    // Start by handling the days from the next month that will be shown at the end of the current month's grid.
    let days_of_month = generate_month(year, month);
    let last_weekday_of_month = get_week_day(year, month, days_of_month as u32);
    let first_days_of_next_month = get_first_days_of_next_month(last_weekday_of_month);
    insert_days(boxes.clone(), first_days_of_next_month, 1);

    // Now handle the days of the current month.
    insert_days(boxes.clone(), days_of_month as u32, 1);

    // Finally, handle the days from the previous month that will be shown at the beginning of the current month's grid.
    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12) // Handle the case where the current month is January.
    } else {
        (year, month - 1) // Otherwise, simply subtract one month.
    };

    let first_weekday_of_month: Weekday = get_week_day(year, month, 1);
    let last_days_of_prev_month = get_last_days_of_prev_month(first_weekday_of_month);
    let days_of_prev_month = generate_month(prev_year, prev_month);
    let start_day = days_of_prev_month - last_days_of_prev_month as u64;

    insert_days(boxes, last_days_of_prev_month, start_day as i32 + 1);
}

/**
 * Function to run the calendar UI.
 *
 * This function is responsible for setting up the UI elements with the appropriate
 * month, year, and days, and handling user interactions such as moving to the next month.
 *
 * Parameters:
 * - `ui`: The main UI window.
 * - `boxes`: The vector model that holds the calendar days to be displayed.
 * - `year`: The year for which the calendar is being generated.
 * - `month`: The month for which the calendar is being generated.
 */
fn run_calendar(ui: &AppWindow, boxes: Rc<VecModel<NewBox>>, year: u64, month: u32) {
    let current_month = month_from_index(month as u64);

    load_calendar(boxes.clone(), year, month);

    let updated_month = month_string(current_month.unwrap());

    // Update the UI with the month name and year.
    ui.set_month(SharedString::from(updated_month));
    let year_str = format!(" {}", year.to_string());
    ui.set_year(SharedString::from(year_str));

    // Set the populated boxes model into the UI.
    ui.set_boxes(boxes.clone().into());
}

/**
 * Function to generate and return the list of weekdays for the UI.
 *
 * Returns:
 * - An `Rc<VecModel<Weekdays>>` containing the names of the weekdays.
 */
fn get_week_days() -> Rc<VecModel<Weekdays>> {
    let week_vec = vec![
        Weekdays {
            day: SharedString::from("Sunday"),
        },
        Weekdays {
            day: SharedString::from("Monday"),
        },
        Weekdays {
            day: SharedString::from("Tuesday"),
        },
        Weekdays {
            day: SharedString::from("Wednesday"),
        },
        Weekdays {
            day: SharedString::from("Thursday"),
        },
        Weekdays {
            day: SharedString::from("Friday"),
        },
        Weekdays {
            day: SharedString::from("Saturday"),
        },
    ];

    let weekdays = Rc::new(slint::VecModel::<Weekdays>::from(Vec::from(week_vec)));
    weekdays
}

/**
 * The main entry point of the application.
 *
 * This function sets up the UI, loads the initial calendar, and handles user interactions
 * such as navigating to the next month.
 *
 * Returns:
 * - A `Result` indicating whether the application started successfully or encountered an error.
 */
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?; // Initialize the UI.
    let boxes = Rc::new(slint::VecModel::<NewBox>::from(Vec::new()));

    let new_boxes = boxes.clone();

    let year = current_year(); // Get the current year.
    let mut month = current_month(); // Get the current month.
    let day = current_day(); // Get the current day.

    let weekday = get_week_day(year, month, day);
    let weekday_str: String = format!("Today is {}", weekday);
    println!("{}", weekday_str); // Print the current day of the week.

    ui.set_weekdays(get_week_days().clone().into()); // Set the weekday labels in the UI.

    run_calendar(&ui, new_boxes, year, month); // Load and display the current month's calendar.

    let ui_handle = ui.as_weak();
    ui.on_next_month(move || {
        let ui = ui_handle.unwrap();
        month += 1; // Move to the next month.
        let boxes = Rc::new(slint::VecModel::<NewBox>::from(Vec::new()));
        run_calendar(&ui, boxes, year, month); // Load and display the new month's calendar.
    });

    ui.run() // Start the UI event loop.
}
