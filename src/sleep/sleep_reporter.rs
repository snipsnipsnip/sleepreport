use crate::app::{ReportSleeps, SleepQuery};
use crate::util::NaiveDateRange;
use chrono::{NaiveDate, TimeZone};
use std::iter;
use std::ops::Range;

#[derive(Debug)]
pub struct SleepReporter<'a, Tz> {
    offset_hours: i32,
    timezone: &'a Tz,
}

impl<'a, Tz> SleepReporter<'a, Tz> {
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

    fn report_sleeps(&self, table: impl SleepQuery) -> String {
        let day_range = table.day_range_in(self.timezone);
        let days = day_range.num_days();
        let header = iter::once(make_header(day_range));

        let lines = (0..24).map(move |i| {
            let hr = self.offset_hours + i;
            let cells = (0..days)
                .map(|d| {
                    if table.is_sleeping_at(d, hr) {
                        "睡眠"
                    } else {
                        ""
                    }
                })
                .collect::<Vec<_>>()
                .join("\t");

            format!("{:02}:00～{:02}:00\t{}", hr % 24, (hr + 1) % 24, cells)
        });

        header.chain(lines).collect::<Vec<_>>().join("\n")
    }
}

fn make_header(day_range: Range<NaiveDate>) -> String {
    let formatted_days = day_range
        .iter_days()
        .map(|d| d.format("%F").to_string())
        .collect::<Vec<_>>()
        .join("\t");
    format!("\t{formatted_days}")
}
