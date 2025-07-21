use std::ops::Range;

use chrono::{DateTime, Datelike, Days, Duration, NaiveTime, TimeZone, Utc};
use ical_property::DateMaybeTime;

#[derive(Debug)]
pub struct SleepTable {
    range: Range<DateTime<Utc>>,
    sleeps: Vec<Range<DateTime<Utc>>>,
}

const OFFSET_HOURS: i32 = 6;

impl SleepTable {
    pub fn new(today: DateTime<impl TimeZone>) -> Option<SleepTable> {
        let last_monday = last_monday_before(&today);
        let start = last_monday.with_time(NaiveTime::MIN).single()?;
        let end = start.clone() + Days::new(7) + Duration::hours(1 + OFFSET_HOURS as i64);

        Some(Self::during(start.to_utc()..end.to_utc()))
    }

    pub fn during(range: Range<DateTime<Utc>>) -> SleepTable {
        SleepTable {
            range: range,
            sleeps: vec![],
        }
    }

    pub fn eat(&mut self, event: &ical_property::Event) -> Option<()> {
        let [s, e] = [&event.start, &event.end].map(|d| self.get_datetime(d.as_ref()));

        self.add_sleep(s?, e?);

        Some(())
    }

    pub fn lines(&self, tz: &impl TimeZone) -> String {
        let slept = {
            let mut slept = vec![false; (self.range.end - self.range.start).num_hours() as usize];

            for sleep in &self.sleeps {
                let sleep_start_index = (sleep.start - self.range.start).num_hours() as usize;
                let sleep_length_mins = (sleep.end - sleep.start).num_minutes();
                let sleep_length_hours = ((sleep_length_mins + 30) as f32 / 60f32).round() as usize;

                slept[sleep_start_index..sleep_start_index + sleep_length_hours].fill(true);
                // (self.range.start + Duration::hours(hr)).with_timezone(tz)
            }

            slept
        };

        let local = fmap(self.range.clone(), |d| d.with_timezone(tz));
        let days = (local.end.clone() - &local.start).num_days() as i32;

        format!(
            "{:?}\n{}",
            local,
            (OFFSET_HOURS..OFFSET_HOURS + 24)
                .map(|hr| {
                    format!(
                        "{:02}:00～{:02}:00\t{}",
                        hr % 24,
                        (hr + 1) % 24,
                        (0_i32..days)
                            .map(|d| if slept[(d * 24 + hr) as usize] {
                                "睡眠"
                            } else {
                                ""
                            })
                            .collect::<Vec<_>>()
                            .join("\t")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn get_datetime(&self, dt: Option<&DateMaybeTime>) -> Option<DateTime<Utc>> {
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
