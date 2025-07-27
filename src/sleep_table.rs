use crate::util::native_date_range::NaiveDateRange;
use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use std::{fmt::Display, ops::Range};

#[derive(Debug)]
pub struct SleepTable {
    range: Range<DateTime<Utc>>,
    sleeps: Vec<Range<DateTime<Utc>>>,
}

impl SleepTable {
    pub fn new(today: DateTime<impl TimeZone>) -> Option<Self> {
        let last_monday = last_monday_before(&today);
        let start = last_monday.with_time(NaiveTime::MIN).single()?;
        let end = start.clone() + Days::new(7) + Duration::hours(23);

        Some(Self::during(start.to_utc()..end.to_utc()))
    }

    pub fn during(range: Range<DateTime<Utc>>) -> Self {
        Self {
            range: range,
            sleeps: vec![],
        }
    }

    pub fn eat(&mut self, sleep: Range<DateTime<Utc>>) {
        if self.range.contains(&sleep.start) || self.range.contains(&sleep.end) {
            self.sleeps.push(sleep);
        }
    }

    pub fn build_csv_lines<Tz>(&self, offset_hours: i32, tz: &Tz) -> impl Iterator<Item = String>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        assert!((0..23).contains(&offset_hours));

        let slept = self.collect_sleeps();

        let local = fmap(&self.range, |d| d.with_timezone(tz));

        let day_range = fmap(&local, |d| d.date_naive());
        let header = [make_header(day_range)];

        let days = (local.end.clone() - &local.start).num_days();
        let lines = (0..24).map(move |i| {
            let hr = offset_hours + i;
            let cells = (0..days)
                .map(|d| {
                    if slept[(d * 24) as usize + hr as usize] {
                        "睡眠"
                    } else {
                        ""
                    }
                })
                .collect::<Vec<_>>()
                .join("\t");

            format!("{:02}:00～{:02}:00\t{}", hr % 24, (hr + 1) % 24, cells)
        });

        header.into_iter().chain(lines.into_iter())
    }

    fn collect_sleeps(&self) -> Vec<bool> {
        let mut slept = vec![false; (self.range.end - self.range.start).num_hours() as usize];

        for sleep in &self.sleeps {
            let sleep_start_index = (sleep.start - self.range.start).num_hours() as usize;
            let sleep_length_mins = (sleep.end - sleep.start).num_minutes();
            let sleep_length_hours = ((sleep_length_mins + 30) as f32 / 60f32).round() as usize;
            let slept_indices = sleep_start_index..sleep_start_index + sleep_length_hours;

            slept[slept_indices].fill(true);
            // (self.range.start + Duration::hours(hr)).with_timezone(tz)
        }

        slept
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

fn last_monday_before<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day.clone() - Days::new(diff_days as u64)
}

fn fmap<T, U>(range: &Range<T>, f: impl Fn(&T) -> U) -> Range<U> {
    f(&range.start)..f(&range.end)
}
