use super::api::{CountSleeps, FetchIcal, LoadSleeps, ReportSleeps};
use crate::io::show_range;
use crate::util::Apply as _;
use anyhow::Context as _;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, Utc};
use std::{
    fmt::Display,
    ops::{Deref, Range},
};

/// Command-line options for the application.
#[derive(Default)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<NaiveDate>,
    pub hours_offset: Option<i32>,
}

/// Application configuration.
pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<DateTime<Utc>>,
    pub hours_offset: i32,
}

/// A trait for configuring the application.
pub trait AppConfiguration {
    /// Configures the application with the given options.
    fn configure(self, conf: impl FnOnce(Opts) -> Option<AppConfig>) -> Option<impl AppDependency>;
}

/// A trait for providing application dependencies.
pub trait AppDependency {
    /// Returns a fetcher for iCalendar files.
    fn fetch_ical(&self) -> impl FetchIcal<Error = impl Into<anyhow::Error>>;
    /// Returns a loader for sleep data.
    fn load_sleeps(&self) -> impl LoadSleeps<Error = impl Into<anyhow::Error>>;
    /// Returns a counter for sleeps.
    fn count_sleeps(&self) -> impl CountSleeps;
    /// Returns a reporter for sleep data.
    fn report_sleeps(&self) -> impl ReportSleeps<SleepReport = impl Display>;
}

/// The main application struct.
pub struct App<C: AppDependency> {
    deps: C,
}

impl<C: AppDependency> App<C> {
    /// Creates a new `App` with the given dependencies.
    pub fn new(deps: C) -> Self {
        Self { deps }
    }

    /// Runs the application.
    pub fn run(self) -> anyhow::Result<()> {
        let c = self.deps;

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

/// Configures the application.
pub fn configure(conf: impl AppConfiguration) -> Option<impl AppDependency> {
    conf.configure(|opts| {
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
}
