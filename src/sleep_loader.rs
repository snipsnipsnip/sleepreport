use anyhow::Context;
use chrono::{DateTime, Utc};
use ical::{IcalParser, parser::ical::component::IcalCalendar};
use ical_property::{DateMaybeTime, Event};
use std::{boxed::Box, io::BufRead, ops::Range};

pub struct SleepLoader;

impl SleepLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load(
        &self,
        reader: impl BufRead,
    ) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
        let parser = IcalParser::new(reader);

        parser.into_iter().flat_map(|vcal| extract_sleeps(vcal))
    }
}

fn extract_sleeps(
    vcal: Result<IcalCalendar, ical::parser::ParserError>,
) -> Box<dyn Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>>> {
    match vcal {
        Ok(ical) => {
            let durations = ical.events.into_iter().map(|ical_event| {
                let ev = ical_event.try_into()?;
                make_sleep_duration(ev).context("extracting duration from ical events")
            });
            Box::new(durations)
        }
        Err(e) => Box::new([Err(e.into())].into_iter()),
    }
}

fn make_sleep_duration(event: Event) -> Option<Range<DateTime<Utc>>> {
    let [s, e] = [&event.start, &event.end].map(|d| get_datetime(d.as_ref()));
    (*s?..*e?).into()
}

fn get_datetime(dt: Option<&DateMaybeTime>) -> Option<&DateTime<Utc>> {
    match dt {
        Some(DateMaybeTime::DateTime(d)) => d.into(),
        _ => None,
    }
}
