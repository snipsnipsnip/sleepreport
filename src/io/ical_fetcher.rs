use crate::app::FetchIcal;
use std::{fs::File, io::BufReader};
use tap::Pipe as _;

/// The concrete implementation of the `FetchIcal` trait for reading from the local filesystem.
/// This struct encapsulates the logic for opening and reading a file given a path.
pub struct IcalFetcher<'a> {
    path: &'a str,
}

impl<'a> IcalFetcher<'a> {
    /// Creates a new `IcalFetcher` with the path to the iCalendar file.
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
}

impl FetchIcal for IcalFetcher<'_> {
    type Error = std::io::Error;

    /// Opens the file at the configured path and returns it as a `BufRead` trait object.
    /// This allows the rest of the application to remain agnostic about the file-based source.
    fn fetch_ical(&self) -> Result<impl std::io::BufRead, Self::Error> {
        BufReader::new(File::open(self.path)?).pipe(Ok)
    }
}
