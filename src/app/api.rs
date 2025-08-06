use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::{io::BufRead, ops::Range};

/// A trait for loading sleep data from an iCalendar source.
pub trait LoadSleeps {
    type Error;

    /// Loads sleep data from the given iCalendar reader.
    fn load_sleeps(
        &self,
        ical: impl BufRead,
    ) -> impl Iterator<Item = Result<Range<DateTime<Utc>>, Self::Error>>;
}

/// A trait for fetching an iCalendar file.
pub trait FetchIcal {
    type Error;

    /// Fetches the iCalendar file.
    fn fetch_ical(&self) -> Result<impl BufRead, Self::Error>;
}

/// A trait for counting sleeps.
pub trait CountSleeps {
    /// Counts the sleeps in the given iterator.
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> impl SleepQuery;
}

/// A trait for querying sleep data.
pub trait SleepQuery {
    /// Returns the range of days in the given timezone.
    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate>;
    /// Returns whether the user is sleeping at the given time.
    fn is_sleeping_at(&self, d: i64, hr: i32) -> bool;
}

/// A trait for reporting sleep data.
pub trait ReportSleeps {
    type SleepReport;

    /// Reports the sleep data for the given query.
    fn report_sleeps(&self, query: impl SleepQuery) -> Self::SleepReport;
}
