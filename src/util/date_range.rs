use crate::util::RangeLike;
use jiff::{ToSpan as _, civil::Date};

pub trait DateRangeExt: RangeLike<Item = Date> {
    fn iter_days(&self) -> impl Iterator<Item = Date> {
        std::iter::successors(Some(*self.start()), move |d| {
            d.checked_add(1.days()).ok().filter(move |d| d < self.end())
        })
    }

    fn num_days(&self) -> i32 {
        self.delta().get_days()
    }
}

impl<T: RangeLike<Item = Date>> DateRangeExt for T {}
