use chrono::{DateTime, Datelike, Days, Duration, NaiveTime, TimeZone, Utc};
use std::ops::Range;

/// Calculates the 7-day report range based on a given date.
/// The calculated range always starts on a Monday and covers a full week.
/// Specifically, it finds the Monday of the week *before* the week containing `today`.
/// The final range spans from that Monday to the following Sunday, inclusive.
pub fn calc_show_range(today: DateTime<impl TimeZone>) -> Option<Range<DateTime<Utc>>> {
    let last_monday = last_monday_before(today);
    let start = last_monday.with_time(NaiveTime::MIN).single()?;
    let end = start.clone() + Days::new(7) + Duration::hours(23);

    (start.to_utc()..end.to_utc()).into()
}

/// Helper function to find the most recent Monday that occurred before the given `day`.
/// If the given `day` is a Monday, it will return the Monday of the previous week.
fn last_monday_before<Tz: TimeZone>(day: DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7, // If it's already Monday, go back a full week.
        x => x, // Otherwise, go back `x` days to get to the previous Monday.
    };
    day - Days::new(diff_days as u64)
}
