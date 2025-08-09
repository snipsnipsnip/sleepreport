use crate::{app::LoadSleeps, util::RangeExt};
use anyhow::Context;
use either::{Left, Right};
use ical::{IcalParser, parser::ical::component::IcalCalendar};
use ical_property::{DateMaybeTime, Event};
use jiff::Timestamp;
use std::{io::BufRead, iter, ops::Range};
use tap::Pipe as _;

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

    fn load_sleeps(&self, reader: impl BufRead) -> impl Iterator<Item = anyhow::Result<Range<Timestamp>>> {
        IcalParser::new(reader).into_iter().flat_map(|vcal| match vcal {
            Ok(ical) => Left(extract_sleeps(ical)),
            Err(e) => Right(iter::once(Err(e).context("parsing ical file"))),
        })
    }
}

/// Processes a parsed `IcalCalendar` to extract all valid sleep events from it.
fn extract_sleeps(ical: IcalCalendar) -> impl Iterator<Item = anyhow::Result<Range<Timestamp>>> {
    ical.events.into_iter().map(|ical_event| {
        let ev = ical_event.try_into().context("parsing an ical event")?;
        make_sleep_duration(ev).context("extracting duration from ical events")
    })
}

/// Converts a single iCalendar `Event` into a `Range<Timestamp>` representing the sleep period.
/// It returns `None` if the event is missing a start or end time.
fn make_sleep_duration(event: Event) -> anyhow::Result<Range<Timestamp>> {
    (event.start..event.end).try_fmap(|dt| match dt {
        Some(DateMaybeTime::DateTime(d)) => d.timestamp().pipe(Timestamp::from_second).map_err(|e| e.into()),
        _ => anyhow::bail!("Found an event without timestamp"),
    })
}
