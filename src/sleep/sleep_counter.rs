use crate::{
    app::{CountSleeps, Hour, SleepQuery},
    sleep::SleepTable,
    util::RangeExt as _,
};
use jiff::{Timestamp, ToSpan as _, civil::Date, tz::TimeZone};
use log::warn;
use std::ops::Range;
use tap::Pipe as _;

/// The concrete implementation of the `CountSleeps` trait.
/// This struct is responsible for creating a `SleepTable` that covers a specific time range.
pub struct SleepCounter {
    count_range: Range<Date>,
    offset_hours: Hour,
    timezone: TimeZone,
}

pub trait SleepCounterConfig {
    fn count_range(&self) -> Range<Date>;
    fn offset_hours(&self) -> Hour;
    fn timezone(&self) -> TimeZone;
}

impl SleepCounter {
    /// Creates a new `SleepCounter` for a given time range.
    /// The range determines the boundaries of the resulting `SleepTable`.
    pub fn new(config: &impl SleepCounterConfig) -> Self {
        Self {
            count_range: config.count_range(),
            offset_hours: config.offset_hours(),
            timezone: config.timezone(),
        }
    }
}

impl CountSleeps for SleepCounter {
    type Error = jiff::Error;

    /// This method takes an iterator of sleep events and populates a `SleepTable`.
    /// The `SleepTable` is then returned as a `SleepQuery` trait object, ready for reporting.
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<Timestamp>>) -> Result<impl SleepQuery, Self::Error> {
        let Range { start, end } = self
            .count_range
            .try_fmap(move |d| self.timezone.to_timestamp(d.to_datetime(Default::default())))?;
        let time_range = start..end + self.offset_hours.hours();

        let mut table = SleepTable::new(time_range)?;
        for sleep in sleeps {
            if table.eat(sleep.clone()).is_none() {
                warn!("event skipped: {sleep:?}")
            }
        }

        Ok(table)
    }
}
