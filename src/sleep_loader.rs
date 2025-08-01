use anyhow::Context;
use chrono::{DateTime, Utc};
use either::{Left, Right};
use ical::{IcalParser, parser::ical::component::IcalCalendar};
use ical_property::{DateMaybeTime, Event};
use std::{io::BufRead, iter, ops::Range};

use crate::util::Apply;

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

        parser
            .into_iter()
            .flat_map(|vcal| extract_sleeps(vcal.map_err(|e| e.into())))
    }
}

fn extract_sleeps(
    vcal: anyhow::Result<IcalCalendar>,
) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
    match vcal {
        Ok(ical) => ical
            .events
            .into_iter()
            .map(|ical_event| {
                let ev = ical_event.try_into()?;
                make_sleep_duration(ev).context("extracting duration from ical events")
            })
            .apply(Left),
        Err(e) => Right(iter::once(Err(e))),
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
