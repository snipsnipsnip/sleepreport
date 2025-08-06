use crate::app::FetchIcal;
use crate::util::Apply as _;
use std::{fs::File, io::BufReader};

/// A fetcher for iCalendar files.
pub struct IcalFetcher<'a> {
    path: &'a str,
}

impl<'a> IcalFetcher<'a> {
    /// Creates a new `IcalFetcher` with the given path.
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
}

impl FetchIcal for IcalFetcher<'_> {
    type Error = std::io::Error;

    /// Fetches the iCalendar file from the configured path.
    fn fetch_ical(&self) -> Result<impl std::io::BufRead, Self::Error> {
        BufReader::new(File::open(self.path)?).apply(Ok)
    }
}
