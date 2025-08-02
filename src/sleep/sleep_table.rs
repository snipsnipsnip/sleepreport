use crate::{sleep::SleepQuery, util::RangeLike};
use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use std::ops::Range;

#[derive(Debug)]
pub struct SleepTable {
    slept: Vec<bool>,
    range: Range<DateTime<Utc>>,
}

impl SleepTable {
    pub fn new(today: DateTime<impl TimeZone>) -> Option<Self> {
        let last_monday = last_monday_before(&today);
        let start = last_monday.with_time(NaiveTime::MIN).single()?;
        let end = start.clone() + Days::new(7) + Duration::hours(23);

        Self::during(start.to_utc()..end.to_utc()).into()
    }

    pub fn during(range: Range<DateTime<Utc>>) -> Self {
        Self {
            slept: vec![false; (range.end - range.start).num_hours() as usize],
            range: range,
        }
    }

    pub fn eat(&mut self, sleep: Range<DateTime<Utc>>) -> Option<()> {
        let slept_indices = self.map_indices(sleep)?;
        self.slept[slept_indices].fill(true);

        Some(())
    }

    fn map_indices(&self, sleep: Range<DateTime<Utc>>) -> Option<Range<usize>> {
        self.range.intersects(&sleep).then_some(())?;

        let sleep_start_index = (sleep.start - self.range.start).num_hours() as usize;
        let sleep_length_mins = (sleep.end - sleep.start).num_minutes();
        let sleep_length_hours = ((sleep_length_mins + 30) as f32 / 60f32).round() as usize;
        let slept_indices = sleep_start_index..sleep_start_index + sleep_length_hours;

        slept_indices.into()
    }
}

impl SleepQuery for SleepTable {
    fn is_sleeping_at(&self, d: i64, hr: i32) -> bool {
        self.slept[(d * 24) as usize + hr as usize]
    }

    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate> {
        self.range.fmap(|d| d.with_timezone(tz).date_naive())
    }
}

fn last_monday_before<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day.clone() - Days::new(diff_days as u64)
}
