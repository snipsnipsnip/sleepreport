mod sleep_loader;
mod sleep_reporter;
mod sleep_table;
mod sleep_tabulator;
mod util {
    mod naive_date_range;
    mod range_like;
    pub use apply::Apply;
    pub use naive_date_range::*;
    pub use range_like::*;
}

use crate::sleep_loader::SleepLoader;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    // let opts: Opts = parse_args(std::env::args());
    let buf = BufReader::new(File::open("basic.ics")?);
    let sleeps = {
        let loader = SleepLoader::new();
        loader.load(buf).collect::<anyhow::Result<Vec<_>>>()?
    };
    let table = sleep_tabulator::make_table(sleeps.into_iter())?;

    println!("{}", table);

    Ok(())
}
