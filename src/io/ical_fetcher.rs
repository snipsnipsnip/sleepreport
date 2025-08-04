use crate::app::FetchIcal;
use crate::util::Apply as _;
use std::{fs::File, io::BufReader};

pub struct IcalFetcher;

impl IcalFetcher {
    pub fn new() -> Self {
        Self
    }
}

impl FetchIcal for IcalFetcher {
    type Error = anyhow::Error;

    fn fetch_ical(&self) -> Result<impl std::io::BufRead, Self::Error> {
        BufReader::new(File::open("basic.ics")?).apply(Ok)
    }
}
