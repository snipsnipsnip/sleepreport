use crate::{
    app::{self, AppConfiguration, AppDependency},
    io, sleep,
    util::Apply as _,
};
use anyhow::Context as _;
use chrono::{DateTime, Datelike, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use std::{fmt::Display, ops::Range};

struct Startup {
    opts: app::Opts,
}

struct Dependencies {
    config: app::AppConfig,
}

impl AppConfiguration for Startup {
    fn configure(
        self,
        conf: impl FnOnce(app::Opts) -> Option<app::AppConfig>,
    ) -> Option<impl AppDependency> {
        conf(self.opts).map(|config| Dependencies { config })
    }
}

impl AppDependency for Dependencies {
    fn load_sleeps(&self) -> impl app::LoadSleeps<Error = impl Into<anyhow::Error>> {
        io::SleepLoader::new()
    }

    fn count_sleeps(&self) -> impl app::CountSleeps {
        sleep::SleepCounter::new(self.config.show_range.clone())
    }

    fn report_sleeps(&self) -> impl app::ReportSleeps<SleepReport = impl Display> {
        sleep::SleepReporter::new(self.config.hours_offset, &Local)
    }

    fn fetch_ical(&self) -> impl app::FetchIcal<Error = impl Into<anyhow::Error>> {
        io::IcalFetcher::new(&self.config.calendar_path)
    }
}

pub fn startup(argv: impl Iterator<Item = String>) -> anyhow::Result<impl AppConfiguration> {
    io::parse_opts(argv).map(|opts| Startup { opts })
}
