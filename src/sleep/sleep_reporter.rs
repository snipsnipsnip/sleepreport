use crate::app::SleepState::{self, Asleep, Awake, Unknown};
use crate::app::{Hour, ReportSleeps, SleepQuery};
use crate::util::{DateRangeExt as _, IterExt as _};
use jiff::{Timestamp, ToSpan as _, civil::Date, tz::TimeZone};
use std::fmt::{self, Display, Formatter};
use std::ops::Range;
use tap::Pipe as _;

/// The concrete implementation of the `ReportSleeps` trait.
/// This struct generates a tab-separated string representation of the sleep data.
#[derive(Debug)]
pub struct TsvSleepReporter {
    report_range: Range<Date>,
    offset_hours: Hour,
    timezone: TimeZone,
}

pub trait SleepReporterConfig {
    fn report_range(&self) -> Range<Date>;
    fn offset_hours(&self) -> Hour;
    fn timezone(&self) -> TimeZone;
}

impl TsvSleepReporter {
    /// Creates a new `SleepReporter`.
    /// `offset_hours` is used to rotate the 24-hour clock in the report.
    pub fn new(config: &impl SleepReporterConfig) -> Self {
        Self {
            report_range: config.report_range(),
            offset_hours: config.offset_hours(),
            timezone: config.timezone(),
        }
    }
}

// Represents a single row in the report (for a specific hour).
#[derive(Debug, PartialEq, Eq)]
pub struct SleepReportRow {
    /// The hour this row starts (e.g., 9 for 09:00)
    hour_start: u8,
    values: Vec<SleepState>,
}

impl Display for SleepReportRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hr = self.hour_start;
        write!(f, "{:02}:00～{:02}:00", hr % 24, (hr + 1) % 24)?;

        for value in &self.values {
            write!(f, "\t{}", show_state(*value))?;
        }

        Ok(())
    }
}

fn show_state(state: SleepState) -> &'static str {
    match state {
        Asleep => "睡眠",
        Awake => "",
        Unknown => "？",
    }
}

// The entire structured report.
#[derive(Debug, PartialEq, Eq)]
pub struct SleepReport {
    dates: Vec<Date>,
    rows: Vec<SleepReportRow>,
}

impl Display for SleepReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let formatted_days = self.dates.iter().map(|d| d.to_string()).into_vec().join("\t");
        write!(f, "\t{}", formatted_days)?;

        for row in &self.rows {
            write!(f, "\n{}", row)?;
        }

        Ok(())
    }
}

impl ReportSleeps for TsvSleepReporter {
    type SleepReport = SleepReport;
    type Error = jiff::Error;

    /// Generates a multi-line string report from a `SleepQuery` object.
    /// The report has a header row with dates and subsequent rows for each hour,
    /// showing "睡眠" if sleep was recorded for that hour.
    fn report_sleeps(&self, table: impl SleepQuery) -> Result<Self::SleepReport, Self::Error> {
        let days = 0..self.report_range.num_days();
        let dates = self.report_range.iter_days().into_vec();
        let start = self
            .report_range
            .start
            .to_datetime(Default::default())
            .pipe(move |d| self.timezone.to_timestamp(d))?;

        let rows = (0..24)
            .map(move |i| {
                let h: u8 = self.offset_hours.into();
                let hour_start = h + i;
                let values = days
                    .clone()
                    .map(|d| state_at(&table, start, hour_start as i32, d))
                    .into_vec();

                SleepReportRow { hour_start, values }
            })
            .into_vec();

        SleepReport { dates, rows }.pipe(Ok)
    }
}

fn state_at(table: &impl SleepQuery, start: Timestamp, hr: i32, d: i32) -> SleepState {
    let hour = start + (d * 24 + hr).hours();
    table.is_sleeping_at(hour)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{RangeExt, assert_eq};
    use jiff::civil::{Date, DateTime, Time, date, datetime, time};
    use std::collections::HashSet;

    struct MockSleepQuery {
        sleeping_hours: HashSet<Timestamp>,
        query_range: Range<Timestamp>,
    }

    impl SleepQuery for MockSleepQuery {
        fn is_sleeping_at(&self, timestamp: Timestamp) -> SleepState {
            if !self.query_range.contains(&timestamp) {
                return Unknown;
            }
            if self.sleeping_hours.contains(&timestamp) {
                Asleep
            } else {
                Awake
            }
        }

        fn query_range(&self) -> &Range<Timestamp> {
            &self.query_range
        }
    }

    struct ReportConfig {
        report_range: Range<Date>,
        offset_hours: Hour,
    }

    impl SleepReporterConfig for ReportConfig {
        fn report_range(&self) -> Range<Date> {
            self.report_range.clone()
        }

        fn offset_hours(&self) -> Hour {
            self.offset_hours
        }

        fn timezone(&self) -> TimeZone {
            TimeZone::UTC
        }
    }

    fn create_test_report(
        report_range: Range<Date>,
        offset_hours: Hour,
        sleeping_hours: HashSet<Timestamp>,
    ) -> anyhow::Result<SleepReport> {
        let Range { start, end } = to_timestamp_range(report_range.clone())?;
        let mock_sleep_query = MockSleepQuery {
            sleeping_hours,
            query_range: start..end + offset_hours.hours(),
        };

        let config = ReportConfig {
            report_range,
            offset_hours,
        };

        let reporter = TsvSleepReporter::new(&config);
        reporter.report_sleeps(mock_sleep_query).map_err(|e| e.into())
    }

    fn to_timestamp_range(report_range: Range<Date>) -> Result<Range<Timestamp>, jiff::Error> {
        report_range.try_fmap(|d| TimeZone::UTC.to_timestamp(d.to_datetime(Time::default())))
    }

    #[test]
    fn test_report_sleeps_basic() -> anyhow::Result<()> {
        let report_range = date(2024, 1, 1)..date(2024, 1, 3);
        let sleeping_hours = ["2024-01-01T22:00:00Z", "2024-01-01T23:00:00Z", "2024-01-02T00:00:00Z"]
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        let report = create_test_report(report_range.clone(), Hour::new(0), sleeping_hours)?;

        let expected_report = SleepReport {
            dates: report_range.iter_days().into_vec(),
            rows: (0..24)
                .map(|i| {
                    let hour_start = i;
                    let values = (0..report_range.num_days())
                        .map(|d| {
                            if (i == 22 || i == 23) && d == 0 {
                                Asleep
                            } else if i == 0 && d == 1 {
                                Asleep
                            } else {
                                Awake
                            }
                        })
                        .into_vec();
                    SleepReportRow { hour_start, values }
                })
                .into_vec(),
        };

        assert_eq!(expected_report, report);

        Ok(())
    }

    #[test]
    fn test_report_sleeps_empty() -> anyhow::Result<()> {
        let report_range = date(2024, 1, 1)..date(2024, 1, 2);
        let report = create_test_report(report_range.clone(), Hour::new(0), HashSet::new())?;

        let expected_report = SleepReport {
            dates: report_range.iter_days().into_vec(),
            rows: (0..24)
                .map(|i| SleepReportRow {
                    hour_start: i,
                    values: vec![Awake; report_range.num_days() as usize],
                })
                .into_vec(),
        };

        assert_eq!(expected_report, report);

        Ok(())
    }

    #[test]
    fn test_report_sleeps_with_offset() -> anyhow::Result<()> {
        let report_range = date(2024, 1, 1)..date(2024, 1, 2);
        let sleeping_hours = ["2024-01-01T02:00:00Z"]
            .map(|s| s.parse())
            .into_iter()
            .collect::<Result<_, _>>()?;

        let report = create_test_report(report_range.clone(), Hour::new(3), sleeping_hours)?;

        let dates = report_range.iter_days().into_vec();
        let rows = (0..24)
            .map(|i| {
                let hour_start = 3 + i;
                let values = (0..report_range.num_days())
                    .map(|d| if hour_start == 2 && d == 0 { Asleep } else { Awake })
                    .into_vec();
                SleepReportRow { hour_start, values }
            })
            .into_vec();

        let expected_report = SleepReport { dates, rows };
        assert_eq!(expected_report, report);
        Ok(())
    }

    #[test]
    fn test_report_sleeps_unknown() -> anyhow::Result<()> {
        let report_range = date(2024, 1, 1)..date(2024, 1, 2);
        let query_range = date(2023, 2, 1)..date(2024, 1, 1);
        let mock_sleep_query = MockSleepQuery {
            sleeping_hours: HashSet::new(),
            query_range: to_timestamp_range(query_range)?,
        };

        let config = ReportConfig {
            report_range: report_range.clone(),
            offset_hours: Hour::new(0),
        };

        let reporter = TsvSleepReporter::new(&config);
        let report = reporter.report_sleeps(mock_sleep_query)?;

        let expected_report = SleepReport {
            dates: report_range.iter_days().into_vec(),
            rows: (0..24)
                .map(|hour_start| {
                    let values = vec![Unknown];
                    SleepReportRow { hour_start, values }
                })
                .into_vec(),
        };
        assert_eq!(expected_report, report);

        Ok(())
    }

    #[test]
    fn test_sleep_report_display() -> anyhow::Result<()> {
        let report = SleepReport {
            dates: vec![date(2024, 1, 1)],
            rows: vec![
                SleepReportRow {
                    hour_start: 0,
                    values: vec![Asleep],
                },
                SleepReportRow {
                    hour_start: 1,
                    values: vec![Awake],
                },
                SleepReportRow {
                    hour_start: 2,
                    values: vec![Unknown],
                },
            ],
        };

        let expected_display = [
            "\t2024-01-01",
            "00:00～01:00\t睡眠",
            "01:00～02:00\t",
            "02:00～03:00\t？",
        ]
        .join("\n");
        assert_eq!(report.to_string(), expected_display);
        Ok(())
    }
}
