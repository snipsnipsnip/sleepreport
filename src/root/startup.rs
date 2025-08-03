use crate::{
    app::{self, AppDependency},
    io, sleep,
};
use anyhow::Context as _;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use std::{fmt::Display, ops::Range};

struct Startup {
    date_range: Range<DateTime<Utc>>,
}

impl AppDependency for Startup {
    fn load_sleeps(&self) -> impl app::LoadSleeps<Error = impl Into<anyhow::Error>> {
        io::SleepLoader::new()
    }

    fn count_sleeps(&self) -> impl app::CountSleeps {
        sleep::SleepCounter::new(self.date_range.clone())
    }

    fn report_sleeps(&self) -> impl app::ReportSleeps<SleepReport = impl Display> {
        sleep::SleepReporter::new(6, &Local)
    }
}

pub fn startup(_argv: impl Iterator<Item = String>) -> anyhow::Result<impl AppDependency> {
    let date_range = calc_show_range(Local::now()).context("calculating today")?;
    Ok(Startup { date_range })
}

fn calc_show_range(today: DateTime<impl TimeZone>) -> Option<Range<DateTime<Utc>>> {
    let last_monday = last_monday_before(today);
    let start = last_monday.with_time(NaiveTime::MIN).single()?;
    let end = start.clone() + Days::new(7) + Duration::hours(23);

    (start.to_utc()..end.to_utc()).into()
}

fn last_monday_before<Tz: TimeZone>(day: DateTime<Tz>) -> DateTime<Tz> {
    let diff_days = match day.weekday().num_days_from_monday() {
        0 => 7,
        x => x,
    };
    day - Days::new(diff_days as u64)
}
