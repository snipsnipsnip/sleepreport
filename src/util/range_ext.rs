use std::{
    borrow::Borrow,
    ops::{Range, Sub},
};

/// Provides extension methods for `Range<T>`.
#[allow(dead_code)]
pub trait RangeExt<T>: Borrow<Range<T>> {
    /// Maps a `Range<T>` to a `Range<U>` by applying a function to its start and end.
    fn fmap<U>(self, f: impl FnMut(T) -> U) -> Range<U>
    where
        Self: Into<Range<T>>,
    {
        let Range { start, end } = self.into();
        let mut f = f;
        f(start)..f(end)
    }

    /// Tries to map a `Range<T>` to a `Range<U>` by applying a function to its start and end,
    /// returning an error if either mapping fails.
    fn try_fmap<U, E>(self, f: impl FnMut(T) -> Result<U, E>) -> Result<Range<U>, E>
    where
        Self: Into<Range<T>>,
    {
        let Range { start, end } = self.into();
        let mut f = f;
        match f(start) {
            Err(err) => Err(err),
            Ok(s) => match f(end) {
                Err(err) => Err(err),
                Ok(e) => Ok(s..e),
            },
        }
    }

    /// Replaces start of the range.
    fn with_start(self, start: T) -> Range<T>
    where
        Self: Into<Range<T>>,
    {
        start..self.into().end
    }

    fn map_start(self, f: impl FnOnce(T) -> T) -> Range<T>
    where
        Self: Into<Range<T>>,
    {
        let Range { start, end } = self.into();
        f(start)..end
    }

    /// Replaces end of the range.
    fn with_end(self, end: T) -> Range<T>
    where
        Self: Into<Range<T>>,
    {
        self.into().start..end
    }

    fn map_end(self, f: impl FnOnce(T) -> T) -> Range<T>
    where
        Self: Into<Range<T>>,
    {
        let Range { start, end } = self.into();
        start..f(end)
    }

    /// Checks if two ranges intersect.
    fn intersects(&self, other: &impl Borrow<Range<T>>) -> bool
    where
        T: PartialOrd,
    {
        let Range { start: sa, end: ea } = self.borrow();
        let Range { start: sb, end: eb } = other.borrow();

        sa <= sb && sb < ea || sb <= sa && sa < eb
    }

    /// Calculates the difference between the end and start of the range.
    fn delta(&self) -> <T as Sub>::Output
    where
        T: Copy + Sub,
    {
        let Range { start, end } = *self.borrow();
        end - start
    }
}

impl<T, R: Borrow<Range<T>>> RangeExt<T> for R {}

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
    fn test_try_fmap() {
        let range = 5..10;
        let result = range
            .clone()
            .try_fmap(|x| if x == 0 { Err("error") } else { Ok(x * 2) });
        assert_eq!(result, Ok(10..20));

        let result = range
            .clone()
            .try_fmap(|x| if x == 5 { Err("error") } else { Ok(x * 2) });
        assert_eq!(result, Err("error"));

        let result = range
            .clone()
            .try_fmap(|x| if x == 10 { Err("error") } else { Ok(x * 2) });
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_intersects() {
        let range1 = 5..10;
        let range2 = 8..12;
        let range3 = 1..4;
        let range4 = 10..15;
        let range5 = 5..9;
        let range6 = 6..9;

        assert!(range1.intersects(&range2));
        assert!(range2.intersects(&range1));
        assert!(!range1.intersects(&range3));
        assert!(!range3.intersects(&range1));
        assert!(!range1.intersects(&range4));
        assert!(!range4.intersects(&range1));
        assert!(range1.intersects(&range5));
        assert!(range5.intersects(&range1));
        assert!(range1.intersects(&range6));
        assert!(range6.intersects(&range1));
    }

    #[test]
    fn test_delta() {
        let range = 5..10;
        assert_eq!(range.delta(), 5);
    }
}
