use crate::util::RangeExt;
use jiff::{SpanTotal, ToSpan as _, Unit, civil::Date};
use std::{borrow::Borrow, ops::Range};

/// Provides extension methods for `Range<Date>`.
pub trait DateRangeExt: Sized + Borrow<Range<Date>> {
    /// Iterates over the days in the range.
    fn iter_days(&self) -> impl Iterator<Item = Date> {
        let Range { start, end } = self.borrow();
        start.series(1.day()).take_while(move |d| d < end)
    }

    /// Calculates the number of days in the range.
    fn num_days(&self) -> i32 {
        self.delta()
            .total(SpanTotal::from(Unit::Day).days_are_24_hours())
            .map(|f| f.round() as i32)
            .expect("num_days() calculation failed")
    }
}

impl DateRangeExt for Range<Date> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{IterExt, assert_eq};
    use jiff::civil::date;

    #[test]
    fn test_iter_days() {
        let start = date(2024, 1, 1);
        let end = date(2024, 1, 4);
        let range = start..end;
        assert_eq!(
            range.iter_days().into_vec(),
            vec![date(2024, 1, 1), date(2024, 1, 2), date(2024, 1, 3),]
        );
    }

    #[test]
    fn test_num_days() {
        let start = date(2024, 1, 1);
        let end = date(2024, 1, 4);
        let range = start..end;
        assert_eq!(range.num_days(), 3);
    }
}
