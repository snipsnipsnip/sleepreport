use crate::{
    app::{CountSleeps, SleepQuery},
    sleep::SleepTable,
};
use chrono::{DateTime, Utc};
use std::ops::Range;

pub struct SleepCounter {
    range: Range<DateTime<Utc>>,
}

impl SleepCounter {
    pub fn new(range: Range<DateTime<Utc>>) -> Self {
        Self { range }
    }
}
impl CountSleeps for SleepCounter {
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> impl SleepQuery {
        let mut table = SleepTable::new(self.range);
        for sleep in sleeps {
            table.eat(sleep);
        }
        table
    }
}
