use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::ops::Range;

pub trait LoadSleeps {
    fn load_sleeps(self) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>>;
}

pub trait SleepQuery {
    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate>;
    fn is_sleeping_at(&self, d: i64, hr: i32) -> bool;
}
