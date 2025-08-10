use crate::{
    app::{self, AppDependency},
    config, io,
    sleep::{self, Hour, SleepReporterConfig},
};
use anyhow::Context as _;
use jiff::{civil::Date, tz::TimeZone};
use std::{fmt::Display, ops::Range};
use tap::Pipe as _;

/// Holds the fully resolved dependencies and configuration needed for the application to run.
/// This struct is the result of the configuration process and acts as the concrete
/// implementation for the `AppDependency` trait.
struct Dependencies {
    config: config::AppConfig,
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
    fn count_sleeps(&self) -> impl app::CountSleeps<Error = impl Into<anyhow::Error>> {
        sleep::SleepCounter::new(self.config.show_range.clone())
    }

    /// Provides the concrete implementation for generating the final, user-facing sleep report.
    fn report_sleeps(&self) -> impl app::ReportSleeps<SleepReport = impl Display, Error = impl Into<anyhow::Error>> {
        sleep::TsvSleepReporter::new(self)
    }

    /// Provides the concrete implementation for fetching the iCalendar data.
    fn fetch_ical(&self) -> impl app::FetchIcal<Error = impl Into<anyhow::Error>> {
        io::IcalFetcher::new(&self.config.calendar_path)
    }
}

impl SleepReporterConfig for Dependencies {
    fn report_range(&self) -> Range<Date> {
        self.config.show_range.clone()
    }

    fn offset_hours(&self) -> Hour {
        Hour::new(self.config.offset_hours.try_into().unwrap())
    }

    fn timezone(&self) -> TimeZone {
        TimeZone::system()
    }
}

/// The main entry point for the application's setup phase.
/// It parses command-line arguments and returns a `Dependencies` struct which can be
/// used to run the app.
pub fn startup(argv: impl Iterator<Item = String>) -> anyhow::Result<impl AppDependency> {
    config::parse_opts(argv)
        .and_then(config::configure)
        .map(|config| Dependencies { config })
}
