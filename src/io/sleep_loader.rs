use crate::{app::LoadSleeps, util::Apply};
use anyhow::Context;
use chrono::{DateTime, Utc};
use either::{Left, Right};
use ical::{IcalParser, parser::ical::component::IcalCalendar};
use ical_property::{DateMaybeTime, Event};
use std::{error::Error, io::BufRead, iter, ops::Range};

/// The concrete implementation of the `LoadSleeps` trait for parsing iCalendar data.
/// This struct is stateless and provides the logic for converting a raw byte stream
/// into an iterator of sleep events.
pub struct SleepLoader;

impl SleepLoader {
    /// Creates a new `SleepLoader`.
    pub fn new() -> Self {
        Self
    }
}

impl LoadSleeps for SleepLoader {
    type Error = anyhow::Error;

    /// This is the main entry point for this struct. It takes a `BufRead` source,
    /// creates an `IcalParser` from it, and then flat-maps the parsed calendars
    /// into a stream of sleep `Range`s using the `extract_sleeps` function.
    fn load_sleeps(&self, reader: impl BufRead) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
        IcalParser::new(reader).into_iter().flat_map(extract_sleeps)
    }
}

/// Processes a single parsed `IcalCalendar` and extracts all valid sleep events from it.
/// It handles both successful parsing of the calendar and errors that may occur during parsing.
fn extract_sleeps(
    vcal: Result<IcalCalendar, impl Error + Send + Sync + 'static>,
) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
    match vcal {
        Ok(ical) => ical
            .events
            .into_iter()
            .map(|ical_event| {
                let ev = ical_event.try_into().context("parsing an ical event")?;
                make_sleep_duration(ev).context("extracting duration from ical events")
            })
            .apply(Left),
        Err(e) => Right(iter::once(Err(e).context("parsing ical file"))),
    }
}

/// Converts a single iCalendar `Event` into a `Range<DateTime<Utc>>` representing the sleep period.
/// It returns `None` if the event is missing a start or end time.
fn make_sleep_duration(event: Event) -> Option<Range<DateTime<Utc>>> {
    let [s, e] = [&event.start, &event.end].map(|d| get_datetime(d.as_ref()));
    (*s?..*e?).into()
}

/// A helper function to extract a `DateTime<Utc>` from the `DateMaybeTime` enum used by the `ical` crate.
/// It returns `None` if the value is not a `DateTime`.
fn get_datetime(dt: Option<&DateMaybeTime>) -> Option<&DateTime<Utc>> {
    match dt {
        Some(DateMaybeTime::DateTime(d)) => d.into(),
        _ => None,
    }
}
