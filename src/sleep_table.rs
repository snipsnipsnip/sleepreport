use std::ops::Range;

use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use ical_property::DateMaybeTime;

#[derive(Debug)]
pub struct SleepTable {
    range: Range<DateTime<Utc>>,
    sleeps: Vec<Range<DateTime<Utc>>>,
}

impl SleepTable {
    pub fn new<Tz: TimeZone>(today: DateTime<Tz>) -> Option<SleepTable> {
        let last_monday = last_monday_before(&today);
        let start = last_monday.with_time(NaiveTime::MIN).single()?;
        let end = start.clone() + Days::new(7) + Duration::hours(7);

        Some(Self::during(start.to_utc()..end.to_utc()))
    }

    pub fn during(range: Range<DateTime<Utc>>) -> SleepTable {
        SleepTable {
            range: range,
            sleeps: vec![],
        }
    }

    pub fn eat(&mut self, event: &ical_property::Event) -> Option<()> {
        let [s, e] = [&event.start, &event.end].map(|d| self.to_local(d.as_ref()));

        self.add_sleep(s?, e?);

        Some(())
    }

    pub fn lines<Tz: TimeZone>(&self, tz: &Tz) -> String {
        let slept = {
            let mut slept = vec![false; (self.range.end - self.range.start).num_hours() as usize];

            for sleep in &self.sleeps {
                let se = fmap(sleep.clone(), |d| (d - self.range.start).num_hours());

                for hr in se.start..=se.end {
                    slept[hr as usize] = true;
                    // (self.range.start + Duration::hours(hr)).with_timezone(tz)
                }
            }

            slept
        };

        let local = fmap(self.range.clone(), |d| d.with_timezone(tz));
        println!("{:?}", local);
        let days = (local.end - local.start).num_days();

        (0..24 as usize)
            .map(|hr| {
                format!(
                    "{}\t{}",
                    hr,
                    (0..days as usize)
                        .map(|d| if slept[d * 24 + hr] { "O" } else { "_" })
                        .collect::<Vec<_>>()
                        .join("\t")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn to_local(&self, dt: Option<&DateMaybeTime>) -> Option<DateTime<Utc>> {
        match dt? {
            DateMaybeTime::DateTime(d) => Some(d.clone()),
            _ => None,
        }
    }

    fn add_sleep(&mut self, s: DateTime<Utc>, e: DateTime<Utc>) {
        if self.range.contains(&s) || self.range.contains(&e) {
            self.sleeps.push(s..e);
        }
    }
}

fn last_monday_before<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day.clone() - Days::new(diff_days as u64)
}

fn fmap<T, U, F: Fn(T) -> U>(range: Range<T>, f: F) -> Range<U> {
    f(range.start)..f(range.end)
}
