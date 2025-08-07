use crate::app::LoadSleeps;
use anyhow::Context;
use chrono::{DateTime, Utc};
use either::{Left, Right};
use ical::{
    IcalParser,
    parser::{ParserError, ical::component::IcalCalendar},
};
use ical_property::{DateMaybeTime, Event};
use std::{error::Error, io::BufRead, iter, ops::Range};

/// The concrete implementation of the `LoadSleeps` trait for parsing iCalendar data.
/// This struct is stateless and provides the logic for converting a raw byte stream
/// into an iterator of sleep events.
pub struct SleepLoader;

impl SleepLoader {
    pub fn new() -> Self {
        Self
    }
}

impl LoadSleeps for SleepLoader {
    type Error = anyhow::Error;

    fn load_sleeps(&self, reader: impl BufRead) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
        IcalParser::new(reader).into_iter().flat_map(|vcal| match vcal {
            Ok(ical) => Left(extract_sleeps(ical)),
            Err(e) => Right(iter::once(Err(e).context("parsing ical file"))),
        })
    }
}

/// Processes a parsed `IcalCalendar` to extract all valid sleep events from it.
fn extract_sleeps(ical: IcalCalendar) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
    ical.events.into_iter().map(|ical_event| {
        let ev = ical_event.try_into().context("parsing an ical event")?;
        make_sleep_duration(ev).context("extracting duration from ical events")
    })
}

/// Converts a single iCalendar `Event` into a `Range<DateTime<Utc>>` representing the sleep period.
/// It returns `None` if the event is missing a start or end time.
fn make_sleep_duration(event: Event) -> Option<Range<DateTime<Utc>>> {
    let [s, e] = [&event.start, &event.end].map(|d| get_datetime(d.as_ref()));
    (*s?..*e?).into()
}

/// A helper function to extract a `DateTime<Utc>` from the `DateMaybeTime` enum used by the `ical` crate.
/// It skips whole-date events and events without date info.
fn get_datetime(dt: Option<&DateMaybeTime>) -> Option<&DateTime<Utc>> {
    match dt {
        Some(DateMaybeTime::DateTime(d)) => d.into(),
        _ => None,
    }
}
