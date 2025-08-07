use std::ops::{Range, Sub};

pub trait RangeLike {
    type Item;

    fn start(&self) -> &Self::Item;
    fn end(&self) -> &Self::Item;

    fn fmap<T>(&self, f: impl FnMut(&Self::Item) -> T) -> Range<T> {
        let mut f = f;
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
        Self::Item: Copy + Sub,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_eq;

    #[test]
    fn test_fmap() {
        let range = 5..10;
        let new_range = range.fmap(|x| x * 2);
        assert_eq!(new_range, 10..20);
    }

    #[test]
    fn test_intersects() {
        let range1 = 5..10;
        let range2 = 8..12;
        let range3 = 1..4;
        let range4 = 10..15;

        assert!(range1.intersects(&range2));
        assert!(!range1.intersects(&range3));
        assert!(!range1.intersects(&range4));
    }

    #[test]
    fn test_delta() {
        let range = 5..10;
        assert_eq!(range.delta(), 5);
    }
}
