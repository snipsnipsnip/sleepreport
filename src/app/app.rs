use super::api::{CountSleeps, FetchIcal, LoadSleeps, ReportSleeps};
use crate::util::IterExt as _;
use anyhow::Context as _;
use std::fmt::Display;

pub trait Provider<T> {
    fn provide(&self) -> T;
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
    fn count_sleeps(&self) -> impl CountSleeps<Error = impl Into<anyhow::Error>>;
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

            c.load_sleeps().load_sleeps(buf).into_try_vec().map_err(|e| e.into())?
        };
        let table = c
            .count_sleeps()
            .count_sleeps(sleeps.into_iter())
            .map_err(|e| e.into())?;
        let report = c.report_sleeps().report_sleeps(table);

        println!("{}", report);

        Ok(())
    }
}
