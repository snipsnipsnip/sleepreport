use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::{io::BufRead, ops::Range};

pub trait LoadSleeps {
    type Error;

    fn load_sleeps(
        &self,
        ical: impl BufRead,
    ) -> impl Iterator<Item = Result<Range<DateTime<Utc>>, Self::Error>>;
}

pub trait CountSleeps {
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> impl SleepQuery;
}

pub trait SleepQuery {
    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate>;
    fn is_sleeping_at(&self, d: i64, hr: i32) -> bool;
}

pub trait ReportSleeps {
    type SleepReport;

    fn report_sleeps(&self, query: impl SleepQuery) -> Self::SleepReport;
}
