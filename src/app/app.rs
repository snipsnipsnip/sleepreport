use super::api::{CountSleeps, FetchIcal, LoadSleeps, ReportSleeps};
use std::fmt::Display;

// use std::env::Args;
// struct Opts {
//     calendar_path: Option<String>,
//     date: Option<NaiveDate>,
// }

pub trait AppDependency {
    fn fetch_ical(&self) -> impl FetchIcal<Error = impl Into<anyhow::Error>>;
    fn load_sleeps(&self) -> impl LoadSleeps<Error = impl Into<anyhow::Error>>;
    fn count_sleeps(&self) -> impl CountSleeps;
    fn report_sleeps(&self) -> impl ReportSleeps<SleepReport = impl Display>;
}

pub struct App<C> {
    deps: C,
}

impl<C: AppDependency> App<C> {
    pub fn new(deps: C) -> Self {
        Self { deps }
    }

    pub fn run(self) -> anyhow::Result<()> {
        // let opts: Opts = parse_args(std::env::args());
        let c = self.deps;

        let sleeps = {
            let fetcher = c.fetch_ical();
            let buf = fetcher.fetch_ical().map_err(|e| e.into())?;

            c.load_sleeps()
                .load_sleeps(buf)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.into())?
        };
        let table = c.count_sleeps().count_sleeps(sleeps.into_iter());
        let report = c.report_sleeps().report_sleeps(table);

        println!("{}", report);

        Ok(())
    }
}
