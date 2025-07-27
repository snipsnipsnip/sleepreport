use chrono::NaiveDate;
use std::ops::Range;

pub trait NaiveDateRange {
    fn start(&self) -> &NaiveDate;
    fn end(&self) -> &NaiveDate;

    fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        self.start().iter_days().take_while(|d| d < self.end())
    }
}

impl NaiveDateRange for Range<NaiveDate> {
    fn start(&self) -> &NaiveDate {
        &self.start
    }

    fn end(&self) -> &NaiveDate {
        &self.end
    }
}
