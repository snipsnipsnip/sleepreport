mod sleep;
mod util;

use crate::sleep::{LoadSleeps, SleepLoader};
use std::fs::File;
use std::io::BufReader;

// use std::env::Args;
// struct Opts {
//     calendar_path: Option<String>,
//     date: Option<NaiveDate>,
// }

fn main() -> anyhow::Result<()> {
    // let opts: Opts = parse_args(std::env::args());
    let buf = BufReader::new(File::open("basic.ics")?);
    let sleeps = {
        let loader = SleepLoader::new(buf);
        loader.load_sleeps().collect::<anyhow::Result<Vec<_>>>()?
    };
    let table = sleep::make_table(sleeps.into_iter())?;

    println!("{}", table);

    Ok(())
}
