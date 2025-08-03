use super::api::{CountSleeps, LoadSleeps, ReportSleeps};
use std::{fmt::Display, fs::File, io::BufReader};

// use std::env::Args;
// struct Opts {
//     calendar_path: Option<String>,
//     date: Option<NaiveDate>,
// }

pub trait AppDependency {
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
        let buf = BufReader::new(File::open("basic.ics")?);
        let c = self.deps;
        let sleeps = c
            .load_sleeps()
            .load_sleeps(buf)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into())?;
        let table = c.count_sleeps().count_sleeps(sleeps.into_iter());
        let report = c.report_sleeps().report_sleeps(table);

        println!("{}", report);

        Ok(())
    }
}
