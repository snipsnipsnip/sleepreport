use super::api::{CountSleeps, FetchIcal, LoadSleeps, ReportSleeps};
use crate::io::show_range;
use crate::util::Apply as _;
use anyhow::Context as _;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, Utc};
use std::{
    fmt::Display,
    ops::{Deref, Range},
};

/// Represents the raw, unprocessed options parsed directly from command-line arguments.
/// This struct allows for optional values, which will be resolved into a concrete
/// `AppConfig` during the configuration phase.
#[derive(Default)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<NaiveDate>,
    pub hours_offset: Option<i32>,
}

/// Holds the fully resolved and validated configuration required for the application to run.
/// This is created from `Opts` and provides guaranteed values for all settings.
pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<DateTime<Utc>>,
    pub hours_offset: i32,
}

/// Defines the first stage of the application setup. It provides a mechanism
/// to transform the raw `Opts` into a fully configured `AppDependency` container.
/// This trait decouples the `App` from the specifics of how configuration is created.
pub trait AppConfiguration {
    /// Takes a closure that contains the logic for converting `Opts` to `AppConfig`,
    /// returning a ready-to-use dependency container.
    fn configure(self, conf: impl FnOnce(Opts) -> Option<AppConfig>) -> Option<impl AppDependency>;
}

/// Defines the contract for the application's dependencies using a trait-based
/// dependency injection pattern. The `App` is generic over this trait, allowing
/// for different implementations (e.g., for production or testing).
pub trait AppDependency {
    /// Provides the functionality to fetch the raw iCalendar data.
    fn fetch_ical(&self) -> impl FetchIcal<Error = impl Into<anyhow::Error>>;
    /// Provides the functionality to parse the raw iCalendar data into sleep intervals.
    fn load_sleeps(&self) -> impl LoadSleeps<Error = impl Into<anyhow::Error>>;
    /// Provides the functionality to process sleep intervals into a queryable data structure.
    fn count_sleeps(&self) -> impl CountSleeps;
    /// Provides the functionality to generate the final, user-facing report from the sleep data.
    fn report_sleeps(&self) -> impl ReportSleeps<SleepReport = impl Display>;
}

/// The core application runner. It orchestrates the main logic by using the
/// dependencies provided by the `AppDependency` trait.
pub struct App<C: AppDependency> {
    deps: C,
}

impl<C: AppDependency> App<C> {
    /// Creates a new `App` instance with its required dependencies.
    pub fn new(deps: C) -> Self {
        Self { deps }
    }

    /// Executes the main application workflow:
    /// 1. Fetches the iCalendar data.
    /// 2. Loads and parses the sleep events.
    /// 3. Counts and organizes the sleep data into a table.
    /// 4. Generates and prints the final report.
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

/// The concrete configuration logic that transforms `Opts` into `AppConfig`.
/// This function contains the business logic for setting default values and calculating
/// derived configuration, such as the `show_range`.
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