use crate::{app, sleep::SleepTable};
use chrono::{DateTime, Utc};
use std::ops::Range;

/// A counter for sleep data.
pub struct SleepCounter {
    range: Range<DateTime<Utc>>,
}

impl SleepCounter {
    /// Creates a new `SleepCounter`.
    pub fn new(range: Range<DateTime<Utc>>) -> Self {
        Self { range }
    }
}
impl app::CountSleeps for SleepCounter {
    /// Counts the sleeps in the given iterator.
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> impl app::SleepQuery {
        let mut table = SleepTable::new(self.range);
        for sleep in sleeps {
            table.eat(sleep);
        }
        table
    }
}
