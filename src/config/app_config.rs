use super::Opts;
use crate::app::Hour;
use anyhow::Context as _;
use jiff::{ToSpan as _, Zoned, civil::Date};
use std::ops::Range;
use tap::Pipe as _;

/// Holds the fully resolved and validated configuration required for the application to run.
/// This is created from `Opts` and provides guaranteed values for all settings.
#[derive(Debug)]
pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<Date>,
    pub offset_hours: Hour,
}

/// The concrete configuration logic that transforms `Opts` into `AppConfig`.
/// This function contains the business logic for setting default values and calculating
/// derived configuration, such as the `show_range`.
pub fn configure(opts: Opts) -> anyhow::Result<AppConfig> {
    AppConfig {
        calendar_path: opts.calendar_path.unwrap_or("basic.ics".into()),
        show_range: calc_show_range(opts.date, opts.num_days),
        offset_hours: opts.offset_hours.unwrap_or(Hour::new(6)),
    }
    .pipe(Ok)
}

/// Calculates the 7-day report range based on a given date.
/// The calculated range always starts on a Monday and covers a full week by default.
/// Specifically, it finds the Monday of the week *before* the week containing `today`.
/// The final range spans from that Monday to the following Sunday, inclusive.
pub fn calc_show_range(day: Option<Date>, num_days: Option<i32>) -> Range<Date> {
    let today = day.unwrap_or_else(|| Zoned::now().date());
    let num_days = num_days.unwrap_or(-7);

    let [start_day, end_day] = {
        let mut r = [today, last_monday(today) + (num_days as i64).days()];
        r.sort();
        r
    };

    start_day..end_day + 1.day()
}

/// Helper function to find the most recent Monday that occurred before the given `day`.
/// If the given `day` is a Monday, it will return the Monday of the previous week.
fn last_monday(day: Date) -> Date {
    day - day.weekday().to_monday_zero_offset().days()
}
