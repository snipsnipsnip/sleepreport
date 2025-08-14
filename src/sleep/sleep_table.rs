use crate::app::SleepQuery;
use crate::app::SleepState::{self, Asleep, Awake, Unknown};
use crate::util::RangeExt as _;
use jiff::{Timestamp, Unit};
use std::ops::Range;
use tap::Pipe as _;

/// A data structure that represents a time range discretized into hourly slots.
/// It is used to efficiently check for sleep at any given hour within that range.
#[derive(Debug)]
pub struct SleepTable {
    /// The overall time range that this table covers.
    range: Range<Timestamp>,
    /// A bool array where each element corresponds to a one-hour slot.
    /// `true` indicates that sleep was recorded during that hour.
    slept: Box<[bool]>,
}

impl SleepTable {
    /// Creates a new `SleepTable` for a given `DateTime` range.
    pub fn new(range: Range<Timestamp>) -> Result<Self, jiff::Error> {
        let slept = vec![false; range.delta().total(Unit::Hour)? as usize].into();
        Self { slept, range }.pipe(Ok)
    }

    /// Marks the hours covered by a given sleep `Range` as sleeping.
    /// This method effectively "paints" the sleep periods onto the table.
    #[must_use]
    pub fn eat(&mut self, sleep: Range<Timestamp>) -> Option<()> {
        let slept_indices = self.map_indices(sleep)?;
        self.slept.get_mut(slept_indices)?.fill(true);

        Some(())
    }

    /// Converts a `DateTime` range into a range of indices into the `slept` vector.
    /// It returns `None` if the sleep range does not overlap with the table's range.
    /// The duration is rounded to the nearest hour.
    fn map_indices(&self, sleep: Range<Timestamp>) -> Option<Range<usize>> {
        if !self.range.intersects(&sleep) {
            return None;
        }

        let Range { start, end } = sleep.fmap(move |t| self.index_at(t));
        if start == end {
            return None;
        }

        Range {
            start: start.unwrap_or(0),
            end: end.unwrap_or(self.slept.len()),
        }
        .into()
    }

    /// Converts a `DateTime` into an index of the `slept` vector.
    /// Returns `None` if out of range.
    fn index_at(&self, hour: Timestamp) -> Option<usize> {
        Some(hour - self.range.start)
            .and_then(|span| span.total(Unit::Hour).ok())
            .filter(|&hours| 0.0 <= hours)
            .map(|hours| hours.round() as usize)
            .filter(move |&i| i < self.slept.len())
    }
}

impl SleepQuery for SleepTable {
    /// Checks if sleep was recorded at a specific `DateTime`.
    /// The time is rounded down to the nearest hour.
    fn is_sleeping_at(&self, hour: Timestamp) -> SleepState {
        match self.index_at(hour).and_then(move |i| self.slept.get(i)) {
            Some(true) => Asleep,
            Some(false) => Awake,
            None => Unknown,
        }
    }

    fn query_range(&self) -> &Range<Timestamp> {
        &self.range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{IterExt as _, assert_eq};
    use jiff::{
        ToSpan as _,
        civil::{Date, DateTime, Time, date},
        tz::TimeZone,
    };
    use std::iter::repeat_n;

    #[test]
    fn test_is_sleeping_at() -> anyhow::Result<()> {
        let start_time = utc_timestamp(date(2024, 1, 1))?;
        let end_time = utc_timestamp(date(2024, 1, 2))?;
        let range = start_time..end_time;
        let mut table = SleepTable::new(range)?;

        assert_eq!(
            (-1..=24)
                .map(|hr: i64| table.is_sleeping_at(start_time + (60 * hr).minutes()))
                .into_vec(),
            repeat_n(Unknown, 1)
                .chain(repeat_n(Awake, 24))
                .chain(repeat_n(Unknown, 1))
                .into_vec()
        );

        let sleep_time = (start_time + (60 * 1).minutes())..(start_time + (60 * 2 + 30).minutes());
        assert_eq!(table.eat(sleep_time), Some(()));

        assert_eq!(
            (-1..=24)
                .map(|hr: i64| table.is_sleeping_at(start_time + (60 * hr).minutes()))
                .into_vec(),
            repeat_n(Unknown, 1)
                .chain(repeat_n(Awake, 1))
                .chain(repeat_n(Asleep, 2))
                .chain(repeat_n(Awake, 21))
                .chain(repeat_n(Unknown, 1))
                .into_vec()
        );

        Ok(())
    }

    fn utc_timestamp(date: Date) -> Result<Timestamp, jiff::Error> {
        TimeZone::UTC.to_timestamp(date.to_datetime(Time::default()))
    }
}
