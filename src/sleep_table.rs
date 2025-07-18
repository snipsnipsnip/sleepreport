use chrono::{DateTime, Local};
use ical_property::DateMaybeTime;

#[derive(Debug)]
pub struct SleepTable {}

impl SleepTable {
    pub fn new() -> SleepTable {
        SleepTable {}
    }

    pub fn eat(&mut self, event: &ical_property::Event) -> Option<()> {
        let s = to_local(event.start.as_ref())?;
        let e = to_local(event.end.as_ref())?;
        println!("eating: {:?} - {:?}", s, e);
        Some(())
    }
}

fn to_local(dt: Option<&DateMaybeTime>) -> Option<DateTime<Local>> {
    if let DateMaybeTime::DateTime(d) = dt? {
        Some(d.with_timezone(&Local))
    } else {
        None
    }
}
