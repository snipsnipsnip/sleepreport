use crate::util::RangeLike;
use chrono::{NaiveDate, TimeDelta};
use std::ops::Range;

pub trait NaiveDateRange: RangeLike<Item = NaiveDate> {
    fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        self.start().iter_days().take_while(|d| d < self.end())
    }

    fn num_days(&self) -> i64 {
        self.delta().num_days()
    }

    fn delta(&self) -> TimeDelta {
        *self.end() - *self.start()
    }
}

impl NaiveDateRange for Range<NaiveDate> {}
