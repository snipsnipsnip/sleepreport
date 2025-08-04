use super::api::{CountSleeps, FetchIcal, LoadSleeps, ReportSleeps};
use crate::io::show_range;
use crate::util::Apply as _;
use anyhow::Context as _;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, Utc};
use std::{fmt::Display, ops::Range};

// use std::env::Args;
#[derive(Default)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<NaiveDate>,
    pub hours_offset: Option<i32>,
}

pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<DateTime<Utc>>,
    pub hours_offset: i32,
}

pub trait AppConfiguration {
    fn configure(self, conf: impl FnOnce(Opts) -> Option<AppConfig>) -> Option<impl AppDependency>;
}

pub trait AppDependency {
    fn fetch_ical(&self) -> impl FetchIcal<Error = impl Into<anyhow::Error>>;
    fn load_sleeps(&self) -> impl LoadSleeps<Error = impl Into<anyhow::Error>>;
    fn count_sleeps(&self) -> impl CountSleeps;
    fn report_sleeps(&self) -> impl ReportSleeps<SleepReport = impl Display>;
}

pub struct App<C> {
    config: C,
}

impl<C: AppConfiguration> App<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }

    pub fn run(self) -> anyhow::Result<()> {
        let c = self
            .config
            .configure(|opts| {
                let d = opts
                    .date
                    .and_then(|d| {
                        d.and_time(NaiveTime::MIN)
                            .and_local_timezone(Local)
                            .single()
                    })
                    .unwrap_or_else(Local::now);
                AppConfig {
                    calendar_path: opts.calendar_path.unwrap_or("basic.ics".into()),
                    show_range: show_range::calc_show_range(d)?,
                    hours_offset: opts.hours_offset.unwrap_or(6),
                }
                .into()
            })
            .context("initialization")?;

        let sleeps = {
            let fetcher = c.fetch_ical();
            let buf = fetcher.fetch_ical().map_err(|e| e.into())?;

            c.load_sleeps()
                .load_sleeps(buf)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.into())?
        };
        let table = c.count_sleeps().count_sleeps(sleeps.into_iter());
        let report = c.report_sleeps().report_sleeps(table);

        println!("{}", report);

        Ok(())
    }
}
