use crate::app::Opts;
use crate::util::Apply as _;
use anyhow::Context as _;
use chrono::NaiveDate;
use getopts::{Matches, Options};

/// Parses command-line arguments into an `Opts` struct.
pub fn parse_opts(args: impl Iterator<Item = String>) -> anyhow::Result<Opts> {
    let mut args = args;
    let program = args.next().context("missing argv[0]")?;
    let getopts = make_getopts();
    getopts
        .parse(args)
        .context("parsing arguments")
        .and_then(make_opts)
        .map_err(move |e| e.context(getopts.usage(&format!("Usage: {program} [options]"))))
}

/// Creates the `getopts::Options` for the application.
fn make_getopts() -> Options {
    let mut getopts = Options::new();
    getopts
        .optflag("h", "help", "print this help menu")
        .optopt(
            "d",
            "date",
            "this tool shows a report for the previous week before the week containing this date",
            "YYYY-MM-DD",
        )
        .optopt("o", "hours-offset", "rotate reports by this hour", "0-23")
        .optopt(
            "c",
            "calendar-path",
            "loads from this ical file",
            "file path or https://... <TODO>",
        );
    getopts
}

/// Creates an `Opts` struct from the parsed command-line arguments.
fn make_opts(m: Matches) -> Result<Opts, anyhow::Error> {
    anyhow::ensure!(!m.opt_present("help"), "help requested");
    anyhow::ensure!(m.free.is_empty(), "unknown usage");

    Opts {
        calendar_path: m.opt_str("calendar-path"),
        date: m
            .opt_str("date")
            .map(|s| NaiveDate::parse_from_str(&s, "%F"))
            .transpose()
            .context("parsing the date option")?,
        hours_offset: m
            .opt_get::<i32>("hours-offset")
            .context("parsing the offset number")?
            .map(|n| {
                Some(n)
                    .filter(|m| (0..24).contains(m))
                    .context("offset number out of range")
            })
            .transpose()?,
    }
    .apply(Ok)
}
