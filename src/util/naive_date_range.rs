use crate::util::RangeLike;
use chrono::NaiveDate;
use std::ops::Range;

pub trait NaiveDateRange: RangeLike<Item = NaiveDate> {
    fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        self.start().iter_days().take_while(move |d| d < self.end())
    }

    fn num_days(&self) -> i64 {
        self.delta().num_days()
    }
}

impl NaiveDateRange for Range<NaiveDate> {}
