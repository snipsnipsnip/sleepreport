use anyhow::Context as _;
use getopts::{Matches, Options};
use jiff::civil::Date;
use tap::Pipe as _;

/// Represents the raw, unprocessed options parsed directly from command-line arguments.
/// This struct allows for optional values, which will be resolved into a concrete
/// `AppConfig` during the configuration phase.
#[derive(Default, Debug)]
pub struct Opts {
    pub calendar_path: Option<String>,
    pub date: Option<Date>,
    pub offset_hours: Option<i32>,
    pub num_days: Option<i32>,
}

/// Parses the command-line arguments provided by the user.
/// This function is the main entry point for handling user input at the start of the application.
/// It orchestrates the creation of the option parser, the parsing itself, and the final conversion
/// into the `Opts` struct.
pub fn parse_opts(args: impl IntoIterator<Item = String>) -> anyhow::Result<Opts> {
    let mut args = args.into_iter();
    let program = args.next().context("missing argv[0]")?;
    let getopts = make_getopts();
    getopts
        .parse(args)
        .context("parsing arguments")
        .and_then(make_opts)
        .map_err(move |e| e.context(getopts.usage(&format!("Usage: {program} [options]"))))
}

/// Defines the command-line flags and options that the application accepts.
/// This function uses the `getopts` crate to specify the expected arguments, their names,
/// and their descriptions, which are used for both parsing and generating help text.
fn make_getopts() -> Options {
    let mut getopts = Options::new();
    getopts
        .optflag("h", "help", "print this help menu")
        .optopt(
            "d",
            &kebab(stringify!(date)),
            "this tool shows a report for the previous week (by default) before the week containing this date",
            "YYYY-MM-DD",
        )
        .optopt(
            "o",
            &kebab(stringify!(offset_hours)),
            "rotate reports by this hours",
            "0-23",
        )
        .optopt(
            "c",
            &kebab(stringify!(calendar_path)),
            "loads from this ical file",
            "file path or https://... <TODO>",
        )
        .optopt(
            "n",
            &kebab(stringify!(num_days)),
            "number of days to show (defaults to -7)",
            "NUM_DAYS",
        );
    getopts
}

/// Converts the parsed `getopts::Matches` into the application's `Opts` struct.
/// This function performs validation (e.g., checking for unknown arguments or help requests)
/// and extracts the values for each option, converting them into the appropriate types.
fn make_opts(m: Matches) -> Result<Opts, anyhow::Error> {
    anyhow::ensure!(!m.opt_present("help"), "help requested");
    anyhow::ensure!(m.free.is_empty(), "unknown usage");

    Opts {
        calendar_path: m.opt_str(&kebab(stringify!(calendar_path))),
        date: m.opt_get(&kebab(stringify!(date))).context("parsing the date option")?,
        offset_hours: m
            .opt_get(&kebab(stringify!(offset_hours)))
            .context("parsing the offset number")?
            .map(|n| {
                Some(n)
                    .filter(|m| (0..24).contains(m))
                    .context("offset number out of range")
            })
            .transpose()?,
        num_days: m
            .opt_get(&kebab(stringify!(num_days)))
            .context("parsing the days option")?,
    }
    .pipe(Ok)
}

fn kebab(s: &str) -> String {
    s.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::Date;
    use std::str::FromStr;

    fn run_parse(args: &[&str]) -> Opts {
        let args = std::iter::once("program_name".into()).chain(args.iter().map(|&s| s.into()));
        parse_opts(args).unwrap()
    }

    #[test]
    fn test_parse_opts_default() {
        let opts = run_parse(&[]);
        assert_eq!(opts.calendar_path, None);
        assert_eq!(opts.date, None);
        assert_eq!(opts.offset_hours, None);
        assert_eq!(opts.num_days, None);
    }

    #[test]
    fn test_parse_opts_date() {
        let opts = run_parse(&["--date", "2024-01-01"]);
        assert_eq!(opts.date, Some(Date::from_str("2024-01-01").unwrap()));
    }

    #[test]
    fn test_parse_opts_offset_hours() {
        let opts = run_parse(&["--offset-hours", "10"]);
        assert_eq!(opts.offset_hours, Some(10));
    }

    #[test]
    fn test_parse_opts_calendar_path() {
        let opts = run_parse(&["--calendar-path", "/path/to/calendar"]);
        assert_eq!(opts.calendar_path, Some("/path/to/calendar".to_string()));
    }

    #[test]
    fn test_parse_opts_days() {
        let opts = run_parse(&["--num-days", "14"]);
        assert_eq!(opts.num_days, Some(14));
    }

    #[test]
    fn test_parse_opts_all() {
        let opts = run_parse(&[
            "--date",
            "2024-01-01",
            "--offset-hours",
            "10",
            "--calendar-path",
            "/path/to/calendar",
            "--num-days",
            "14",
        ]);
        assert_eq!(opts.date, Some(Date::from_str("2024-01-01").unwrap()));
        assert_eq!(opts.offset_hours, Some(10));
        assert_eq!(opts.calendar_path, Some("/path/to/calendar".to_string()));
        assert_eq!(opts.num_days, Some(14));
    }

    #[test]
    fn test_parse_opts_invalid_arg() {
        let args = vec!["program_name".to_string(), "--invalid-arg".to_string()];
        assert!(parse_opts(args).is_err());
    }
}
