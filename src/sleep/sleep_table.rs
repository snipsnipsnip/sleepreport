use crate::app::SleepQuery;
use crate::util::RangeLike;
use chrono::{DateTime, Duration, DurationRound, NaiveDate, TimeDelta, TimeZone, Utc};
use std::{
    cmp::{max, min},
    ops::Range,
};

/// A data structure that represents a time range discretized into hourly slots.
/// It is used to efficiently check for sleep at any given hour within that range.
#[derive(Debug)]
pub struct SleepTable {
    /// The overall time range that this table covers.
    range: Range<DateTime<Utc>>,
    /// A bool array where each element corresponds to a one-hour slot.
    /// `true` indicates that sleep was recorded during that hour.
    slept: Box<[bool]>,
}

impl SleepTable {
    /// Creates a new `SleepTable` for a given `DateTime` range.
    pub fn new(range: Range<DateTime<Utc>>) -> Self {
        let slept = vec![false; range.delta().num_hours() as usize].into();
        Self { slept, range }
    }

    /// Marks the hours covered by a given sleep `Range` as sleeping.
    /// This method effectively "paints" the sleep periods onto the table.
    #[must_use]
    pub fn eat(&mut self, sleep: Range<DateTime<Utc>>) -> Option<()> {
        let slept_indices = self.map_indices(sleep)?;
        self.slept.get_mut(slept_indices)?.fill(true);

        Some(())
    }

    /// Converts a `DateTime` range into a range of indices into the `slept` vector.
    /// It returns `None` if the sleep range does not overlap with the table's range.
    /// The duration is rounded to the nearest hour.
    fn map_indices(&self, sleep: Range<DateTime<Utc>>) -> Option<Range<usize>> {
        if !self.intersects(&sleep) {
            return None;
        }

        let Range { start, end } = sleep.fmap(move |&t| self.index_at(t));
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
    fn index_at(&self, hour: DateTime<Utc>) -> Option<usize> {
        Some(hour - self.start())
            .filter(|&delta| TimeDelta::zero() <= delta)
            .map(|delta| (delta.num_minutes() as f32 / 60f32).round() as usize)
            .filter(move |&i| i < self.slept.len())
    }
}

impl SleepQuery for SleepTable {
    /// Checks if sleep was recorded at a specific `DateTime`.
    /// The time is rounded down to the nearest hour.
    fn is_sleeping_at(&self, hour: DateTime<Utc>) -> Option<bool> {
        let index = self.index_at(hour)?;

        self.slept.get(index).copied()
    }
}

impl RangeLike for SleepTable {
    type Item = DateTime<Utc>;

    fn start(&self) -> &Self::Item {
        self.range.start()
    }

    fn end(&self) -> &Self::Item {
        self.range.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{IterExt as _, assert_eq};
    use std::iter::repeat_n;

    #[test]
    fn test_is_sleeping_at() {
        let start_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end_time = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();
        let range = start_time..end_time;
        let mut table = SleepTable::new(range);

        assert_eq!(
            (-1..=24)
                .map(|hr: i64| table.is_sleeping_at(start_time + Duration::minutes(60 * hr)))
                .into_vec(),
            repeat_n(None, 1)
                .chain(repeat_n(Some(false), 24))
                .chain(repeat_n(None, 1))
                .into_vec()
        );

        let sleep_time = (start_time + Duration::minutes(60 * 1))..(start_time + Duration::minutes(60 * 2 + 30));
        assert_eq!(table.eat(sleep_time), Some(()));

        assert_eq!(
            (-1..=24)
                .map(|hr: i64| table.is_sleeping_at(start_time + Duration::minutes(60 * hr)))
                .into_vec(),
            repeat_n(None, 1)
                .chain(repeat_n(Some(false), 1))
                .chain(repeat_n(Some(true), 2))
                .chain(repeat_n(Some(false), 21))
                .chain(repeat_n(None, 1))
                .into_vec()
        );
    }
}
