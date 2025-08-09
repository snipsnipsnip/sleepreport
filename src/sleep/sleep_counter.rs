use crate::{app, sleep::SleepTable, util::RangeExt as _};
use jiff::{Timestamp, ToSpan as _, civil::Date, tz::TimeZone};
use log::warn;
use std::ops::Range;

/// The concrete implementation of the `CountSleeps` trait.
/// This struct is responsible for creating a `SleepTable` that covers a specific time range.
pub struct SleepCounter {
    range: Range<Date>,
}

impl SleepCounter {
    /// Creates a new `SleepCounter` for a given time range.
    /// The range determines the boundaries of the resulting `SleepTable`.
    pub fn new(range: Range<Date>) -> Self {
        Self { range }
    }
}

impl app::CountSleeps for SleepCounter {
    type Error = jiff::Error;

    /// This method takes an iterator of sleep events and populates a `SleepTable`.
    /// The `SleepTable` is then returned as a `SleepQuery` trait object, ready for reporting.
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<Timestamp>>) -> Result<impl app::SleepQuery, Self::Error> {
        let tz = TimeZone::system();
        let Range { start, end } = self
            .range
            .try_fmap(move |d| tz.to_timestamp(d.to_datetime(Default::default())))?;

        // Count for an extra day to prepare for the offset hours
        let mut table = SleepTable::new(start..end + 24.hours())?;
        for sleep in sleeps {
            if let None = table.eat(sleep.clone()) {
                warn!("event skipped: {sleep:?}")
            }
        }

        Ok(table)
    }
}
