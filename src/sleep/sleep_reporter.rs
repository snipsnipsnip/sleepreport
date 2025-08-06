use crate::app::{ReportSleeps, SleepQuery};
use crate::util::{NaiveDateRange, RangeLike};
use chrono::{DateTime, Days, NaiveDate, NaiveTime, TimeDelta, TimeZone, Utc};
use std::iter;
use std::ops::Range;

/// The concrete implementation of the `ReportSleeps` trait.
/// This struct generates a tab-separated string representation of the sleep data.
#[derive(Debug)]
pub struct SleepReporter<'a, Tz> {
    offset_hours: i32,
    timezone: &'a Tz,
}

impl<'a, Tz> SleepReporter<'a, Tz> {
    /// Creates a new `SleepReporter`.
    /// `offset_hours` is used to rotate the 24-hour clock in the report.
    /// `timezone` is used to correctly display the date range in the header.
    pub fn new(offset_hours: i32, timezone: &'a Tz) -> Self {
        assert!((0..24).contains(&offset_hours));
        Self {
            offset_hours,
            timezone,
        }
    }
}

impl<Tz> ReportSleeps for SleepReporter<'_, Tz>
where
    Tz: TimeZone,
{
    type SleepReport = String;

    /// Generates a multi-line string report from a `SleepQuery` object.
    /// The report has a header row with dates and subsequent rows for each hour,
    /// showing "睡眠" if sleep was recorded for that hour.
    fn report_sleeps(&self, table: impl SleepQuery) -> String {
        let day_range = table.fmap(move |d| d.with_timezone(self.timezone).date_naive());
        let days = day_range.num_days();
        let header = iter::once(make_header(day_range));

        let lines = (0..24).map(move |i| {
            let hr = self.offset_hours + i;
            let cells = (0..days)
                .map(|d| show_cell(&table, hr, d))
                .collect::<Vec<_>>()
                .join("\t");

            format!("{:02}:00～{:02}:00\t{}", hr % 24, (hr + 1) % 24, cells)
        });

        header.chain(lines).collect::<Vec<_>>().join("\n")
    }
}

fn show_cell(table: &impl SleepQuery, hr: i32, d: i64) -> &'static str {
    let hour = *table.start() + (TimeDelta::days(d) + TimeDelta::hours(hr as i64));
    match table.is_sleeping_at(&hour) {
        Some(true) => "睡眠",
        Some(false) => "",
        None => "？",
    }
}

/// Creates the header row for the sleep report, containing tab-separated dates.
fn make_header(day_range: Range<NaiveDate>) -> String {
    let formatted_days = day_range
        .iter_days()
        .map(|d| d.format("%F").to_string())
        .collect::<Vec<_>>()
        .join("\t");
    format!("\t{formatted_days}")
}
