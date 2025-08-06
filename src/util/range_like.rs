use std::ops::{Range, Sub};

pub trait RangeLike {
    type Item;

    fn start(&self) -> &Self::Item;
    fn end(&self) -> &Self::Item;

    fn fmap<T>(&self, f: impl Fn(&Self::Item) -> T) -> Range<T> {
        f(self.start())..f(self.end())
    }

    fn intersects(&self, other: &impl RangeLike<Item = Self::Item>) -> bool
    where
        Self::Item: PartialOrd,
    {
        self.start() <= other.start() && other.start() < self.end()
            || self.start() <= other.end() && other.end() < self.end()
    }

    fn delta(&self) -> <Self::Item as Sub>::Output
    where
        Self::Item: Copy,
        Self::Item: Sub<Self::Item>,
    {
        *self.end() - *self.start()
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
