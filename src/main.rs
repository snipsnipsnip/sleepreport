#![forbid(unsafe_code)]
mod sleep_loader;
mod sleep_table;
mod sleep_tabulator;
mod util {
    pub(crate) mod native_date_range;
}
use crate::sleep_loader::SleepLoader;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    // let opts: Opts = parse_args(std::env::args());
    let buf = BufReader::new(File::open("basic.ics")?);
    let loader = SleepLoader::new();
    let sleeps = loader.load(buf).collect::<anyhow::Result<Vec<_>>>()?;
    let table = sleep_tabulator::make_table(sleeps.into_iter())?;

    println!("{}", table);

    Ok(())
}
