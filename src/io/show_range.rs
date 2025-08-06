use chrono::{DateTime, Datelike, Days, Duration, NaiveTime, TimeZone, Utc};
use std::ops::Range;

/// Calculates the date range to show in the report.
/// The range is from the Monday of the previous week to the Sunday of the previous week.
pub fn calc_show_range(today: DateTime<impl TimeZone>) -> Option<Range<DateTime<Utc>>> {
    let last_monday = last_monday_before(today);
    let start = last_monday.with_time(NaiveTime::MIN).single()?;
    let end = start.clone() + Days::new(7) + Duration::hours(23);

    (start.to_utc()..end.to_utc()).into()
}

/// Calculates the last Monday before the given date.
fn last_monday_before<Tz: TimeZone>(day: DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day - Days::new(diff_days as u64)
}
