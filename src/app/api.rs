use jiff::Timestamp;
use std::{io::BufRead, ops::Range};

pub type Hour = light_ranged_integers::RangedU8<0, 23>;

/// Defines the contract for parsing sleep data from a raw iCalendar source.
/// This trait abstracts the process of converting a stream of bytes into a structured
/// iterator of sleep time ranges.
pub trait LoadSleeps {
    type Error;

    /// Takes a `BufRead` source (like a file or network stream) and returns an
    /// iterator of `Result<Range<Timestamp>, Self::Error>`, where each `Range`
    /// represents a single sleep event. Parsing may fail which results in `Err` side.
    fn load_sleeps(&self, ical: impl BufRead) -> impl Iterator<Item = Result<Range<Timestamp>, Self::Error>>;
}

/// Defines the contract for fetching the iCalendar data.
/// This abstracts the source of the data, which could be a local file, a network request, etc.
pub trait FetchIcal {
    type Error;

    /// Returns a `BufRead` implementation that provides access to the raw iCalendar data.
    fn fetch_ical(&self) -> Result<impl BufRead, Self::Error>;
}

/// Defines the contract for transforming a stream of sleep events into a queryable data structure.
/// This is the bridge between raw data and the analysis phase.
pub trait CountSleeps {
    type Error;

    /// Consumes an iterator of sleep `Range`s and produces a `SleepQuery` object,
    /// which can be used to ask questions about the sleep patterns.
    fn count_sleeps(self, sleeps: impl Iterator<Item = Range<Timestamp>>) -> Result<impl SleepQuery, Self::Error>;
}

/// Defines the contract for a queryable sleep data structure.
/// This trait provides methods to inspect the processed sleep data, which are used
/// by the reporting module to generate the final output.
pub trait SleepQuery {
    /// Time range this data contains.
    fn query_range(&self) -> &Range<Timestamp>;

    /// Checks if there was sleep recorded at a specific hour on a specific day.
    /// Minutes and seconds of `hour` will be round.
    /// Returns `None` if the queried time is out of range.
    fn is_sleeping_at(&self, hour: Timestamp) -> Option<bool>;
}

/// Defines the contract for generating user-facing sleep report.
/// This abstracts the presentation logic.
pub trait ReportSleeps {
    type SleepReport;
    type Error;

    /// Takes a `SleepQuery` object and produces a `SleepReport`, which is typically a formatted string.
    fn report_sleeps(&self, query: impl SleepQuery) -> Result<Self::SleepReport, Self::Error>;
}
