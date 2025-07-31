use crate::sleep_reporter::SleepReporter;
use anyhow::Context;
use chrono::{DateTime, Local, Utc};
use std::ops::Range;

pub fn make_table(sleeps: impl Iterator<Item = Range<DateTime<Utc>>>) -> anyhow::Result<String> {
    let mut table = SleepReporter::new(Local::now()).context("init a sleep table")?;

    for sleep in sleeps {
        table.eat(sleep);
    }

    let lines = table.build_csv_lines(6, &Local).collect::<Vec<_>>();

    Ok(lines.join("\n"))
}
