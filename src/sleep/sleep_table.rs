use crate::app::SleepQuery;
use crate::util::RangeLike;
use chrono::{DateTime, Duration, DurationRound, NaiveDate, TimeDelta, TimeZone, Utc};
use std::ops::Range;

/// A data structure that represents a time range discretized into hourly slots.
/// It is used to efficiently check for sleep at any given hour within that range.
#[derive(Debug)]
pub struct SleepTable {
    /// A boolean vector where each element corresponds to a one-hour slot.
    /// `true` indicates that sleep was recorded during that hour.
    slept: Vec<bool>,
    /// The overall time range that this table covers.
    range: Range<DateTime<Utc>>,
}

impl SleepTable {
    /// Creates a new `SleepTable` for a given `DateTime` range.
    /// The `slept` vector is initialized to `false` for all hours in the range.
    pub fn new(range: Range<DateTime<Utc>>) -> Self {
        Self {
            slept: vec![false; range.delta().num_hours() as usize],
            range: range,
        }
    }

    /// Marks the hours covered by a given sleep `Range` as `true` in the `slept` vector.
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
        self.range.intersects(&sleep).then_some(())?;

        let sleep_start_index = (sleep.start - self.range.start).num_hours() as usize;
        let sleep_length_mins = sleep.delta().num_minutes();
        let sleep_length_hours = ((sleep_length_mins + 30) as f32 / 60f32).round() as usize;
        let slept_indices = sleep_start_index..sleep_start_index + sleep_length_hours;

        slept_indices.into()
    }
}

impl SleepQuery for SleepTable {
    /// Checks if sleep was recorded at a specific `DateTime`.
    /// The time is rounded down to the nearest hour.
    fn is_sleeping_at(&self, hour: &DateTime<Utc>) -> Option<bool> {
        let dt = hour.duration_round(TimeDelta::hours(1)).ok()?;

        let index = (dt - self.range.start).num_hours();
        (0 <= index).then_some(())?;

        self.slept.get(index as usize).copied()
    }
    /// Returns the `NaiveDate` range of the table, converted to a specific timezone.
    /// This is used by the reporter to create the date header.
    fn day_range_in(&self, tz: &impl TimeZone) -> Range<NaiveDate> {
        self.range.fmap(|d| d.with_timezone(tz).date_naive())
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
