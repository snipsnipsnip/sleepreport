use crate::sleep::{SleepReporter, SleepTable};
use anyhow::Context;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use std::ops::Range;

pub fn make_table(sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> anyhow::Result<String> {
    let mut table = create_sleep_table(Local::now()).context("could not create a sleep table")?;

    for sleep in sleeps {
        table.eat(sleep);
    }

    let reporter = SleepReporter::new(table);
    let lines = reporter.build_csv_lines(6, &Local).collect::<Vec<_>>();

    Ok(lines.join("\n"))
}

fn create_sleep_table(today: DateTime<impl TimeZone>) -> Option<SleepTable> {
    let last_monday = last_monday_before(&today);
    let start = last_monday.with_time(NaiveTime::MIN).single()?;
    let end = start.clone() + Days::new(7) + Duration::hours(23);

    SleepTable::new(start.to_utc()..end.to_utc()).into()
}

fn last_monday_before<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day.clone() - Days::new(diff_days as u64)
}
