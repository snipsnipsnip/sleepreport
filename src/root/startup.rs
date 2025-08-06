use crate::{
    app::{self, AppDependency},
    io, sleep,
    util::Apply as _,
};
use anyhow::Context as _;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use std::{fmt::Display, ops::Range};

/// Holds the fully resolved dependencies and configuration needed for the application to run.
/// This struct is the result of the configuration process and acts as the concrete
/// implementation for the `AppDependency` trait.
struct Dependencies {
    config: app::AppConfig,
}

/// The dependency injection container for the application.
/// This implementation provides the concrete types for the traits required by the `App`.
/// It wires up the `io` and `sleep` modules to provide the necessary functionality.
impl AppDependency for Dependencies {
    /// Provides the concrete implementation for loading sleep data from a source.
    fn load_sleeps(&self) -> impl app::LoadSleeps<Error = impl Into<anyhow::Error>> {
        io::SleepLoader::new()
    }

    /// Provides the concrete implementation for processing raw sleep data into a queryable format.
    fn count_sleeps(&self) -> impl app::CountSleeps {
        sleep::SleepCounter::new(self.config.show_range.clone())
    }

    /// Provides the concrete implementation for generating the final, user-facing sleep report.
    fn report_sleeps(&self) -> impl app::ReportSleeps<SleepReport = impl Display> {
        sleep::SleepReporter::new(self.config.hours_offset, &Local)
    }

    /// Provides the concrete implementation for fetching the iCalendar data.
    fn fetch_ical(&self) -> impl app::FetchIcal<Error = impl Into<anyhow::Error>> {
        io::IcalFetcher::new(&self.config.calendar_path)
    }
}

/// The main entry point for the application's setup phase.
/// It parses command-line arguments and returns a `Dependencies` struct which can be
/// used to run the app.
pub fn startup(argv: impl Iterator<Item = String>) -> anyhow::Result<impl AppDependency> {
    io::parse_opts(argv)
        .and_then(app::configure)
        .map(|config| Dependencies { config })
}
