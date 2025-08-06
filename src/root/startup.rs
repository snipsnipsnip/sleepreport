use crate::{app, io, sleep, util::Apply as _};
use anyhow::Context as _;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use std::{fmt::Display, ops::Range};

/// The application's startup configuration.
struct Startup {
    opts: app::Opts,
}

/// The application's dependencies.
struct Dependencies {
    config: app::AppConfig,
}

impl app::AppConfiguration for Startup {
    /// Configures the application with the given options.
    fn configure(
        self,
        conf: impl FnOnce(app::Opts) -> Option<app::AppConfig>,
    ) -> Option<impl app::AppDependency> {
        conf(self.opts).map(|config| Dependencies { config })
    }
}

impl app::AppDependency for Dependencies {
    /// Returns a loader for sleep data.
    fn load_sleeps(&self) -> impl app::LoadSleeps<Error = impl Into<anyhow::Error>> {
        io::SleepLoader::new()
    }

    /// Returns a counter for sleeps.
    fn count_sleeps(&self) -> impl app::CountSleeps {
        sleep::SleepCounter::new(self.config.show_range.clone())
    }

    /// Returns a reporter for sleep data.
    fn report_sleeps(&self) -> impl app::ReportSleeps<SleepReport = impl Display> {
        sleep::SleepReporter::new(self.config.hours_offset, &Local)
    }

    /// Returns a fetcher for iCalendar files.
    fn fetch_ical(&self) -> impl app::FetchIcal<Error = impl Into<anyhow::Error>> {
        io::IcalFetcher::new(&self.config.calendar_path)
    }
}

/// Starts the application.
pub fn startup(argv: impl Iterator<Item = String>) -> anyhow::Result<impl app::AppConfiguration> {
    io::parse_opts(argv).map(|opts| Startup { opts })
}
