use anyhow::Context;
use chrono::{DateTime, Utc};
use either::{Left, Right};
use ical::{IcalParser, parser::ical::component::IcalCalendar};
use ical_property::{DateMaybeTime, Event};
use std::{error::Error, io::BufRead, iter, ops::Range};

use crate::{sleep::LoadSleeps, util::Apply};

pub struct SleepLoader<B> {
    parser: IcalParser<B>,
}

impl<B> SleepLoader<B>
where
    B: BufRead,
{
    pub fn new(reader: B) -> Self {
        Self {
            parser: IcalParser::new(reader),
        }
    }
}

impl<B> LoadSleeps for SleepLoader<B>
where
    B: BufRead,
{
    fn load_sleeps(self) -> impl Iterator<Item = anyhow::Result<Range<DateTime<Utc>>>> {
        self.parser
            .into_iter()
            .flat_map(|vcal| extract_sleeps(vcal))
    }
}

fn extract_sleeps(
    vcal: Result<IcalCalendar, impl Into<anyhow::Error>>,
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
        Err(e) => Right(iter::once(Err(e.into()))),
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
