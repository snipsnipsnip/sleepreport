mod sleep;
mod util;

use crate::sleep::SleepLoader;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    // let opts: Opts = parse_args(std::env::args());
    let buf = BufReader::new(File::open("basic.ics")?);
    let sleeps = {
        let loader = SleepLoader::new();
        loader.load(buf).collect::<anyhow::Result<Vec<_>>>()?
    };
    let table = sleep::make_table(sleeps.into_iter())?;

    println!("{}", table);

    Ok(())
}
