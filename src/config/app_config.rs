use super::Opts;
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
    pub offset_hours: i32,
}

/// The concrete configuration logic that transforms `Opts` into `AppConfig`.
/// This function contains the business logic for setting default values and calculating
/// derived configuration, such as the `show_range`.
pub fn configure(opts: Opts) -> anyhow::Result<AppConfig> {
    AppConfig {
        calendar_path: opts.calendar_path.unwrap_or("basic.ics".into()),
        show_range: opts.date.unwrap_or_else(|| Zoned::now().date()).pipe(calc_show_range),
        offset_hours: opts.offset_hours.unwrap_or(6),
    }
    .pipe(Ok)
}

/// Calculates the 7-day report range based on a given date.
/// The calculated range always starts on a Monday and covers a full week.
/// Specifically, it finds the Monday of the week *before* the week containing `today`.
/// The final range spans from that Monday to the following Sunday, inclusive.
pub fn calc_show_range(today: Date) -> Range<Date> {
    let start_day = last_monday_before(today);
    let end_day = start_day + 7.days();

    (start_day..end_day).into()
}

/// Helper function to find the most recent Monday that occurred before the given `day`.
/// If the given `day` is a Monday, it will return the Monday of the previous week.
fn last_monday_before(day: Date) -> Date {
    let diff_days = match day.weekday().to_monday_zero_offset() {
        0 => 7.days(), // If it's already Monday, go back a full week.
        x => x.days(), // Otherwise, go back `x` days to get to the previous Monday.
    };
    day - diff_days
}
