use crate::{app, sleep::SleepTable};
use chrono::{DateTime, Utc};
use std::ops::Range;

/// The concrete implementation of the `CountSleeps` trait.
/// This struct is responsible for creating a `SleepTable` that covers a specific time range.
pub struct SleepCounter {
    range: Range<DateTime<Utc>>,
}

impl SleepCounter {
    /// Creates a new `SleepCounter` for a given time range.
    /// The range determines the boundaries of the resulting `SleepTable`.
    pub fn new(range: Range<DateTime<Utc>>) -> Self {
        Self { range }
    }
}

impl app::CountSleeps for SleepCounter {
    /// This method takes an iterator of sleep events and populates a `SleepTable`.
    /// The `SleepTable` is then returned as a `SleepQuery` trait object, ready for reporting.
    fn count_sleeps(
        self,
        sleeps: impl Iterator<Item = Range<DateTime<Utc>>>,
    ) -> impl app::SleepQuery {
        let mut table = SleepTable::new(self.range);
        for sleep in sleeps {
            table.eat(sleep);
        }
        table
    }
}
