use crate::app::FetchIcal;
use crate::util::Apply as _;
use std::{fs::File, io::BufReader};

pub struct IcalFetcher<'a> {
    path: &'a str,
}

impl<'a> IcalFetcher<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
}

impl FetchIcal for IcalFetcher<'_> {
    type Error = std::io::Error;

    fn fetch_ical(&self) -> Result<impl std::io::BufRead, Self::Error> {
        BufReader::new(File::open(self.path)?).apply(Ok)
    }
}
