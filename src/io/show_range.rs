use jiff::{ToSpan as _, civil::Date};
use std::ops::Range;

/// Calculates the 7-day report range based on a given date.
/// The calculated range always starts on a Monday and covers a full week.
/// Specifically, it finds the Monday of the week *before* the week containing `today`.
/// The final range spans from that Monday to the following Sunday, inclusive.
pub fn calc_show_range(today: Date) -> Range<Date> {
    let start_day = last_monday_before(today);
    let end_day = start_day.clone() + 7.days();

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
