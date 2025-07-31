use chrono::NaiveDate;
use std::ops::Range;

pub trait RangeLike {
    type Item;
    fn start(&self) -> &Self::Item;
    fn end(&self) -> &Self::Item;

    fn fmap<T>(&self, f: impl Fn(&Self::Item) -> T) -> Range<T> {
        f(self.start())..f(self.end())
    }
}

pub trait NaiveDateRange: RangeLike<Item = NaiveDate> {
    fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        self.start().iter_days().take_while(|d| d < self.end())
    }
}

impl<T> RangeLike for Range<T> {
    type Item = T;

    fn start(&self) -> &T {
        &self.start
    }

    fn end(&self) -> &T {
        &self.end
    }
}

impl NaiveDateRange for Range<NaiveDate> {}
