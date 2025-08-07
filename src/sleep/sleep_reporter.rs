use crate::app::{ReportSleeps, SleepQuery};
use crate::util::{DateRangeExt as _, IterExt as _};
use jiff::ToSpan as _;
use jiff::civil::Date;
use std::iter;
use std::ops::Range;

/// The concrete implementation of the `ReportSleeps` trait.
/// This struct generates a tab-separated string representation of the sleep data.
#[derive(Debug)]
pub struct SleepReporter {
    report_range: Range<Date>,
    offset_hours: i32,
}

pub trait SleepReporterConfig {
    fn report_range(&self) -> Range<Date>;
    fn offset_hours(&self) -> i32;
}

impl SleepReporter {
    /// Creates a new `SleepReporter`.
    /// `offset_hours` is used to rotate the 24-hour clock in the report.
    pub fn new(config: &impl SleepReporterConfig) -> Self {
        assert!((0..24).contains(&config.offset_hours()));
        Self {
            report_range: config.report_range(),
            offset_hours: config.offset_hours(),
        }
    }
}

impl ReportSleeps for SleepReporter {
    type SleepReport = String;

    /// Generates a multi-line string report from a `SleepQuery` object.
    /// The report has a header row with dates and subsequent rows for each hour,
    /// showing "睡眠" if sleep was recorded for that hour.
    fn report_sleeps(&self, table: impl SleepQuery) -> String {
        let days = self.report_range.num_days();
        let header = iter::once(make_header(&self.report_range));

        let lines = (0..24).map(move |i| {
            let hr = self.offset_hours + i;
            let cells = (0..days).map(|d| show_cell(&table, hr, d)).into_vec().join("\t");

            format!("{:02}:00～{:02}:00\t{}", hr % 24, (hr + 1) % 24, cells)
        });

        header.chain(lines).into_vec().join("\n")
    }
}

fn show_cell(table: &impl SleepQuery, hr: i32, d: i32) -> &'static str {
    let hour = *table.start() + (d * 24 + hr).hours();
    match table.is_sleeping_at(hour) {
        Some(true) => "睡眠",
        Some(false) => "",
        None => "？",
    }
}

/// Creates the header row for the sleep report, containing tab-separated dates.
fn make_header(day_range: &Range<Date>) -> String {
    let formatted_days = day_range.iter_days().map(|d| d.to_string()).into_vec().join("\t");
    format!("\t{formatted_days}")
}
