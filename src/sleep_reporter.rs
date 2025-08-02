use crate::util::NaiveDateRange;
use chrono::{NaiveDate, TimeZone};
use std::iter;
use std::{fmt::Display, ops::Range};

#[derive(Debug)]
pub struct SleepReporter<T> {
    table: T,
}

pub trait SleepQuery {
    fn is_sleeping_at(&self, d: i64, hr: i32) -> bool;
    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate>;
}

impl<T: SleepQuery> SleepReporter<T> {
    pub fn new(table: T) -> Self {
        Self { table }
    }

    pub fn build_csv_lines<Tz>(&self, offset_hours: i32, tz: &Tz) -> impl Iterator<Item = String>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        assert!((0..24).contains(&offset_hours));

        let day_range = self.table.day_range_in(tz);
        let days = day_range.num_days();
        let header = iter::once(make_header(day_range));

        let lines = (0..24).map(move |i| {
            let hr = offset_hours + i;
            let cells = (0..days)
                .map(|d| {
                    if self.table.is_sleeping_at(d, hr) {
                        "睡眠"
                    } else {
                        ""
                    }
                })
                .collect::<Vec<_>>()
                .join("\t");

            format!("{:02}:00～{:02}:00\t{}", hr % 24, (hr + 1) % 24, cells)
        });

        header.chain(lines)
    }
}

fn make_header(day_range: Range<NaiveDate>) -> String {
    let formatted_days = day_range
        .iter_days()
        .map(|d| d.format("%F").to_string())
        .collect::<Vec<_>>()
        .join("\t");
    format!("\t{}", formatted_days)
}
