use anyhow::Context as _;
use crate::io::show_range;
use jiff::{Zoned, civil::Date};
use std::ops::Range;
use tap::Pipe as _;

/// Represents the raw, unprocessed options parsed directly from command-line arguments.
/// This struct allows for optional values, which will be resolved into a concrete
/// `AppConfig` during the configuration phase.
#[derive(Default, Debug)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<Date>,
    pub offset_hours: Option<i32>,
}

/// Holds the fully resolved and validated configuration required for the application to run.
/// This is created from `Opts` and provides guaranteed values for all settings.
#[derive(Debug)]
pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<Date>,
    pub offset_hours: i32,
}

/// The concrete configuration logic that transforms `Opts` into `AppConfig`.
/// This function contains the business logic for setting default values and calculating
/// derived configuration, such as the `show_range`.
pub fn configure(opts: Opts) -> anyhow::Result<AppConfig> {
    AppConfig {
        calendar_path: opts.calendar_path.unwrap_or("basic.ics".into()),
        show_range: show_range::calc_show_range(opts.date.unwrap_or_else(|| Zoned::now().date())),
        offset_hours: opts.offset_hours.unwrap_or(6),
    }
    .pipe(Ok)
}
