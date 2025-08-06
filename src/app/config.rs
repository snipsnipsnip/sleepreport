use crate::util::Apply as _;
use crate::{app::AppDependency, io::show_range};
use anyhow::Context as _;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, Utc};
use std::{fmt::Display, ops::Range};

/// Represents the raw, unprocessed options parsed directly from command-line arguments.
/// This struct allows for optional values, which will be resolved into a concrete
/// `AppConfig` during the configuration phase.
#[derive(Default, Debug)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<NaiveDate>,
    pub hours_offset: Option<i32>,
}

/// Holds the fully resolved and validated configuration required for the application to run.
/// This is created from `Opts` and provides guaranteed values for all settings.
#[derive(Debug)]
pub struct AppConfig {
    pub calendar_path: String,
    pub show_range: Range<DateTime<Utc>>,
    pub hours_offset: i32,
}

/// The concrete configuration logic that transforms `Opts` into `AppConfig`.
/// This function contains the business logic for setting default values and calculating
/// derived configuration, such as the `show_range`.
pub fn configure(opts: Opts) -> anyhow::Result<AppConfig> {
    let date = opts
        .date
        .and_then(|d| {
            d.and_time(NaiveTime::MIN)
                .and_local_timezone(Local)
                .single()
        })
        .unwrap_or_else(Local::now);
    AppConfig {
        calendar_path: opts.calendar_path.unwrap_or("basic.ics".into()),
        show_range: show_range::calc_show_range(date).context("parsing date range")?,
        hours_offset: opts.hours_offset.unwrap_or(6),
    }
    .apply(Ok)
}
